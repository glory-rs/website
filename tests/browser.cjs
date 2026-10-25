const fs = require('node:fs');
const assert = require('node:assert/strict');
const { chromium } = require(process.env.GLORY_PLAYWRIGHT || 'D:/Works/glory-rs/glory/tests/playwright/node_modules/playwright');

process.env.TEMP = process.env.TMP = 'D:/BuildCache/glory-website/tmp';
const base = process.env.GLORY_WEBSITE_BASE || 'http://127.0.0.1:8796';
const output = process.env.GLORY_WEBSITE_REPORT || 'D:/BuildCache/glory-website/browser-report.json';
fs.mkdirSync(process.env.TEMP, { recursive: true });
const plain = text => text.replace(/[\u2068\u2069]/g, '');

(async () => {
  let browser;
  const report = { base, checks: [], errors: [] };
  const check = (name) => { report.checks.push(name); console.log(`PASS ${name}`); };
  try {
    browser = await chromium.launch({ headless: true });
    const context = await browser.newContext({ permissions: ['clipboard-read', 'clipboard-write'] });
    const page = await context.newPage();
    const chooseLanguage = async (name) => {
      await page.locator('.language-picker').getByRole('button').click();
      await page.getByRole('menuitem', { name, exact: true }).click();
    };
    page.on('pageerror', error => report.errors.push(error.message));
    page.on('console', message => {
      if (message.type() === 'error' && !message.text().includes('status of 404')) report.errors.push(message.text());
    });
    await page.goto(`${base}/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.getByRole('heading', { name: 'One language. Every interface.' }).waitFor();
    check('SSR home hydrates');

    const counter = page.locator('.counter-label').first();
    await assert.equal(plain(await counter.textContent()), 'Clicks: 0');
    await page.getByRole('button', { name: 'Add one to the counter' }).click();
    await page.waitForFunction(() => document.querySelector('.counter-label')?.textContent.replace(/[\u2068\u2069]/g, '') === 'Clicks: 1');
    await page.getByRole('button', { name: 'Reset', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('.counter-label')?.textContent.replace(/[\u2068\u2069]/g, '') === 'Clicks: 0');
    check('Counter updates and resets after hydration');

    const primary = page.getByRole('navigation', { name: 'Primary navigation' });
    await primary.getByRole('link', { name: 'Docs', exact: true }).click();
    await page.waitForURL('**/docs/');
    await page.getByRole('heading', { name: 'Learn Glory from first principles.' }).waitFor();
    await page.locator('.docs-sidebar').getByRole('link', { name: 'Architecture', exact: true }).click();
    await page.waitForURL('**/docs/architecture/');
    await page.getByRole('heading', { level: 1 }).waitFor();
    await page.goBack();
    await page.getByRole('heading', { name: 'Learn Glory from first principles.' }).waitFor();
    check('History routing and back navigation');

    await page.setViewportSize({ width: 1440, height: 1000 });
    for (const prefix of ['', '/zh']) {
      await page.goto(`${base}${prefix}/docs/widgets/`);
      await page.waitForFunction(() => window.gloryReady === true);
      await page.evaluate(() => { window.navigationDocument = true; });
      const globalNav = page.locator('.desktop-nav');
      assert.equal(await globalNav.getByRole('link').count(), 3);
      assert.equal(await globalNav.getByRole('link', { name: 'Sapid ↗', exact: true }).getAttribute('href'), 'https://sapid.taidge.org/');
      assert.equal(await globalNav.locator('a[href="https://github.com/glory-rs/glory/tree/f710ea096b8cab932e876093f34e90e259197141/crates"]').count(), 1);
      const chapters = await page.locator(`.docs-sidebar a[href^="${prefix}/docs/"]`).evaluateAll(links => links.map(link => link.getAttribute('href')));
      assert.equal(chapters.length, 10);
      for (const href of chapters) {
        await page.locator(`.docs-sidebar a[href="${href}"]`).click();
        await page.waitForURL(`${base}${href}`);
        await page.waitForFunction(href => document.querySelector('.docs-sidebar [aria-current="page"]')?.getAttribute('href') === href, href);
        assert.equal(await page.evaluate(() => window.navigationDocument), true, 'chapter changes use the same document');
        assert.equal(await globalNav.locator('.is-active').count(), 1);
        const globalCurrent = globalNav.locator('.is-active');
        assert.equal(await globalCurrent.getAttribute('href'), `${prefix}/docs/`);
        assert.equal(await globalCurrent.getAttribute('aria-current'), href === `${prefix}/docs/` ? 'page' : 'location');
        assert.equal(await page.locator('.docs-sidebar .is-active').count(), 1);
        assert.equal(await page.locator('.docs-sidebar [aria-current="page"]').getAttribute('href'), href);
      }
      await page.locator(`.docs-sidebar a[href="${prefix}/docs/styling/"]`).click();
      await page.waitForURL(`${base}${prefix}/docs/styling/`);
      await page.waitForFunction(href => document.querySelector('.docs-sidebar [aria-current="page"]')?.getAttribute('href') === href, `${prefix}/docs/styling/`);
      await page.locator(`.docs-sidebar a[href="${prefix}/docs/"]`).hover();
      assert.notEqual(
        await page.locator(`.docs-sidebar a[href="${prefix}/docs/"]`).evaluate(e => getComputedStyle(e).backgroundColor),
        await page.locator('.docs-sidebar .is-active').evaluate(e => getComputedStyle(e).backgroundColor),
        'hover and current chapter have distinct backgrounds',
      );
      await page.locator(`.docs-sidebar a[href="${prefix}/docs/ssr/"]`).click();
      await page.waitForURL(`${base}${prefix}/docs/ssr/`);
      await page.goBack();
      await page.waitForURL(`${base}${prefix}/docs/styling/`);
      await page.waitForFunction(href => document.querySelector('.docs-sidebar [aria-current="page"]')?.getAttribute('href') === href, `${prefix}/docs/styling/`);
      assert.equal(await page.locator('.docs-sidebar [aria-current="page"]').getAttribute('href'), `${prefix}/docs/styling/`);
      assert.equal(await globalNav.locator('.is-active').getAttribute('href'), `${prefix}/docs/`);
    }
    check('Both languages: Docs stays current across all sidebar chapters and Back; hover differs from selection; API and Sapid stay external');

    await page.goto(`${base}/docs/ssr/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.evaluate(() => window.scrollTo(0, 1000));
    await page.waitForFunction(() => scrollY >= 900);
    const pinnedNavigation = await page.evaluate(() => ({
      header: document.querySelector('.site-header').getBoundingClientRect().toJSON(),
      sidebar: document.querySelector('.docs-sidebar').getBoundingClientRect().toJSON(),
      height: innerHeight,
    }));
    assert.equal(pinnedNavigation.header.top, 0);
    assert.ok(pinnedNavigation.sidebar.top >= pinnedNavigation.header.bottom && pinnedNavigation.sidebar.top <= pinnedNavigation.header.bottom + 8, JSON.stringify(pinnedNavigation));
    assert.ok(pinnedNavigation.sidebar.bottom <= pinnedNavigation.height, JSON.stringify(pinnedNavigation));
    check('Long documentation pages retain the global header and a scrollable sidebar below it');

    for (const prefix of ['', '/zh']) {
      await page.setViewportSize({ width: 390, height: 800 });
      await page.goto(`${base}${prefix}/docs/widgets/`);
      await page.waitForFunction(() => window.gloryReady === true);
      await page.locator('.menu-button').click();
      const mobileSection = page.locator('.mobile-nav');
      await mobileSection.waitFor();
      assert.equal(await mobileSection.locator('.is-active').count(), 1);
      assert.equal(await mobileSection.locator('[aria-current="location"]').getAttribute('href'), `${prefix}/docs/`);
      assert.equal(await mobileSection.getByRole('link', { name: 'Sapid ↗', exact: true }).getAttribute('href'), 'https://sapid.taidge.org/');
      await page.screenshot({ path: `D:/BuildCache/glory-website/navigation-mobile-${prefix ? 'zh' : 'en'}.png` });
      await mobileSection.locator(`a[href="${prefix}/docs/"]`).click();
      await page.waitForURL(`${base}${prefix}/docs/`);
      await page.locator('.mobile-nav').waitFor({ state: 'hidden' });
    }
    check('Both mobile menus preserve the Docs section, include Sapid and close after changing chapter');

    await page.setViewportSize({ width: 1280, height: 900 });

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
    await staticPage.locator('.counter-label').filter({ hasText: /Clicks:/ }).first().waitFor();
    for (const [prefix, heading] of [['', 'Build applications with Sapid'], ['/zh', '使用 Sapid 构建应用界面']]) {
      await staticPage.goto(`${base}${prefix}/docs/widgets/`);
      await staticPage.getByRole('heading', { name: heading, exact: true }).waitFor();
      assert.equal(await staticPage.locator('.desktop-nav [aria-current="location"]').getAttribute('href'), `${prefix}/docs/`);
      assert.equal(await staticPage.locator('.docs-sidebar [aria-current="page"]').getAttribute('href'), `${prefix}/docs/widgets/`);
      assert.equal(await staticPage.locator('.docs-article a[href="https://sapid.taidge.org/"]').count(), 1);
    }
    await staticContext.close();
    check('No-JS SSR includes both levels of current navigation and bilingual Sapid documentation');

    for (const width of [1280, 768, 390, 320]) {
      await page.setViewportSize({ width, height: 800 });
      for (const prefix of ['', '/zh']) {
        for (const chapter of ['/', '/docs/', '/docs/architecture/', '/docs/reactivity/', '/docs/widgets/', '/docs/custom-widgets/', '/docs/styling/', '/docs/i18n/', '/docs/web/', '/docs/ssr/', '/docs/platforms/']) {
          const path = prefix + chapter;
          await page.goto(`${base}${path}`);
          await page.waitForFunction(() => window.gloryReady === true);
          const dimensions = await page.evaluate(() => ({ viewport: innerWidth, content: document.documentElement.scrollWidth }));
          assert.ok(dimensions.content <= dimensions.viewport, `${path} overflows at ${width}px: ${dimensions.content}px`);
          if (chapter !== '/') {
            const spacing = await page.evaluate(() => ({
              header: document.querySelector('.site-header').getBoundingClientRect().bottom,
              content: document.querySelector('.docs-sidebar').getBoundingClientRect().top,
            }));
            assert.ok(spacing.content >= spacing.header, `${path} overlaps the header at ${width}px: ${JSON.stringify(spacing)}`);
          }
        }
      }
    }
    check('Both languages: no horizontal overflow or header overlap at 1280, 768, 390, or 320px');

    await page.setViewportSize({ width: 390, height: 800 });
    await page.goto(`${base}/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.getByRole('button', { name: 'Toggle navigation' }).click();
    const mobile = page.getByRole('navigation', { name: 'Mobile navigation' });
    await mobile.waitFor();
    assert.equal(await mobile.getByRole('link', { name: 'Sapid ↗', exact: true }).getAttribute('href'), 'https://sapid.taidge.org/');
    await mobile.getByRole('link', { name: 'Docs', exact: true }).click();
    await page.waitForURL('**/docs/');
    await page.getByRole('heading', { name: 'Learn Glory from first principles.' }).waitFor();
    check('Mobile navigation opens and routes');

    await page.goto(`${base}/404.html`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.getByRole('heading', { name: 'This page drifted away.' }).waitFor();
    check('Rendered 404 page');

    await page.setViewportSize({ width: 1280, height: 900 });
    await page.goto(`${base}/docs/web/?source=language#example`);
    await page.waitForFunction(() => window.gloryReady === true);
    // A language change must keep this document and replace the keyed page.
    const identity = await page.evaluate(() => {
      const el = document.querySelector('.docs-title'); window.testHeading = el; return !!el;
    });
    assert.ok(identity);
    await chooseLanguage('简体中文');
    await page.waitForURL('**/zh/docs/web/?source=language#example');
    await page.getByRole('heading', { name: 'Web 应用。', exact: true }).waitFor();
    assert.equal(await page.locator('html').getAttribute('lang'), 'zh-CN');
    assert.equal(await page.title(), 'Web 应用 — Glory');
    assert.equal(await page.locator('link[rel=canonical]').getAttribute('href'), 'https://glory.taidge.org/zh/docs/web/');
    assert.equal(await page.locator('meta[property="og:url"]').getAttribute('content'), 'https://glory.taidge.org/zh/docs/web/');
    assert.equal(await page.locator('link[hreflang="en"]').getAttribute('href'), 'https://glory.taidge.org/docs/web/');
    assert.match(await page.locator('meta[name=description]').getAttribute('content'), /浏览器/);
    // Soft navigation replaces the language-keyed page, not the document.
    assert.equal(await page.evaluate(() => !!window.testHeading && !window.testHeading.isConnected), true);
    await chooseLanguage('English');
    await page.waitForURL('**/docs/web/?source=language#example');
    await page.getByRole('heading', { name: 'Web applications.', exact: true }).waitFor();
    await page.goBack();
    await page.getByRole('heading', { name: 'Web 应用。', exact: true }).waitFor();
    assert.equal(await page.locator('html').getAttribute('lang'), 'zh-CN');
    check('Language switching and Back preserve chapter, query, fragment and metadata');

    const adoptContext = await browser.newContext({ locale: 'en-US' });
    await adoptContext.addInitScript(() => {
      const observer = new MutationObserver(() => {
        const title = document.querySelector('.docs-title');
        if (title && !window.initialTitle) { window.initialTitle = title; observer.disconnect(); }
      });
      observer.observe(document, { childList: true, subtree: true });
    });
    const adoptPage = await adoptContext.newPage();
    await adoptPage.goto(`${base}/zh/docs/architecture/`);
    await adoptPage.waitForFunction(() => window.gloryReady === true);
    assert.equal(await adoptPage.evaluate(() => window.initialTitle === document.querySelector('.docs-title')), true);
    assert.equal(await adoptPage.locator('html').getAttribute('lang'), 'zh-CN');
    await adoptContext.close();
    check('Chinese deep-link hydration adopts SSR DOM despite English browser preference');

    await page.goto(`${base}/zh/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.getByRole('heading', { name: '一种语言。 每一种界面。' }).waitFor();
    await page.getByRole('button', { name: '计数器加一', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('.counter-label')?.textContent.replace(/[\u2068\u2069]/g, '') === '点击次数：1');
    await page.getByRole('button', { name: '重置', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('.counter-label')?.textContent.replace(/[\u2068\u2069]/g, '') === '点击次数：0');
    await page.getByRole('button', { name: '复制代码', exact: true }).first().click();
    await page.waitForFunction(() => navigator.clipboard.readText().then(text => text.includes('scope.cage')));
    await page.setViewportSize({ width: 390, height: 800 });
    await page.getByRole('button', { name: '切换导航', exact: true }).click();
    await page.getByRole('navigation', { name: '移动端导航' }).getByRole('link', { name: '文档', exact: true }).click();
    await page.waitForURL('**/zh/docs/');
    await page.getByRole('heading', { name: '从基本原理学习 Glory。' }).waitFor();
    check('Chinese counter, copy and mobile navigation work after hydration');

    const chineseStatic = await browser.newContext({ javaScriptEnabled: false });
    const staticZh = await chineseStatic.newPage();
    await staticZh.goto(`${base}/zh/docs/ssr/`);
    await staticZh.getByRole('heading', { name: 'SSR 与水合。', exact: true }).waitFor();
    assert.equal(await staticZh.locator('html').getAttribute('lang'), 'zh-CN');
    await chineseStatic.close();
    for (const [path, heading, lang] of [['/missing/deep', 'This page drifted away.', 'en'], ['/zh/missing/deep', '这个页面走丢了。', 'zh-CN']]) {
      const response = await page.goto(`${base}${path}`);
      assert.equal(response.status(), 404);
      await page.waitForFunction(() => window.gloryReady === true);
      await page.getByRole('heading', { name: heading, exact: true }).waitFor();
      assert.equal(await page.locator('html').getAttribute('lang'), lang);
      assert.equal(await page.locator('meta[name=robots]').getAttribute('content'), 'noindex');
    }
    const xml = await context.request.get(`${base}/sitemap.xml`);
    assert.equal(xml.status(), 200);
    assert.equal((await xml.text()).match(/<url>/g).length, 22);
    check('Chinese no-JS SSR, localized real 404 responses and bilingual sitemap');

    await page.setViewportSize({ width: 1280, height: 900 });
    await page.goto(`${base}/docs/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.getByRole('navigation', { name: 'Documentation', exact: true })
      .getByRole('link', { name: 'Internationalization', exact: true }).click();
    await page.waitForURL('**/docs/i18n/');
    await page.getByRole('heading', { name: 'Internationalization with glory-i18n.', exact: true }).waitFor();
    assert.equal(await page.getByRole('navigation', { name: 'Documentation', exact: true }).locator('[aria-current="page"]').count(), 1);
    const demo = page.locator('.i18n-demo');
    assert.equal(plain(await demo.locator('.i18n-demo-message').textContent()), 'You have one message.');
    await demo.getByRole('button', { name: 'Add a message', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('.i18n-demo-message').textContent.replace(/[\u2068\u2069]/g, '') === 'You have 2 messages.');
    await demo.getByRole('button', { name: '简体中文', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('.i18n-demo-message').textContent.replace(/[\u2068\u2069]/g, '') === '你有 2 条消息。');
    await demo.getByRole('button', { name: '增加一条消息', exact: true }).click();
    await demo.getByRole('button', { name: 'English', exact: true }).click();
    await page.waitForFunction(() => document.querySelector('.i18n-demo-message').textContent.replace(/[\u2068\u2069]/g, '') === 'You have 3 messages.');
    check('Guide examples hydrate, select English plurals and retain count during reactive language changes');

    await page.goto(`${base}/docs/i18n/?source=guide#demo`);
    await page.waitForFunction(() => window.gloryReady === true);
    const picker = page.locator('.language-picker').getByRole('button', { name: 'English', exact: true });
    await picker.focus();
    await picker.press('Enter');
    await page.getByRole('menu', { name: 'English', exact: true }).waitFor();
    assert.equal(await picker.getAttribute('aria-expanded'), 'true');
    assert.equal(await page.getByRole('menuitem').count(), 2);
    await page.getByRole('menuitem', { name: 'English', exact: true }).press('ArrowDown');
    await page.waitForFunction(() => document.activeElement?.textContent === '简体中文');
    await page.getByRole('menuitem', { name: '简体中文', exact: true }).press('Enter');
    await page.waitForURL('**/zh/docs/i18n/?source=guide#demo');
    await page.getByRole('heading', { name: '使用 glory-i18n 实现国际化。', exact: true }).waitFor();
    assert.equal(await page.locator('html').getAttribute('lang'), 'zh-CN');
    assert.equal(await page.locator('link[rel=canonical]').getAttribute('href'), 'https://glory.taidge.org/zh/docs/i18n/');
    const zhPicker = page.locator('.language-picker').getByRole('button', { name: '简体中文', exact: true });
    await zhPicker.click();
    await page.getByRole('menuitem', { name: 'English', exact: true }).press('Escape');
    await page.getByRole('menu', { name: '简体中文', exact: true }).waitFor({ state: 'hidden' });
    assert.equal(await zhPicker.getAttribute('aria-expanded'), 'false');
    await zhPicker.click();
    await page.locator('.docs-title').click();
    await page.getByRole('menu', { name: '简体中文', exact: true }).waitFor({ state: 'hidden' });
    check('Language dropdown shows two supported languages and handles keyboard, Escape and outside dismissal');

    await page.setViewportSize({ width: 320, height: 800 });
    await zhPicker.click();
    const menuBounds = await page.getByRole('menu', { name: '简体中文', exact: true }).boundingBox();
    assert.ok(menuBounds.x >= 0 && menuBounds.x + menuBounds.width <= 320, JSON.stringify(menuBounds));
    await page.getByRole('menuitem', { name: '简体中文', exact: true }).click();
    await page.getByRole('menu', { name: '简体中文', exact: true }).waitFor({ state: 'hidden' });
    assert.ok(page.url().endsWith('/zh/docs/i18n/?source=guide#demo'));
    const staticGuide = await browser.newContext({ javaScriptEnabled: false });
    for (const [prefix, name] of [['', 'Internationalization with glory-i18n.'], ['/zh', '使用 glory-i18n 实现国际化。']]) {
      const staticGuidePage = await staticGuide.newPage();
      await staticGuidePage.goto(`${base}${prefix}/docs/i18n/`);
      await staticGuidePage.getByRole('heading', { name, exact: true }).waitFor();
      assert.match(await staticGuidePage.locator('.docs-article').textContent(), /hydrate_with/);
    }
    await staticGuide.close();
    check('Both guide routes render without JavaScript; dropdown fits 320px and current selection keeps its URL');

    await page.setViewportSize({ width: 1440, height: 1000 });
    await page.goto(`${base}/zh/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.screenshot({ path: 'D:/BuildCache/glory-website/zh-home.png', fullPage: false });
    await page.goto(`${base}/zh/docs/i18n/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await zhPicker.click();
    await page.getByRole('menu', { name: '简体中文', exact: true }).waitFor();
    await page.screenshot({ path: 'D:/BuildCache/glory-website/zh-i18n-guide.png', fullPage: false });

    await page.goto(`${base}/docs/widgets/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.locator('.docs-sidebar a[href="/docs/"]').hover();
    await page.screenshot({ path: 'D:/BuildCache/glory-website/navigation-widgets.png', fullPage: false });
    await page.goto(`${base}/zh/docs/`);
    await page.waitForFunction(() => window.gloryReady === true);
    await page.locator('.docs-map').scrollIntoViewIfNeeded();
    await page.screenshot({ path: 'D:/BuildCache/glory-website/navigation-overview.png', fullPage: false });

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
