# Glory website

The official Glory website, built with Glory semantic widgets and Sapid. The site uses synchronous static generation and WebAssembly hydration; page UI and behavior are handwritten Rust.

## Pages

- `/` — framework overview, live counter, SSR/hydration, and Sapid examples
- `/docs/` — installation and first application
- `/docs/reactivity/` — Cage, Bond, Lotus, and a live example
- `/sapid/` — widget library overview and interactive controls
- `/404.html` — rendered not-found page

The current checkout uses sibling path dependencies on `../glory` and `../sapid`.

## Requirements

- Rust 1.94 or newer
- `wasm32-unknown-unknown`
- `wasm-bindgen-cli` 0.2.129
- PowerShell and Python 3 for the build/preview scripts
- Playwright for optional browser acceptance (the script reuses `../glory/tests/playwright`)

## Build and preview

```powershell
./scripts/build.ps1 -Serve
```

Open <http://127.0.0.1:8796/>. Generated files are written to `D:/BuildCache/glory-website/dist`; deploy that directory to a static HTTP host. All asset URLs are root-relative. Configure the host to serve `404.html` for unknown paths.

To build without starting the preview:

```powershell
./scripts/build.ps1
```

## Checks

```powershell
cargo test --locked --all-targets
./scripts/check.ps1 -Browser
```

The full gate runs formatting, native tests, strict native and wasm32 Clippy, the WASM/SSR build, and real Chromium acceptance. Browser checks cover hydration, counter updates, code copy, History navigation, no-JavaScript SSR, mobile navigation, the 404 page, and horizontal overflow at 320/390/768/1280px.

Build artifacts, Cargo target files, temporary files, and reports stay under `D:/BuildCache/glory-website`.

## Project layout

- `src/app.rs` — shared route-driven application
- `src/routes.rs` — bounded route and metadata catalogue
- `src/components.rs` — navigation, footer, documentation, and code widgets
- `src/pages/` — home, guide, reactivity, Sapid, and 404 pages
- `src/web.rs` — browser History, launch, and hydration host
- `src/bin/generate.rs` — static HTML generation from Glory SSR snapshots
- `styles/site.css` — site design and responsive layout
- `tests/` — semantic/native and Chromium acceptance
