# Cloudflare Pages deployment

The unpublished 0.6.1 development cycle uses sibling `main` sources through
`development_ref` in dependencies.json. The previous frozen revisions remain in
the record; remove the development override and review new immutable revisions
before restoring a frozen deployment. Cargo.lock remains committed for this app.

Pushes to `main` build and deploy through `.github/workflows/deploy.yml`.
The workflow can also be run from GitHub Actions with **Run workflow** on `main`.
The production website is <https://glory.taidge.org/>.

## One-time account setup

In the website repository's **Settings → Secrets and variables → Actions**, add:

| Type | Name | Value |
| --- | --- | --- |
| Secret | `CLOUDFLARE_API_TOKEN` | Cloudflare API token with **Account → Cloudflare Pages → Edit**, scoped to the selected account |
| Secret | `CLOUDFLARE_ACCOUNT_ID` | The account's 32-character Account ID, not a Zone ID |
| Variable (optional) | `CLOUDFLARE_PAGES_PROJECT_NAME` | Defaults to `glory-website`; use an existing Pages project name to deploy there |

Create the token at <https://dash.cloudflare.com/profile/api-tokens>.
Never commit credentials. The workflow exposes the token only to its deployment step.

The first deployment creates a **Direct Upload** Pages project if it does not
exist, with `main` as the production branch. An existing project must use `main`
as its production branch. Every successful deployment updates the project's
custom domain, `glory.taidge.org`.

`glory.taidge.org` is attached in **Workers & Pages → glory-website → Custom
domains**, with a `glory` CNAME in the Cloudflare-managed `taidge.org` zone
pointing to `glory-website-3fn.pages.dev`. Generated asset/navigation URLs remain
root-relative.

## Redirect the previous domain and default Pages address

The account-level Bulk Redirect List `glory_pages_to_custom_domain` redirects
`glory-website-3fn.pages.dev` and the previous `glory.taidge.com` domain directly
to `https://glory.taidge.org/` with status **301**.
Its rule is named `Glory Pages to official domain`.

The importable list is recorded in [pages-dev-redirects.csv](pages-dev-redirects.csv).
It has no header row; columns are source URL, target URL, status code, preserve
query string, include subdomains, subpath matching, and preserve path suffix.
Paths and query strings are preserved for both sources. Subdomain matching is
enabled only for the Pages source so preview subdomains also redirect. For example:

```text
https://glory-website-3fn.pages.dev/docs/web/?from=old
→ https://glory.taidge.org/docs/web/?from=old
https://glory.taidge.com/zh/docs/i18n/?from=old
→ https://glory.taidge.org/zh/docs/i18n/?from=old
```

Manage the list and its enabled rule in the Cloudflare account's **Bulk
Redirects** dashboard. This is account-level configuration, independent of site
deployments. The CSV is a recovery/import reference, not an asset copied into
`dist`; GitHub Actions does not apply account-level redirect rules. The existing
Pages-only deployment token does not need broader permissions.

Keep the previous domain attached to Pages and its DNS record in the
Cloudflare-managed `taidge.com` zone so HTTPS requests reach the redirect rule.
Activate and verify `glory.taidge.org` before enabling the old-domain redirect.

Pages `_redirects` does not support domain-level redirects. Do not add a blanket
`/*` redirect there: it would also redirect requests on the official domain.

## Build and dependency policy

The Windows runner checks out `website`, `glory`, and `sapid` as sibling folders.
`dependencies.json` pins the two dependencies to full, already-pushed commit
SHAs. Update that file to adopt a new dependency revision. A push to a dependency
repository alone does not trigger a website deployment.

Rust 1.98.0, wasm-bindgen-cli 0.2.129, and Wrangler 4.152.0 are pinned. Rust
toolchains, Cargo caches/targets, npm cache, temporary files, and site output
are under `D:/BuildCache/glory-website`. The workflow verifies Cargo's actual
target directory before compiling the site.

Native tests, strict native/WASM Clippy, and bilingual Chromium acceptance must
pass before deployment. The browser gate uses the actual release WASM/SSR output
and Playwright 1.60.0, with tools and browser downloads on D drive. It covers both
languages, hydration identity, language switching, document metadata, live
controls, localized 404 responses, and four viewport widths. The
generated site is also retained as the `website-dist` Actions artifact for
30 days, including when the later deployment step fails. Build-only checks
do not require Cloudflare credentials.

For a local production build with `wasm-bindgen` 0.2.129 on PATH:

```powershell
./scripts/build.ps1 -Release
```

Alternatively, pass `-Bindgen D:/path/to/wasm-bindgen.exe`. Output is
`D:/BuildCache/glory-website/dist`. Each build removes the old output directory
before generation. Individual assets above Pages' 25 MiB limit are rejected.

English and `/zh/` routes are prerendered to their own `index.html`, with a
top-level `404.html` and `/zh/404.html`; Cloudflare serves the nearest ancestor's
missing-page document with a 404 status. Both languages are listed in
`sitemap.xml` and connected by hreflang alternates. Do not add a
catch-all SPA rewrite to `/index.html`, since hydration uses the matching route's
SSR tree. CSS, bindgen JS/WASM and snippets share an `assets-<content-hash>/` directory.
Images/icons use a separate `media-<content-hash>/` directory computed before
compilation, so browser and SSR builds embed the same URLs. Relative bindgen WASM
and snippet imports stay inside their asset directory. A content change changes
the URL; unchanged content keeps the same URL. No timestamp query is necessary.

`_headers` gives both immutable directories `public, max-age=31536000, immutable`.
Entry HTML and other unversioned files use Pages' default
`public, max-age=0, must-revalidate` (ETag/304); `asset-manifest.json` uses `no-cache`.
Do not add a global Cache-Control header: Pages merges matching rules, which would
produce conflicting directives on immutable assets.

Before each upload, `deploy.ps1` retrieves the previous production manifest and
copies the last three distinct asset releases into the new deployment. Each file
is verified against SHA-256 and size before upload. Limits are 25 MiB per file,
500 files per release and 200 MiB overall. Failed downloads/integrity checks stop
deployment. The first migration discovers the old loader/WASM/snippets/styles and
media from the existing site and carries them forward as a legacy release.
This retention works across fresh runners and does not depend on the expiry of
Actions artifacts or Cloudflare's opportunistic old-asset cache. Retention is
bounded by three previous releases; tabs older than that should reload.

The `taidge.org` Cache Rule **Glory and Sapid origin browser caching** matches only
`glory.taidge.org` and `sapid.taidge.org`, and respects origin Browser TTL. Keep
HTML outside any Cache Everything/Edge TTL override. The zone's default 4-hour
Browser TTL otherwise overrides short origin TTLs. Existing open tabs continue
running their loaded version until the reader reloads or navigates to a new document.

## Troubleshooting

- Missing secrets: add both secrets above, then rerun the failed workflow.
- Project lookup/creation fails: check the Account ID, token account scope, and
  Cloudflare Pages Edit permission.
- Dependency checkout fails: check that the pinned commit is pushed and public.
- Build failures: inspect the build/test step; the production site is updated
  only after those steps succeed.

References: [Cloudflare CI upload](https://developers.cloudflare.com/pages/how-to/use-direct-upload-with-continuous-integration/),
[custom domains](https://developers.cloudflare.com/pages/configuration/custom-domains/),
[Pages-to-custom-domain redirects](https://developers.cloudflare.com/pages/how-to/redirect-to-custom-domain/).
