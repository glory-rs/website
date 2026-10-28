# Glory website

The official Glory learning site, built with Glory semantic widgets and Sapid. The site uses synchronous static generation and WebAssembly hydration; page UI and behavior use the semantic Widget API in Rust. The documented XML display examples compile against the same pinned main revision.

The content follows Glory main at `3c0c657` (2026-10-29): its HTML/CSS display contract, optional XML DSL, shared templates, built-in semantic widgets, custom Widget implementations, reactivity, styling, WebAssembly, SSR, hydration, and cross-platform hosts.

## Pages

- `/` — design overview, live Glory counter, host model, and learning paths
- `/docs/` — introduction, installation, first application, and reading guide
- `/docs/architecture/` — retained semantics, typed patches, Owners, and hosts
- `/docs/reactivity/` — Cage, Bond, Vary, and live examples
- `/docs/display/` — XML DSL, shared templates, components, keyed rows, and current browser-only boundaries
- `/docs/widgets/` — Glory's built-in elements, control flow, and lists
- `/docs/custom-widgets/` — implementing the Widget trait in Rust
- `/docs/styling/` — typed styles, external CSS, and bounded stylesheets
- `/docs/i18n/` — Fluent catalogs, language switching, parameters/plurals, and SSR hydration
- `/docs/web/` — WebAssembly launch and browser host lifecycle
- `/docs/ssr/` — HTML, snapshots, hydration, resources, and streaming
- `/docs/platforms/` — Web, SSR, Blitz desktop, and Android/iOS capabilities and boundaries
- `/404.html` — rendered not-found page

The current checkout uses sibling path dependencies on `../glory` and `../sapid`.

## Documentation navigation

The global menu contains Docs, API source, and the [Sapid component
library](https://sapid.taidge.org/). Docs stays current throughout `/docs/` and
`/zh/docs/`; the sidebar alone identifies the current chapter. Ancestor links
use `aria-current="location"`, while an exact page uses `aria-current="page"`.
Hover has its own appearance. Desktop and mobile share the same global links.
Long documentation pages keep the header and desktop chapter navigation visible;
the sidebar scrolls within the remaining viewport height.

Documentation uses a compact reading scale, separate from the homepage hero:
28–36px titles, 20–24px section headings, and normal body text. Keep headings
short in both languages and put explanations in the lead. The getting-started
snippet is compiled from `src/examples/getting_started.rs`; the Web guide
provides the complete browser host setup.

The API source link and installation example use the pinned Glory revision in
`deploy/dependencies.json`. Keep them aligned when adopting another revision;
the old docs.rs 0.3.1 release documents the previous architecture.

`src/navigation.rs` supplies the sidebar and the documentation overview with one
reading order: Start here, Core concepts, Building interfaces, Application
guides, and Reference. Existing chapter URLs and previous/next links remain
valid. The widgets, custom widgets, and styling guides explain where Sapid fits;
the displayed `save_action.rs` example is also compiled as part of the site.

## Languages

English uses the paths above. Simplified Chinese uses `/zh/` and the same chapter
paths, for example <https://glory.taidge.org/zh/docs/>. Both versions are generated
as complete static HTML and hydrate from the URL's language. The header language
link preserves the chapter, query string and fragment. Browser preferences never
override an explicit URL.

The optional `glory-i18n` crate supplies Fluent catalogs and Owner-owned reactive
messages. Text lives in `locales/en.ftl` and `locales/zh-CN.ftl`; page structure
and code examples are shared Rust. Add matching message IDs and variable names
in both resources. Native tests reject incomplete resources and resolve every
message, including the live counter's parameterized label.

Static generation emits 24 localized content pages, English and Chinese 404
documents, language alternates, canonical URLs, `sitemap.xml` and `robots.txt`.
Client navigation updates the same document metadata. Cloudflare Pages selects
the nearest ancestor 404; keep both `/404.html` and `/zh/404.html` in the artifact.

The header language menu lists English and 简体中文, with keyboard navigation,
Escape/outside dismissal, and the current language highlighted. The i18n guide
includes a live language/plural demo and displays the same Rust/Fluent example
files compiled by the site. Native tests exercise parameters, plural cases,
reactive switching and the SSR example; the browser gate checks hydration,
the language menu and demo in both guide routes. Example resources live in
`locales/demo.*.ftl`, and modules in `src/examples/`.

## Automatic deployment

Pushes to `main` build and publish the production site at
<https://glory.taidge.org/> through GitHub Actions and Cloudflare Pages.
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

## Display and semantic APIs

The optional `glory/dsl` feature adds `view!`, `template!`, `field!` and
`#[component]`; handwritten display templates need only `web`. Both use the same
sparse browser host. Raw HTML does not instantiate a semantic Widget per element.
Direct display currently supports browser CSR; its SSR/hydration, Blitz and
controlled-input migrations remain pending. Semantic widgets retain their
Web/SSR/native capabilities. Both layers use Cage/Bond/Vary and core ownership.
Do not import both preludes into the same namespace.

`src/examples/display_*.rs` are the actual source of the displayed guide snippets
and compile on native and wasm32. The website enables `dsl` to check these
examples; its pages continue to render/hydrate through semantic widgets and Sapid.
