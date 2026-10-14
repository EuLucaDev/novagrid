//! Colaboração em tempo real (Nova Live) no motor: o que mudar no documento compartilhado vira
//! uma alteração para a sessão, e alterações de colegas entram sem misturar o histórico local.
//!
//! - Comandos que alteram a planilha e entram no desfazer viram [`LiveChange::Op`] com o contexto
//!   de quem editou (aba e seleção), para que o colega rode o mesmo comando no mesmo lugar.
//! - Desfazer/refazer viram [`LiveChange::Replace`] (o documento inteiro).
//! - Visão pessoal (aba ativa, zoom, célula ativa, rolagem, modo de exibição) não é conteúdo: fica
//!   fora da impressão digital e é preservada quando chega um documento novo.

use std::sync::Arc;

use novagrid_calc::Calc;
use novagrid_model::Workbook;
use serde_json::{Value as Json, json};

use crate::selection::Selection;
use crate::{EngineError, Result, Session};

/// Uma alteração local do documento compartilhado, para enviar à sessão.
#[derive(Clone, Debug, PartialEq)]
pub enum LiveChange {
    Op { cmd: String, params: Json, ctx: Json },
    Replace,
}

/// Campos de visão pessoal, que não fazem parte do conteúdo compartilhado.
const VIEW_KEYS: [&str; 5] = ["active_sheet", "zoom", "view_active", "view_top_left", "view"];

fn strip_view(v: &mut Json) {
    match v {
        Json::Object(map) => {
            for k in VIEW_KEYS {
                map.remove(k);
            }
            for x in map.values_mut() {
                strip_view(x);
            }
        }
        Json::Array(items) => items.iter_mut().for_each(strip_view),
        _ => {}
    }
}

fn err(e: impl std::fmt::Display) -> EngineError {
    EngineError::Other(format!("colaboração: {e}"))
}

// Only audited document edits may execute on a peer. Other local edits travel as data.
fn remote_command_allowed(cmd: &str) -> bool {
    matches!(cmd, "cell.set" | "home.insertSheet" | "home.bold" | "format.bold")
}

impl Session {
    /// Começa a acompanhar o documento ativo para uma sessão de colaboração.
    pub fn live_start(&mut self) -> Result<()> {
        let uid = self.doc()?.uid;
        self.live_doc = Some(uid);
        self.live_outbox.clear();
        Ok(())
    }

    /// Para de acompanhar (fim da sessão).
    pub fn live_stop(&mut self) {
        self.live_doc = None;
        self.live_outbox.clear();
    }

    /// O documento compartilhado está aberto e é o ativo?
    pub fn live_active(&self) -> bool {
        self.live_index().is_ok()
    }

    /// Alterações locais acumuladas desde a última chamada.
    pub fn live_take(&mut self) -> Vec<LiveChange> {
        std::mem::take(&mut self.live_outbox)
    }

    fn live_index(&self) -> Result<usize> {
        let uid = self.live_doc.ok_or_else(|| err("nenhuma sessão"))?;
        self.documents().iter().position(|d| d.uid == uid).ok_or_else(|| err("o documento compartilhado foi fechado"))
    }

    /// O documento compartilhado inteiro, para quem entra ou para ressincronizar.
    pub fn live_snapshot(&self) -> Result<Vec<u8>> {
        let i = self.live_index()?;
        let d = self.documents().get(i).ok_or_else(|| err("sem documento"))?;
        serde_json::to_vec(&*d.wb).map_err(err)
    }

    /// Impressão digital do conteúdo (sem a visão pessoal).
    pub fn live_digest(&self) -> Result<u64> {
        let i = self.live_index()?;
        let d = self.documents().get(i).ok_or_else(|| err("sem documento"))?;
        let mut v = serde_json::to_value(&*d.wb).map_err(err)?;
        strip_view(&mut v);
        let bytes = serde_json::to_vec(&v).map_err(err)?;
        Ok(fnv64(&bytes))
    }

