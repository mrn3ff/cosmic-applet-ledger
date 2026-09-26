# Ledger — icon & logo package (Rise)

## svg/
- ledger-app-light.svg / ledger-app-dark.svg: app tile (rounded square + mark)
- ledger-mark-black.svg / ledger-mark-white.svg: mark only, transparent
- ledger-mark-currentcolor.svg: mark only, inherits CSS `color` (for the web UI)
- ledger-symbolic.svg: panel icon, full detail (24px and up)
- ledger-symbolic-16.svg: panel icon, pixel-snapped for 16px
- ledger-lockup-*.svg: mark + "Ledger" wordmark (text converted to outlines, no font needed)

## png/
- app-light/, app-dark/: 16–1024 px (16 uses the pixel-snapped glyph)
- mark-black/, mark-white/: 16–512 px, transparent (16 uses the pixel-snapped glyph)
- lockups/: @1x (64 px tall), @2x, @4x

## web/
- favicon.svg: switches light/dark automatically with the browser's color scheme
- favicon.ico: 16 / 32 / 48
- apple-touch-icon.png (180, square; iOS rounds it)
- icon-192.png, icon-512.png: for a web app manifest

## linux/
freedesktop layout: hicolor/scalable/apps and hicolor/symbolic/apps.
Rename both files to your app ID (e.g. `com.yourname.Ledger.svg` and
`com.yourname.Ledger-symbolic.svg`) so the desktop can find them.

## Colors
| Token      | Light     | Dark      |
|------------|-----------|-----------|
| Ink (mark) | #111111   | #F2F2F0   |
| Tile       | #FFFFFF   | #18181A   |
| Tile edge  | #000 @ 8% | #FFF @ 9% |

## Type
Wordmark: Geist Medium, tracking −3% (tile lockup) / −2% (mark lockup).
Geist is licensed under the SIL Open Font License.

## Geometry (100-unit grid)
Mark spans 20–80. L stem and base: 12 units thick. Columns: 10 wide, 6-unit gaps
throughout, heights 18 / 30 / 42. Tile corner radius: 22.5.
