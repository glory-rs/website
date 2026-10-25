# Cloudflare Pages deployment

Pushes to `main` build and deploy through `.github/workflows/deploy.yml`.
The workflow can also be run from GitHub Actions with **Run workflow** on `main`.
The production website is <https://glory.taidge.com/>.

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
custom domain, `glory.taidge.com`.

`glory.taidge.com` is attached in **Workers & Pages → glory-website → Custom
domains**, with a `glory` CNAME in the Cloudflare-managed `taidge.com` zone
pointing to `glory-website-3fn.pages.dev`. Generated asset/navigation URLs remain
root-relative.

## Redirect the default Pages address

The account-level Bulk Redirect List `glory_pages_to_custom_domain` redirects
`glory-website-3fn.pages.dev` to `https://glory.taidge.com/` with status **301**.
Its rule is named `Glory Pages to official domain`.

The importable list is recorded in [pages-dev-redirects.csv](pages-dev-redirects.csv).
It has no header row; columns are source URL, target URL, status code, preserve
query string, include subdomains, subpath matching, and preserve path suffix.
All four boolean options are enabled, so paths, query strings, and Pages preview
subdomains redirect to the corresponding official URL. For example:

```text
https://glory-website-3fn.pages.dev/docs/web/?from=old
→ https://glory.taidge.com/docs/web/?from=old
```

Manage the list and its enabled rule in the Cloudflare account's **Bulk
Redirects** dashboard. This is account-level configuration, independent of site
deployments. The CSV is a recovery/import reference, not an asset copied into
`dist`; GitHub Actions does not apply account-level redirect rules. The existing
Pages-only deployment token does not need broader permissions.

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

Native tests must pass before the release WASM/SSR build and deployment. The
generated site is also retained as the `website-dist` Actions artifact for
seven days, including when the later deployment step fails. Build-only checks
do not require Cloudflare credentials.

For a local production build with `wasm-bindgen` 0.2.129 on PATH:

```powershell
./scripts/build.ps1 -Release
```

Alternatively, pass `-Bindgen D:/path/to/wasm-bindgen.exe`. Output is
`D:/BuildCache/glory-website/dist`. Each build removes the old output directory
before generation. Individual assets above Pages' 25 MiB limit are rejected.

All routes are prerendered to their own `index.html`, with a top-level
`404.html`; Cloudflare serves the correct missing-page response. Do not add a
catch-all SPA rewrite to `/index.html`, since hydration uses the matching route's
SSR tree. Assets have stable filenames, so `_headers` requires revalidation
instead of marking old JS/WASM as immutable.

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