    /// Carrega o documento recebido, preservando a visão pessoal (aba, zoom, célula, rolagem).
    pub fn live_load(&mut self, bytes: &[u8]) -> Result<()> {
        let i = self.live_index()?;
        let mut wb: Workbook = serde_json::from_slice(bytes).map_err(err)?;
        let d = self.docs.get_mut(i).ok_or_else(|| err("sem documento"))?;
        let old = d.wb.clone();
        if old.active_sheet < wb.sheets.len() {
            wb.active_sheet = old.active_sheet;
        }
        for sheet in wb.sheets.iter_mut() {
            if let Some(prev) = old.sheets.iter().find(|s| s.name == sheet.name) {
                let s = Arc::make_mut(sheet);
                s.zoom = prev.zoom;
                s.view_active = prev.view_active;
                s.view_top_left = prev.view_top_left;
                s.view = prev.view.clone();
            }
        }
        let mut calc = Calc::new();
        calc.recalc_all(&mut wb);
        d.calc = calc;
        d.wb = Arc::new(wb);
        d.revision += 1;
        Ok(())
    }

    /// Roda o comando de um colega no contexto (aba e seleção) em que ele editou. O histórico de
    /// desfazer local não ganha o passo dele, e a aba e a seleção locais voltam ao que eram.
    pub fn live_apply(&mut self, cmd: &str, params: &Json, ctx: &Json) -> Result<()> {
        if !remote_command_allowed(cmd) { return Err(err("comando não permitido na colaboração")); }
        let i = self.live_index()?;
        let prev_active = self.active_index();
        self.set_active(i);
        let (local_sheet, local_sel, undo_len) = {
            let d = self.doc()?;
            (d.wb.active_sheet, d.selection.clone(), d.undo.len())
        };
        let remote_sheet = ctx.get("sheet").and_then(Json::as_u64).map(|s| s as usize);
        let remote_sel: Option<Selection> = ctx.get("selection").and_then(|s| serde_json::from_value(s.clone()).ok());
        {
            let d = self.doc_mut()?;
            if let Some(s) = remote_sheet.filter(|s| *s < d.wb.sheets.len() && *s != d.wb.active_sheet) {
                Arc::make_mut(&mut d.wb).active_sheet = s;
            }
            if let Some(sel) = remote_sel {
                d.selection = sel;
            }
        }
        self.live_applying = true;
        let result = self.execute(cmd, params.clone());
        self.live_applying = false;
        if let Ok(d) = self.doc_mut() {
            d.undo.truncate(undo_len);
            if d.wb.active_sheet != local_sheet && local_sheet < d.wb.sheets.len() {
                Arc::make_mut(&mut d.wb).active_sheet = local_sheet;
            }
            d.selection = local_sel;
        }
        self.set_active(prev_active);
        result.map(|_| ())
    }

    /// Chamado por [`Session::execute`] depois de cada comando no documento compartilhado.
    pub(crate) fn live_record(&mut self, id: &str, params: &Json, undoable: bool, changed: bool, ctx: Json) {
        if self.live_applying || !changed {
            return;
        }
        if undoable && remote_command_allowed(id) {
            self.live_outbox.push(LiveChange::Op { cmd: id.to_string(), params: params.clone(), ctx });
        } else {
            self.live_outbox.push(LiveChange::Replace);
        }
    }
}

/// Contexto de quem edita: aba ativa e seleção antes do comando.
pub(crate) fn context(sheet: usize, selection: &Selection) -> Json {
    json!({ "sheet": sheet, "selection": selection })
}

