const fs = require('node:fs');
const assert = require('node:assert/strict');
const { chromium } = require(process.env.GLORY_PLAYWRIGHT || 'D:/Works/glory-rs/glory/tests/playwright/node_modules/playwright');

process.env.TEMP = process.env.TMP = 'D:/BuildCache/glory-website/tmp';
const base = process.env.GLORY_WEBSITE_BASE || 'http://127.0.0.1:8796';
const output = 'D:/BuildCache/glory-website/browser-report.json';

(async () => {
  let browser;
  const report = { base, checks: [], errors: [] };
  const check = (name) => { report.checks.push(name); console.log(`PASS ${name}`); };
  try {
    browser = await chromium.launch({ headless: true });
    const context = await browser.newContext({ permissions: ['clipboard-read', 'clipboard-write'] });
    const page = await context.newPage();
    page.on('pageerror', error => report.errors.push(error.message));
    page.on('console', message => {
      if (message.type() === 'error') report.errors.push(message.text());
    });
    await page.goto(`${base}/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.getByRole('heading', { name: 'One language. Every interface.' }).waitFor();
    check('SSR home hydrates');

    const counter = page.locator('.counter-label').first();
    await assert.equal(await counter.textContent(), 'Clicks: 0');
    await page.getByRole('button', { name: 'Add one to the counter' }).click();
    await page.waitForFunction(() => document.querySelector('.counter-label')?.textContent === 'Clicks: 1');
    await page.getByRole('button', { name: 'Reset', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('.counter-label')?.textContent === 'Clicks: 0');
    check('Counter updates and resets after hydration');

    const primary = page.getByRole('navigation', { name: 'Primary navigation' });
    await primary.getByRole('link', { name: 'Docs', exact: true }).click();
    await page.waitForURL('**/docs/');
    await page.getByRole('heading', { name: 'Learn Glory from first principles.' }).waitFor();
    await primary.getByRole('link', { name: 'Architecture', exact: true }).click();
    await page.waitForURL('**/docs/architecture/');
    await page.getByRole('heading', { level: 1 }).waitFor();
    await page.goBack();
    await page.getByRole('heading', { name: 'Learn Glory from first principles.' }).waitFor();
    check('History routing and back navigation');

    await page.goto(`${base}/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.getByRole('button', { name: 'Copy code' }).first().click();
    await page.waitForFunction(() => navigator.clipboard.readText().then(text => text.includes('scope.cage')));
    const clipboard = await page.evaluate(() => navigator.clipboard.readText());
    assert.match(clipboard, /scope\.cage/);
    check('Code copy writes the real source');

    const staticContext = await browser.newContext({ javaScriptEnabled: false });
    const staticPage = await staticContext.newPage();
    await staticPage.goto(`${base}/`);
    await staticPage.getByRole('heading', { name: 'One language. Every interface.' }).waitFor();
    await staticPage.getByText('Clicks: 0').first().waitFor();
    await staticContext.close();
    check('SSR content works without JavaScript');

    for (const width of [1280, 768, 390, 320]) {
      await page.setViewportSize({ width, height: 800 });
      for (const path of ['/', '/docs/', '/docs/architecture/', '/docs/reactivity/', '/docs/widgets/', '/docs/custom-widgets/', '/docs/styling/', '/docs/web/', '/docs/ssr/', '/docs/platforms/']) {
        await page.goto(`${base}${path}`);
        await page.waitForFunction(() => window.gloryReady === true);
        const dimensions = await page.evaluate(() => ({ viewport: innerWidth, content: document.documentElement.scrollWidth }));
        assert.ok(dimensions.content <= dimensions.viewport, `${path} overflows at ${width}px: ${dimensions.content}px`);
        if (path !== '/') {
          const spacing = await page.evaluate(() => ({
            header: document.querySelector('.site-header').getBoundingClientRect().bottom,
            content: document.querySelector('.docs-sidebar').getBoundingClientRect().top,
          }));
          assert.ok(spacing.content >= spacing.header, `${path} overlaps the header at ${width}px: ${JSON.stringify(spacing)}`);
        }
      }
    }
    check('No horizontal overflow or header overlap at 1280, 768, 390, or 320px');

    await page.setViewportSize({ width: 390, height: 800 });
    await page.goto(`${base}/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.getByRole('button', { name: 'Toggle navigation' }).click();
    const mobile = page.getByRole('navigation', { name: 'Mobile navigation' });
    await mobile.waitFor();
    await mobile.getByRole('link', { name: 'Get started', exact: true }).click();
    await page.waitForURL('**/docs/');
    await page.getByRole('heading', { name: 'Learn Glory from first principles.' }).waitFor();
    check('Mobile navigation opens and routes');

    await page.goto(`${base}/404.html`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.getByRole('heading', { name: 'This page drifted away.' }).waitFor();
    check('Rendered 404 page');
    assert.deepEqual(report.errors, [], 'Browser console and hydration errors');
    check('No browser console or hydration errors');
  } catch (error) {
    report.errors.push(error.stack || String(error));
    process.exitCode = 1;
  } finally {
    if (browser) await browser.close();
    fs.writeFileSync(output, JSON.stringify(report, null, 2));
    console.log(`Browser report: ${output}`);
    if (report.errors.length) console.error(report.errors.join('\n'));
  }
})();
