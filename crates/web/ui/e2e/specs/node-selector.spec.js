const { expect, test } = require("../base-test");
const { navigateAndWait, waitForWsConnected, watchPageErrors } = require("../helpers");

test.describe("Node selector", () => {
	test("node selector is hidden when no nodes connected", async ({ page }) => {
		const pageErrors = watchPageErrors(page);
		await navigateAndWait(page, "/chats/main");
		await waitForWsConnected(page);

		const nodeCombo = page.locator("#nodeCombo");
		await expect(nodeCombo).toBeHidden();

		expect(pageErrors).toEqual([]);
	});

	test("node selector exists in chat toolbar DOM", async ({ page }) => {
		const pageErrors = watchPageErrors(page);
		await navigateAndWait(page, "/chats/main");
		await waitForWsConnected(page);

		const nodeCombo = page.locator("#nodeCombo");
		await expect(nodeCombo).toHaveCount(1);

		const nodeComboBtn = page.locator("#nodeComboBtn");
		await expect(nodeComboBtn).toHaveCount(1);

		const nodeDropdown = page.locator("#nodeDropdown");
		await expect(nodeDropdown).toHaveCount(1);
		await expect(nodeDropdown).toBeHidden();

		expect(pageErrors).toEqual([]);
	});

	test("node combo label shows Local by default", async ({ page }) => {
		const pageErrors = watchPageErrors(page);
		await navigateAndWait(page, "/chats/main");
		await waitForWsConnected(page);

		const label = page.locator("#nodeComboLabel");
		await expect(label).toHaveText("Local");

		expect(pageErrors).toEqual([]);
	});

	test("node selector renders injected ssh target distinctly", async ({ page }) => {
		const pageErrors = watchPageErrors(page);
		await navigateAndWait(page, "/chats/main");
		await waitForWsConnected(page);

		await page.evaluate(async () => {
			const appScript = document.querySelector('script[type="module"][src*="js/app.js"]');
			if (!appScript) throw new Error("app.js module not found");
			const appUrl = new URL(appScript.src, window.location.origin);
			const prefix = appUrl.href.slice(0, appUrl.href.length - "js/app.js".length);
			const [{ setAll, select }, selector, state] = await Promise.all([
				import(`${prefix}js/stores/node-store.js`),
				import(`${prefix}js/nodes-selector.js`),
				import(`${prefix}js/state.js`),
			]);

			setAll([
				{
					nodeId: "ssh:deploy@box",
					displayName: "SSH: deploy@box",
					platform: "ssh",
				},
			]);
			select("ssh:deploy@box");
			state.nodeCombo.classList.remove("hidden");
			selector.restoreNodeSelection("ssh:deploy@box");
			selector.renderNodeList();
		});

		await expect(page.locator("#nodeCombo")).toBeVisible();
		await expect(page.locator("#nodeComboLabel")).toHaveText("SSH: deploy@box");
		await page.locator("#nodeComboBtn").click();
		await expect(page.locator("#nodeDropdown")).toBeVisible();
		await expect(page.getByText("OpenSSH target", { exact: true })).toBeVisible();

		expect(pageErrors).toEqual([]);
	});
});
