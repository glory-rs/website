# Glory website

The official Glory learning site, built with Glory semantic widgets and Sapid. The site uses synchronous static generation and WebAssembly hydration; page UI and behavior are handwritten Rust.

The content focuses on Glory itself: its design principles, built-in semantic widgets, custom Widget implementations, reactivity, styling, WebAssembly, SSR, hydration, and cross-platform hosts.

## Pages

- `/` — design overview, live Glory counter, host model, and learning paths
- `/docs/` — introduction, installation, first application, and reading guide
- `/docs/architecture/` — retained semantics, typed patches, Owners, and hosts
- `/docs/reactivity/` — Cage, Bond, Lotus, and live examples
- `/docs/widgets/` — Glory's built-in elements, control flow, and lists
- `/docs/custom-widgets/` — implementing the Widget trait in Rust
- `/docs/styling/` — typed styles, external CSS, and bounded stylesheets
- `/docs/web/` — WebAssembly launch and browser host lifecycle
- `/docs/ssr/` — HTML, snapshots, hydration, resources, and streaming
- `/docs/platforms/` — Web, SSR, and Blitz desktop capabilities and boundaries
- `/404.html` — rendered not-found page

The current checkout uses sibling path dependencies on `../glory` and `../sapid`.

## Languages

English uses the paths above. Simplified Chinese uses `/zh/` and the same chapter
paths, for example <https://glory.taidge.com/zh/docs/>. Both versions are generated
as complete static HTML and hydrate from the URL's language. The header language
link preserves the chapter, query string and fragment. Browser preferences never
override an explicit URL.

The optional `glory-i18n` crate supplies Fluent catalogs and Owner-owned reactive
messages. Text lives in `locales/en.ftl` and `locales/zh-CN.ftl`; page structure
and code examples are shared Rust. Add matching message IDs and variable names
in both resources. Native tests reject incomplete resources and resolve every
message, including the live counter's parameterized label.

Static generation emits 20 localized content pages, English and Chinese 404
documents, language alternates, canonical URLs, `sitemap.xml` and `robots.txt`.
Client navigation updates the same document metadata. Cloudflare Pages selects
the nearest ancestor 404; keep both `/404.html` and `/zh/404.html` in the artifact.

## Automatic deployment

Pushes to `main` build and publish the production site at
<https://glory.taidge.com/> through GitHub Actions and Cloudflare Pages.
The workflow checks out the dependency revisions pinned in
`deploy/dependencies.json`, runs native tests, and generates release HTML/WASM.
See [deployment setup and troubleshooting](deploy/README.md).

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

Open <http://127.0.0.1:8796/>. Generated files are written to `D:/BuildCache/glory-website/dist`; deploy that directory to a static HTTP host. All asset URLs are root-relative. The preview script serves localized 404 pages, matching Cloudflare Pages.

To build without starting the preview:

```powershell
./scripts/build.ps1
```

## Checks

```powershell
cargo test --locked --all-targets
./scripts/check.ps1 -Browser
```

The full gate runs formatting, native tests, strict native and wasm32 Clippy, the WASM/SSR build, and real Chromium acceptance. Browser checks cover hydration, counter updates, code copy, History navigation, no-JavaScript SSR, mobile navigation, the 404 page, and horizontal overflow/header overlap at 320/390/768/1280px.

Build artifacts, Cargo target files, temporary files, and reports stay under `D:/BuildCache/glory-website`.

## Project layout

- `src/app.rs` — shared route-driven application
- `src/routes.rs` — bounded route and metadata catalogue
- `src/components.rs` — navigation, footer, documentation, and code widgets
- `src/pages/` — home, learning guides, and 404 page
- `src/web.rs` — browser History, launch, and hydration host
- `src/bin/generate.rs` — static HTML generation from Glory SSR snapshots
- `styles/site.css` — site design and responsive layout
- `tests/` — semantic/native and Chromium acceptance
