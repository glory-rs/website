const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium } = require(process.env.GLORY_PLAYWRIGHT || 'D:/Works/glory-rs/glory/tests/playwright/node_modules/playwright');
const base = process.env.GLORY_WEBSITE_BASE || 'http://127.0.0.1:8796';
process.env.TEMP = process.env.TMP = 'D:/BuildCache/glory-website/tmp';
(async () => {
  const browser = await chromium.launch({ headless: true, ...(process.env.GLORY_CHROME ? { executablePath: process.env.GLORY_CHROME } : {}) });
  const report = { checks: [], errors: [] };
  try {
    const page = await browser.newPage();
    page.on('pageerror', error => report.errors.push(error.message));
    const ready = () => page.waitForFunction(() => window.gloryReady);
    const frames = () => page.evaluate(async () => { for (let i = 0; i < 12; i++) await new Promise(requestAnimationFrame); });
    const navigate = target => page.evaluate(target => {
      const link = document.createElement('a'); link.href = target; link.dataset.gloryRoute = '';
      document.querySelector('#app').append(link); link.click(); link.remove();
    }, target);
    await page.route('**/glory_website.js', async route => {
      await page.evaluate(() => {
        window.ssrCounter = document.querySelector('.counter-label');
        history.replaceState({ otherApplication: 'retained' }, '', location.href);
      });
      await route.continue();
    });
    await page.goto(base + '/'); await ready();
    assert(await page.evaluate(() => window.ssrCounter === document.querySelector('.counter-label')));
    assert.equal(await page.evaluate(() => history.state.otherApplication), 'retained');
    await page.getByRole('button', { name: 'Add one to the counter' }).click();
    await page.waitForFunction(() => document.querySelector('.counter-label').textContent.includes('1'));
    await page.evaluate(() => scrollTo(0, 400)); await frames();
    await navigate('/?source=router'); await frames();
    assert(await page.evaluate(() => window.ssrCounter === document.querySelector('.counter-label')));
    assert(await page.evaluate(() => Math.abs(scrollY - 400) < 5));
    const firstKey = await page.evaluate(() => history.state.__glory_routing_v1);
    await navigate('/index.html?source=alias'); await frames();
    assert(await page.evaluate(() => window.ssrCounter === document.querySelector('.counter-label')));
    assert(await page.locator('.counter-label').first().textContent().then(text => text.includes('1')));
    assert.notEqual(await page.evaluate(() => history.state.__glory_routing_v1), firstKey);
    await page.evaluate(() => scrollTo(0, 400)); await frames();
    await navigate('/docs/'); await page.getByRole('heading', { name: 'Getting started', exact: true }).waitFor();
    await page.goBack(); await ready(); await frames();
    assert(await page.evaluate(() => Math.abs(scrollY - 400) < 5));
    report.checks.push('SSR identity, foreign history state, query/alias page retention and entry-keyed scroll restoration');
    await navigate('/zh/not-a-page?source=router');
    await page.getByRole('heading', { name: '页面不存在', exact: true }).waitFor();
    assert.equal(await page.locator('html').getAttribute('lang'), 'zh-CN');
    assert.equal(new URL(page.url()).pathname, '/zh/not-a-page');
    report.checks.push('Unknown internal URLs keep their address and render the localized fallback');
    const late = await browser.newPage();
    late.on('pageerror', error => report.errors.push(error.message));
    await late.route('**/glory_website.js', async route => {
      await late.evaluate(() => history.replaceState({ foreign: true }, '', '/zh/docs/web/?source=late#example'));
      await route.continue();
    });
    await late.goto(base + '/');
    await late.waitForFunction(() => window.gloryReady);
    await late.getByRole('heading', { name: 'Web 应用', exact: true }).waitFor();
    assert.equal(await late.locator('html').getAttribute('lang'), 'zh-CN');
    report.checks.push('Hydrate the server route before adopting a changed pre-WASM browser URL');
    assert.deepEqual(report.errors, []);
  } finally {
    await browser.close();
    fs.writeFileSync('D:/BuildCache/glory-website/routing-browser-report.json', JSON.stringify(report, null, 2));
  }
  console.log(JSON.stringify(report, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
