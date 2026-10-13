# Asset attribution

Every non-code asset in this repository (images, icons, fonts, example files, presets) is listed here with its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if an asset file is missing from this table.

**Policy (mandatory):** Nova Grid contains **no Microsoft, Adobe, Avid or Autodesk iconography, images, artwork, fonts, templates, themes, table styles or sample files**. Every asset is original work by Nova Grid contributors or third-party material under an open licence (OSI open source, public domain / CC0, or Creative Commons that allows redistribution). Screenshots of Microsoft software are never committed. Font files are not added here: shared fonts live in [EuLucaDev/novafoods-fonts](https://github.com/EuLucaDev/novafoods-fonts), an optional build input (`NOVAFOODS_FONTS_DIR`). The one exception is the Nova Foods brand files in `docs/brand/`: Nova Foods trademarks, not open source, used under `docs/brand/LICENSE-brand.txt`.

Generated-in-code art is original and has no file to list: the UI icon set (`crates/ui-egui/src/icons.rs`), themes (`crates/engine/src/cmd/view.rs`), cell styles (`crates/engine/src/cmd/format.rs`), table styles (`crates/engine/src/tables.rs`), chart palettes and the sample workbooks (`crates/engine/src/sample.rs`).

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `docs/brand/novafoods-logo.svg` | Nova Foods Team | craftrules `assets/brand/` | Nova Foods brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/novafoods-logo.png` | Nova Foods Team | craftrules `assets/brand/` | Nova Foods brand terms | Trademark |
| `docs/brand/novafoods-logo-white.svg` | Nova Foods Team | craftrules `assets/brand/` | Nova Foods brand terms | Trademark |
| `docs/brand/novafoods-logo-white.png` | Nova Foods Team | craftrules `assets/brand/` | Nova Foods brand terms | Trademark |
| `docs/brand/novafoods-mark.svg` | Nova Foods Team | craftrules `assets/brand/` | Nova Foods brand terms | Trademark |
| `docs/brand/novafoods-mark.png` | Nova Foods Team | craftrules `assets/brand/` | Nova Foods brand terms | Trademark |
| `docs/brand/novafoods-mark-black.svg` | Nova Foods Team | craftrules `assets/brand/` | Nova Foods brand terms | Trademark |
| `docs/brand/novafoods-mark-black.png` | Nova Foods Team | craftrules `assets/brand/` | Nova Foods brand terms | Trademark |
| `docs/brand/LICENSE-brand.txt` | (licence text) | craftrules | — | |
| `assets/app-icon/novagrid.svg` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | PLACEHOLDER app icon (ledger page); replaced by the owner's creature art later |
| `packaging/macos/dmg/background.svg` | @XusBadia | original (hand-written SVG, derived from the Nova Grid app icon `assets/app-icon/novagrid.svg`, referenced via `<image>`, not copied; text is outlined glyph paths; `packaging/macos/dmg/generate.py` renders it to `background.tiff`) | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | Nova Grid macOS DMG window background for the Finder window; added 2026-10-08 |
| `packaging/macos/dmg/background.tiff` | @XusBadia | Rendered from `packaging/macos/dmg/background.svg` by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | 1x + 2x HiDPI TIFF for the macOS DMG window; includes the app icon (see its row) and LittleCMS's built-in sRGB profile ("No copyright, use freely") |
| `assets/app-icon/novagrid-1024.png` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, rendered from novagrid.svg |
| `assets/app-icon/novagrid-macos-512.png` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, macOS grid render |
| `assets/app-icon/novagrid.icns` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, macOS bundle icon |
| `assets/app-icon/novagrid.ico` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Windows exe icon |
| `assets/app-icon/hicolor/16x16/apps/br.com.novafoods.novagrid.png` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/24x24/apps/br.com.novafoods.novagrid.png` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/32x32/apps/br.com.novafoods.novagrid.png` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/48x48/apps/br.com.novafoods.novagrid.png` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novagrid.png` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/128x128/apps/br.com.novafoods.novagrid.png` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/256x256/apps/br.com.novafoods.novagrid.png` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/512x512/apps/br.com.novafoods.novagrid.png` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, Linux hicolor render |
| `assets/app-icon/hicolor/scalable/apps/br.com.novafoods.novagrid.svg` | Nova Grid contributors | original (hand-written SVG; `packaging/icons.sh`) | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Placeholder icon, copy of novagrid.svg |
| `assets/app-icon/README.md` | Nova Grid contributors | original | MIT OR Apache-2.0 | Icon documentation |
| `assets/app-icon/LICENSE.txt` | (licence text) | original | — | Licence of the app icon files |
| `docs/images/hero-sales.png` | Nova Grid contributors | screenshot of Nova Grid rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/budget.png` | Nova Grid contributors | screenshot of Nova Grid rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/grades-formulas.png` | Nova Grid contributors | screenshot of Nova Grid rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/dark-insert.png` | Nova Grid contributors | screenshot of Nova Grid rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/pivot.png` | Nova Grid contributors | screenshot of Nova Grid rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/format-chart.png` | Nova Grid contributors | screenshot of Nova Grid rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
| `docs/images/page-break.png` | Nova Grid contributors | screenshot of Nova Grid rendered offscreen (`crates/ui-egui/examples/snapshot.rs`) showing an original sample workbook | MIT OR Apache-2.0 | README screenshot; no third-party content |