/// Impressão digital estável (FNV-1a 64), igual à de `nova_live::digest`.
pub fn fnv64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    use super::*;

    #[test]
    fn remote_commands_cannot_open_export_or_execute_code() {
        let (_, mut b) = pair();
        let before = b.live_digest().unwrap();
        for command in ["file.open", "file.saveAs", "file.export", "session.open", "script.run", "app.quit", "lisp.eval"] {
            assert!(b.live_apply(command, &serde_json::json!({"path": "/tmp/not-authorized"}), &serde_json::json!({})).is_err());
            assert_eq!(before, b.live_digest().unwrap());
        }
    }

    fn pair() -> (Session, Session) {
        let mut a = Session::new();
        a.new_workbook();
        a.live_start().unwrap();
        let mut b = Session::new();
        b.new_workbook();
        b.live_start().unwrap();
        let snap = a.live_snapshot().unwrap();
        b.live_load(&snap).unwrap();
        (a, b)
    }

    /// Entrega a B tudo o que A mudou (como faria a sessão).
    fn sync(a: &mut Session, b: &mut Session) {
        for change in a.live_take() {
            match change {
                LiveChange::Op { cmd, params, ctx } => b.live_apply(&cmd, &params, &ctx).unwrap(),
                LiveChange::Replace => {
                    let snap = a.live_snapshot().unwrap();
                    b.live_load(&snap).unwrap();
                }
            }
        }
    }

    fn cell(s: &Session, sheet: usize, a1: &str) -> String {
        let d = s.doc().unwrap();
        let c = novagrid_core::CellRef::parse(a1).unwrap();
        d.wb.sheets[sheet].cell(c).map(|c| format!("{:?}", c.value)).unwrap_or_default()
    }

    #[test]
    fn edits_replicate_with_the_authors_selection_and_keep_the_peers_view() {
        let (mut a, mut b) = pair();
        a.execute("cell.set", json!({"cell": "A1", "input": "Receita"})).unwrap();
        a.execute("cell.set", json!({"cell": "A2", "input": "=1+2"})).unwrap();
        // Comando que usa a seleção de quem editou (sem `range`).
        a.execute("selection.set", json!({"range": "A1:A2"})).unwrap();
        a.execute("home.bold", json!({})).unwrap();
        // B está olhando outra célula: a edição de A não muda a seleção de B.
        b.execute("selection.set", json!({"range": "D9"})).unwrap();
        sync(&mut a, &mut b);
        assert_eq!(a.live_digest().unwrap(), b.live_digest().unwrap());
        assert_eq!(cell(&b, 0, "A1"), cell(&a, 0, "A1"));
        assert_eq!(b.doc().unwrap().selection.current().a1(), "D9");
        assert!(b.doc().unwrap().undo.is_empty(), "o desfazer de B não ganha os passos de A");

        // Uma aba nova em A; B continua na aba em que estava.
        a.execute("home.insertSheet", json!({})).unwrap();
        a.execute("cell.set", json!({"cell": "B2", "input": "na aba nova"})).unwrap();
        sync(&mut a, &mut b);
        assert_eq!(a.live_digest().unwrap(), b.live_digest().unwrap());
        assert_eq!(b.doc().unwrap().wb.sheets.len(), a.doc().unwrap().wb.sheets.len());

        // Desfazer em A chega a B como documento inteiro.
        a.execute("edit.undo", json!({})).unwrap();
        sync(&mut a, &mut b);
        assert_eq!(a.live_digest().unwrap(), b.live_digest().unwrap());

        // E B também edita de volta.
        b.execute("cell.set", json!({"cell": "C3", "input": "42"})).unwrap();
        sync(&mut b, &mut a);
        assert_eq!(a.live_digest().unwrap(), b.live_digest().unwrap());
        assert_eq!(cell(&a, 0, "C3"), cell(&b, 0, "C3"));
    }

    #[test]
    fn view_changes_are_not_shared_and_do_not_change_the_digest() {
        let (mut a, b) = pair();
        let before = a.live_digest().unwrap();
        a.execute("selection.set", json!({"range": "F20"})).unwrap();
        assert!(a.live_take().is_empty());
        assert_eq!(a.live_digest().unwrap(), before);
        assert_eq!(b.live_digest().unwrap(), before);
    }
}
