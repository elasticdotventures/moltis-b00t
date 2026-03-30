// ── Nodes page ──────────────────────────────────────────────

import { signal } from "@preact/signals";
import { html } from "htm/preact";
import { render } from "preact";
import { useEffect } from "preact/hooks";
import { onEvent } from "./events.js";
import { sendRpc } from "./helpers.js";
import { navigate } from "./router.js";
import { settingsPath } from "./routes.js";
import { ConfirmDialog, requestConfirm } from "./ui.js";

// ── Signals ─────────────────────────────────────────────────
var nodes = signal([]);
var pendingPairs = signal([]);
var pairedDevices = signal([]);
var loading = signal(false);
var activeTab = signal("connected"); // "connected" | "paired" | "pending"
var toasts = signal([]);
var toastId = 0;
var generatedToken = signal(null); // { token, deviceId, command }
var generatingToken = signal(false);
var deviceName = signal("");
var doctor = signal(null);
var doctorLoading = signal(false);
var doctorError = signal("");
var doctorTest = signal(null);
var doctorTestLoading = signal(false);
var doctorPinLoading = signal(false);

// ── Helpers ─────────────────────────────────────────────────

function isSshTargetNode(node) {
	return node?.platform === "ssh" || String(node?.nodeId || "").startsWith("ssh:");
}

function sshTargetValue(node) {
	if (!node) return "";
	if (String(node.nodeId || "").startsWith("ssh:")) {
		return String(node.nodeId).slice(4);
	}
	return String(node.displayName || "")
		.replace(/^SSH:\s*/i, "")
		.trim();
}

function nodeDisplayLabel(node) {
	if (!node) return "Local";
	if (isSshTargetNode(node)) {
		var target = sshTargetValue(node);
		return target ? `SSH: ${target}` : node.displayName || node.nodeId;
	}
	return node.displayName || node.nodeId;
}

function gatewayWsUrl() {
	var proto = location.protocol === "https:" ? "wss:" : "ws:";
	var host = location.hostname;
	var port = location.port;
	return `${proto}//${host}${port ? `:${port}` : ""}/ws`;
}

async function generateToken() {
	generatingToken.value = true;
	var name = deviceName.value.trim() || null;
	var res = await sendRpc("device.token.create", {
		displayName: name,
		platform: "remote",
	});
	if (res?.ok) {
		var wsUrl = gatewayWsUrl();
		generatedToken.value = {
			token: res.payload.deviceToken,
			deviceId: res.payload.deviceId,
			command: `moltis node add --host ${wsUrl} --token ${res.payload.deviceToken}`,
		};
		showToast("Token generated", "success");
		await refreshPairedDevices();
	} else {
		showToast(res?.error?.message || "Failed to generate token", "error");
	}
	generatingToken.value = false;
}

function copyToClipboard(text) {
	if (navigator.clipboard?.writeText) {
		navigator.clipboard.writeText(text).then(() => showToast("Copied to clipboard", "success"));
	}
}
function showToast(message, type) {
	var id = ++toastId;
	toasts.value = toasts.value.concat([{ id: id, message: message, type: type }]);
	setTimeout(() => {
		toasts.value = toasts.value.filter((t) => t.id !== id);
	}, 4000);
}

async function refreshNodes() {
	loading.value = true;
	try {
		var res = await sendRpc("node.list", {});
		if (res?.ok) nodes.value = res.payload || [];
	} catch {
		// ignore
	}
	loading.value = false;
}

async function refreshPendingPairs() {
	try {
		var res = await sendRpc("node.pair.list", {});
		if (res?.ok) pendingPairs.value = res.payload || [];
	} catch {
		// ignore
	}
}

async function refreshPairedDevices() {
	try {
		var res = await sendRpc("device.pair.list", {});
		if (res?.ok) pairedDevices.value = res.payload || [];
	} catch {
		// ignore
	}
}

async function refreshDoctor() {
	doctorLoading.value = true;
	doctorError.value = "";
	try {
		var response = await fetch("/api/ssh/doctor");
		if (!response.ok) {
			throw new Error("Failed to load remote exec status");
		}
		doctor.value = await response.json();
	} catch (error) {
		doctorError.value = error.message || "Failed to load remote exec status";
	} finally {
		doctorLoading.value = false;
	}
}

async function testActiveSshRoute() {
	doctorTestLoading.value = true;
	doctorError.value = "";
	try {
		var response = await fetch("/api/ssh/doctor/test-active", { method: "POST" });
		var data = await response.json();
		if (!response.ok) {
			throw new Error(data?.error || "Failed to test SSH route");
		}
		doctorTest.value = data;
		showToast(
			data.reachable ? "Active SSH route is reachable" : data.failure_hint || "Active SSH route check failed",
			data.reachable ? "success" : "error",
		);
	} catch (error) {
		doctorError.value = error.message || "Failed to test SSH route";
		showToast(doctorError.value, "error");
	} finally {
		doctorTestLoading.value = false;
	}
}

async function repairActiveRouteHostPin() {
	var snapshot = doctor.value;
	var activeRoute = snapshot?.active_route || null;
	if (!activeRoute?.target_id) {
		showToast("The active SSH route cannot be managed from the doctor panel", "error");
		return;
	}

	doctorPinLoading.value = true;
	doctorError.value = "";
	try {
		var scanResponse = await fetch("/api/ssh/host-key/scan", {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({
				target: activeRoute.target,
				port: activeRoute.port ?? null,
			}),
		});
		var scanData = await scanResponse.json();
		if (!scanResponse.ok) {
			throw new Error(scanData?.error || "Failed to scan SSH host key");
		}

		var pinResponse = await fetch(`/api/ssh/targets/${activeRoute.target_id}/pin`, {
			method: "POST",
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify({ known_host: scanData.known_host }),
		});
		var pinData = await pinResponse.json();
		if (!pinResponse.ok) {
			throw new Error(pinData?.error || "Failed to pin SSH host key");
		}

		await refreshDoctor();
		showToast(activeRoute.host_pinned ? "Active SSH host pin refreshed" : "Active SSH host pinned", "success");
	} catch (error) {
		doctorError.value = error.message || "Failed to repair SSH host pin";
		showToast(doctorError.value, "error");
	} finally {
		doctorPinLoading.value = false;
	}
}

async function clearActiveRouteHostPin() {
	var snapshot = doctor.value;
	var activeRoute = snapshot?.active_route || null;
	if (!activeRoute?.target_id) {
		showToast("The active SSH route cannot be managed from the doctor panel", "error");
		return;
	}

	doctorPinLoading.value = true;
	doctorError.value = "";
	try {
		var response = await fetch(`/api/ssh/targets/${activeRoute.target_id}/pin`, {
			method: "DELETE",
		});
		var data = await response.json();
		if (!response.ok) {
			throw new Error(data?.error || "Failed to clear SSH host pin");
		}
		await refreshDoctor();
		showToast("Active SSH host pin cleared", "success");
	} catch (error) {
		doctorError.value = error.message || "Failed to clear SSH host pin";
		showToast(doctorError.value, "error");
	} finally {
		doctorPinLoading.value = false;
	}
}

async function refreshAll() {
	await Promise.all([refreshNodes(), refreshPendingPairs(), refreshPairedDevices(), refreshDoctor()]);
}

async function approvePair(id) {
	var res = await sendRpc("node.pair.approve", { id });
	if (res?.ok) {
		showToast("Pairing approved — device token issued", "success");
		await refreshAll();
	} else {
		showToast(res?.error?.message || "Failed to approve", "error");
	}
}

async function rejectPair(id) {
	var res = await sendRpc("node.pair.reject", { id });
	if (res?.ok) {
		showToast("Pairing rejected", "success");
		await refreshAll();
	} else {
		showToast(res?.error?.message || "Failed to reject", "error");
	}
}

async function revokeDevice(deviceId) {
	var ok = await requestConfirm(
		`Revoke device "${deviceId}"?`,
		"This will disconnect the device and invalidate its token.",
	);
	if (!ok) return;
	var res = await sendRpc("device.token.revoke", { deviceId });
	if (res?.ok) {
		showToast("Device token revoked", "success");
		await refreshAll();
	} else {
		showToast(res?.error?.message || "Failed to revoke", "error");
	}
}

// ── Components ──────────────────────────────────────────────

function TabBar() {
	var tabs = [
		{ id: "connected", label: "Connected", count: nodes.value.length },
		{ id: "paired", label: "Paired Devices", count: pairedDevices.value.length },
		{ id: "pending", label: "Pending", count: pendingPairs.value.length },
	];

	return html`<div class="flex gap-1 mb-4">
		${tabs.map(
			(t) =>
				html`<button
					key=${t.id}
					class="px-3 py-1.5 text-sm rounded-md transition-colors ${
						activeTab.value === t.id
							? "bg-[var(--accent)] text-white"
							: "bg-[var(--surface-alt)] text-[var(--text-muted)] hover:bg-[var(--hover)]"
					}"
					onClick=${() => (activeTab.value = t.id)}
				>
					${t.label}${t.count > 0 ? html` <span class="ml-1 opacity-70">(${t.count})</span>` : null}
				</button>`,
		)}
	</div>`;
}

function ConnectNodeForm() {
	var wsUrl = gatewayWsUrl();
	return html`<div class="rounded-lg border border-[var(--border)] bg-[var(--surface-alt)] p-4">
		<h3 class="text-sm font-medium text-[var(--text-strong)] mb-3">Connect a Remote Node</h3>
		<p class="text-xs text-[var(--text-muted)] mb-3">
			Generate a token and run the command on the remote machine you want to connect.
		</p>

		<div class="mb-3">
			<label class="block text-xs text-[var(--text-muted)] mb-1">This gateway's public endpoint</label>
			<div class="flex items-center gap-2">
				<code class="flex-1 text-xs bg-[var(--bg)] px-2 py-1.5 rounded border border-[var(--border)] break-all">
					${wsUrl}
				</code>
				<button
					class="provider-btn provider-btn-secondary provider-btn-sm shrink-0"
					onClick=${() => copyToClipboard(wsUrl)}
				>
					Copy
				</button>
			</div>
			<p class="text-xs text-[var(--text-muted)] mt-1">
				The remote node will connect back to this address. Replace with your public IP or domain if needed.
			</p>
		</div>

		<div class="mb-3">
			<label class="block text-xs text-[var(--text-muted)] mb-1">Remote node name (optional)</label>
			<input
				type="text"
				class="w-full text-sm bg-[var(--bg)] px-2 py-1.5 rounded border border-[var(--border)] text-[var(--text-strong)] placeholder-[var(--text-muted)]"
				placeholder="e.g. my-server"
				value=${deviceName.value}
				onInput=${(e) => (deviceName.value = e.target.value)}
			/>
		</div>

		<button
			class="provider-btn text-sm px-3 py-1.5 w-full"
			onClick=${generateToken}
			disabled=${generatingToken.value}
		>
			${generatingToken.value ? "Generating..." : "Generate Connection Token"}
		</button>

		${
			generatedToken.value
				? html`<div class="mt-3 p-3 rounded bg-[var(--bg)] border border-[var(--border)]">
					<div class="flex items-center justify-between mb-2">
						<span class="text-xs font-medium text-green-500">Token generated</span>
						<button
							class="provider-btn provider-btn-secondary provider-btn-sm"
							onClick=${() => copyToClipboard(generatedToken.value.command)}
						>
							Copy command
						</button>
					</div>
					<code
						class="block text-xs break-all bg-[var(--surface-alt)] px-2 py-1.5 rounded border border-[var(--border)] select-all"
					>
						${generatedToken.value.command}
					</code>
					<p class="text-xs text-[var(--text-muted)] mt-2">
						Run this command on the remote machine to connect. The token is shown only once — copy it now.
					</p>
				</div>`
				: null
		}

		<p class="text-xs text-[var(--text-muted)] mt-3">
			Manage tokens in the <button
				class="underline hover:text-[var(--text-strong)]"
				onClick=${() => (activeTab.value = "paired")}
			>Paired Devices</button> tab.
		</p>
	</div>`;
}

function formatBytes(bytes) {
	if (bytes == null) return null;
	var gb = bytes / 1073741824;
	if (gb >= 1) return `${gb.toFixed(1)} GB`;
	var mb = bytes / 1048576;
	return `${mb.toFixed(0)} MB`;
}

function TelemetryBar({ label, value, max }) {
	if (value == null || max == null || max === 0) return null;
	var pct = Math.min(100, Math.max(0, (value / max) * 100));
	var color = pct > 80 ? "bg-red-500" : pct > 60 ? "bg-yellow-500" : "bg-green-500";
	return html`<div class="flex items-center gap-2 text-xs text-[var(--text-muted)]">
		<span class="w-8 shrink-0">${label}</span>
		<div class="flex-1 h-1.5 rounded bg-[var(--border)] overflow-hidden">
			<div class="${color} h-full rounded" style="width:${pct.toFixed(1)}%"></div>
		</div>
		<span class="w-16 text-right shrink-0">${pct.toFixed(0)}%</span>
	</div>`;
}

function NodeTelemetry({ telemetry }) {
	if (!telemetry) return null;
	var parts = [];
	if (telemetry.cpuCount != null) {
		parts.push(html`<span>${telemetry.cpuCount} cores</span>`);
	}
	if (telemetry.memTotal != null) {
		parts.push(html`<span>${formatBytes(telemetry.memTotal)} RAM</span>`);
	}
	if (telemetry.uptimeSecs != null) {
		var h = Math.floor(telemetry.uptimeSecs / 3600);
		var d = Math.floor(h / 24);
		var uptimeStr = d > 0 ? `${d}d ${h % 24}h` : `${h}h`;
		parts.push(html`<span>up ${uptimeStr}</span>`);
	}
	if (telemetry.stale) {
		parts.push(html`<span class="text-yellow-500">(stale)</span>`);
	}

	return html`<div class="mt-1.5 flex flex-col gap-1">
		${telemetry.cpuUsage != null ? html`<${TelemetryBar} label="CPU" value=${telemetry.cpuUsage} max=${100} />` : null}
		${
			telemetry.memTotal != null && telemetry.memAvailable != null
				? html`<${TelemetryBar}
						label="MEM"
						value=${telemetry.memTotal - telemetry.memAvailable}
						max=${telemetry.memTotal}
					/>`
				: null
		}
		${parts.length > 0 ? html`<div class="text-xs text-[var(--text-muted)] flex gap-2 flex-wrap">${parts}</div>` : null}
	</div>`;
}

function DoctorBadge({ level }) {
	var tone =
		level === "error"
			? "bg-red-500/15 text-red-500"
			: level === "warn"
				? "bg-yellow-500/15 text-yellow-500"
				: "bg-green-500/15 text-green-500";
	return html`<span class="text-[10px] uppercase tracking-wide px-1.5 py-0.5 rounded ${tone}">${level}</span>`;
}

function RemoteExecStatusCard() {
	var snapshot = doctor.value;
	var execHost = snapshot?.exec_host || "local";
	var activeRoute = snapshot?.active_route || null;
	var checkList = snapshot?.checks || [];
	var canManageActivePin = Boolean(activeRoute?.target_id);

	return html`<div class="rounded-lg border border-[var(--border)] bg-[var(--surface-alt)] p-4 flex flex-col gap-3">
		<div class="flex items-start justify-between gap-3 flex-wrap">
			<div>
				<h3 class="text-sm font-medium text-[var(--text-strong)] mb-1">Remote Exec Status</h3>
				<p class="text-xs text-[var(--text-muted)] m-0">
					Moltis is currently configured to run commands through
					<strong class="text-[var(--text-strong)]"> ${execHost}</strong>
					${activeRoute ? html` using <code>${activeRoute.label}</code>` : null}.
				</p>
				${
					activeRoute
						? html`<div class="text-xs text-[var(--text-muted)] mt-1">
							${
								activeRoute.host_pinned
									? "Active route is pinned to a stored host key."
									: canManageActivePin
										? "Active route is currently inheriting global known_hosts policy."
										: "Active route is not directly manageable here because it comes from legacy config."
							}
						</div>`
						: null
				}
			</div>
			<div class="flex gap-2 flex-wrap">
				<button
					class="provider-btn provider-btn-secondary provider-btn-sm"
					onClick=${refreshDoctor}
					disabled=${doctorLoading.value}
				>
					${doctorLoading.value ? "Refreshing..." : "Refresh Doctor"}
				</button>
				${
					execHost === "ssh" && activeRoute
						? html`<button
							class="provider-btn provider-btn-secondary provider-btn-sm"
							onClick=${testActiveSshRoute}
							disabled=${doctorTestLoading.value}
						>
							${doctorTestLoading.value ? "Testing..." : "Test Active SSH Route"}
						</button>`
						: null
				}
				${
					execHost === "ssh" && activeRoute && canManageActivePin
						? html`<button
							class="provider-btn provider-btn-secondary provider-btn-sm"
							onClick=${repairActiveRouteHostPin}
							disabled=${doctorPinLoading.value}
						>
							${doctorPinLoading.value ? "Scanning..." : activeRoute.host_pinned ? "Refresh Active Pin" : "Pin Active Route"}
						</button>`
						: null
				}
				${
					execHost === "ssh" && activeRoute?.host_pinned && canManageActivePin
						? html`<button
							class="provider-btn provider-btn-secondary provider-btn-sm"
							onClick=${clearActiveRouteHostPin}
							disabled=${doctorPinLoading.value}
						>
							${doctorPinLoading.value ? "Clearing..." : "Clear Active Pin"}
						</button>`
						: null
				}
				<button
					class="provider-btn provider-btn-secondary provider-btn-sm"
					onClick=${() => navigate(settingsPath("ssh"))}
				>
					SSH Settings
				</button>
			</div>
		</div>

		<div class="grid gap-2 md:grid-cols-5">
			<div class="rounded border border-[var(--border)] bg-[var(--bg)] px-3 py-2">
				<div class="text-[10px] uppercase tracking-wide text-[var(--text-muted)]">Backend</div>
				<div class="text-sm text-[var(--text-strong)] mt-1">${execHost}</div>
			</div>
			<div class="rounded border border-[var(--border)] bg-[var(--bg)] px-3 py-2">
				<div class="text-[10px] uppercase tracking-wide text-[var(--text-muted)]">Paired Nodes</div>
				<div class="text-sm text-[var(--text-strong)] mt-1">${snapshot?.paired_node_count ?? 0}</div>
			</div>
			<div class="rounded border border-[var(--border)] bg-[var(--bg)] px-3 py-2">
				<div class="text-[10px] uppercase tracking-wide text-[var(--text-muted)]">Managed Targets</div>
				<div class="text-sm text-[var(--text-strong)] mt-1">
					${snapshot?.managed_target_count ?? 0}
					${snapshot?.pinned_target_count ? html` <span class="text-xs text-[var(--text-muted)]">(${snapshot.pinned_target_count} pinned)</span>` : null}
				</div>
			</div>
			<div class="rounded border border-[var(--border)] bg-[var(--bg)] px-3 py-2">
				<div class="text-[10px] uppercase tracking-wide text-[var(--text-muted)]">Managed Keys</div>
				<div class="text-sm text-[var(--text-strong)] mt-1">
					${snapshot?.managed_key_count ?? 0}
					${snapshot?.encrypted_key_count ? html` <span class="text-xs text-[var(--text-muted)]">(${snapshot.encrypted_key_count} encrypted)</span>` : null}
				</div>
			</div>
		</div>

		${doctorError.value ? html`<div class="text-xs text-red-500">${doctorError.value}</div>` : null}

		${
			doctorTest.value
				? html`<div class="rounded border border-[var(--border)] bg-[var(--bg)] px-3 py-2 text-xs">
					<div class="font-medium text-[var(--text-strong)]">
						${doctorTest.value.route_label || "Active SSH route"}
					</div>
					<div class="${doctorTest.value.reachable ? "text-green-500" : "text-red-500"} mt-1">
						${doctorTest.value.reachable ? "Reachable" : "Unreachable"}${doctorTest.value.exit_code != null ? ` (exit ${doctorTest.value.exit_code})` : ""}
					</div>
					${
						doctorTest.value.failure_hint
							? html`<div class="mt-1 text-[11px] text-[var(--text-muted)]">
								Hint: ${doctorTest.value.failure_hint}
							</div>`
							: null
					}
					${
						doctorTest.value.stderr
							? html`<pre class="mt-2 whitespace-pre-wrap break-all text-[11px] text-[var(--text-muted)]">${doctorTest.value.stderr}</pre>`
							: null
					}
				</div>`
				: null
		}

		<div class="flex flex-col gap-2">
			${checkList.map(
				(check) => html`<div class="rounded border border-[var(--border)] bg-[var(--bg)] px-3 py-2">
					<div class="flex items-center gap-2 flex-wrap">
						<div class="text-sm text-[var(--text-strong)]">${check.title}</div>
						<${DoctorBadge} level=${check.level} />
					</div>
					<div class="text-xs text-[var(--text-muted)] mt-1">${check.message}</div>
					${check.hint ? html`<div class="text-xs text-[var(--text-muted)] mt-1">Hint: ${check.hint}</div>` : null}
				</div>`,
			)}
		</div>
	</div>`;
}

function SshTargetCard({ node }) {
	var target = sshTargetValue(node) || "configured target";
	return html`<div
		class="flex items-start gap-3 p-3 rounded-lg bg-[var(--surface-alt)] border border-[var(--border)]"
	>
		<div class="w-2 h-2 rounded-full bg-sky-500 shrink-0 mt-1" title="Configured SSH target"></div>
		<div class="flex-1 min-w-0">
			<div class="flex items-center gap-2 flex-wrap">
				<div class="text-sm font-medium text-[var(--text-strong)] truncate">${nodeDisplayLabel(node)}</div>
				<span class="text-[10px] uppercase tracking-wide px-1.5 py-0.5 rounded bg-sky-500/15 text-sky-500">
					ssh
				</span>
				<span class="text-[10px] uppercase tracking-wide px-1.5 py-0.5 rounded bg-[var(--bg)] text-[var(--text-muted)] border border-[var(--border)]">
					configured
				</span>
			</div>
			<div class="text-xs text-[var(--text-muted)] mt-1">
				<code>${target}</code>
			</div>
			<p class="text-xs text-[var(--text-muted)] mt-2 mb-0">
				Uses your local OpenSSH configuration for remote exec. This is an execution route, not a paired
				WebSocket node, so telemetry and presence are not available here.
			</p>
		</div>
	</div>`;
}

function ConnectedNodesList() {
	var sshTargets = nodes.value.filter(isSshTargetNode);
	var connectedNodes = nodes.value.filter((node) => !isSshTargetNode(node));

	if (connectedNodes.length === 0 && sshTargets.length === 0) {
		return html`<div class="flex flex-col gap-4">
			<div class="text-sm text-[var(--text-muted)] py-4 text-center">
				<p>No nodes connected.</p>
			</div>
			<${ConnectNodeForm} />
		</div>`;
	}

	return html`<div class="flex flex-col gap-2">
		${
			sshTargets.length > 0 &&
			html`<div class="flex flex-col gap-2">
				<div class="text-xs uppercase tracking-wide text-[var(--text-muted)]">Configured SSH Targets</div>
				${sshTargets.map((node) => html`<${SshTargetCard} key=${node.nodeId} node=${node} />`)}
			</div>`
		}
		${
			connectedNodes.length > 0 &&
			html`<div class="flex flex-col gap-2">
				<div class="text-xs uppercase tracking-wide text-[var(--text-muted)]">Connected Paired Nodes</div>
				${connectedNodes.map(
					(n) =>
						html`<div
							key=${n.nodeId}
							class="flex items-center gap-3 p-3 rounded-lg bg-[var(--surface-alt)] border border-[var(--border)]"
						>
							<div class="w-2 h-2 rounded-full bg-green-500 shrink-0" title="Connected"></div>
							<div class="flex-1 min-w-0">
								<div class="text-sm font-medium text-[var(--text-strong)] truncate">
									${nodeDisplayLabel(n)}
								</div>
								<div class="text-xs text-[var(--text-muted)]">
									${n.platform || "unknown"} · v${n.version || "?"}
									${n.remoteIp ? html` · ${n.remoteIp}` : null}
								</div>
								${
									n.capabilities?.length
										? html`<div class="text-xs text-[var(--text-muted)] mt-1">
											caps: ${n.capabilities.join(", ")}
										</div>`
										: null
								}
								<${NodeTelemetry} telemetry=${n.telemetry} />
							</div>
						</div>`,
				)}
			</div>`
		}
	</div>`;
}

function PairedDevicesList() {
	if (pairedDevices.value.length === 0) {
		return html`<div class="text-sm text-[var(--text-muted)] py-8 text-center">
			No paired devices.
		</div>`;
	}

	return html`<div class="flex flex-col gap-2">
		${pairedDevices.value.map(
			(d) =>
				html`<div
					key=${d.deviceId}
					class="flex items-center gap-3 p-3 rounded-lg bg-[var(--surface-alt)] border border-[var(--border)]"
				>
					<div class="flex-1 min-w-0">
						<div class="text-sm font-medium text-[var(--text-strong)] truncate">
							${d.displayName || d.deviceId}
						</div>
						<div class="text-xs text-[var(--text-muted)]">
							${d.platform || "unknown"}
							${d.createdAt ? html` · paired ${d.createdAt}` : null}
						</div>
					</div>
					<button
						class="provider-btn-danger text-xs px-2 py-1"
						onClick=${() => revokeDevice(d.deviceId)}
					>
						Revoke
					</button>
				</div>`,
		)}
	</div>`;
}

function PendingPairsList() {
	if (pendingPairs.value.length === 0) {
		return html`<div class="text-sm text-[var(--text-muted)] py-8 text-center">
			No pending pairing requests.
		</div>`;
	}

	return html`<div class="flex flex-col gap-2">
		${pendingPairs.value.map(
			(r) =>
				html`<div
					key=${r.id}
					class="flex items-center gap-3 p-3 rounded-lg bg-[var(--surface-alt)] border border-[var(--border)]"
				>
					<div class="flex-1 min-w-0">
						<div class="text-sm font-medium text-[var(--text-strong)] truncate">
							${r.displayName || r.deviceId}
						</div>
						<div class="text-xs text-[var(--text-muted)]">${r.platform || "unknown"}</div>
					</div>
					<div class="flex gap-1.5">
						<button
							class="provider-btn text-xs px-2 py-1"
							onClick=${() => approvePair(r.id)}
						>
							Approve
						</button>
						<button
							class="provider-btn-secondary text-xs px-2 py-1"
							onClick=${() => rejectPair(r.id)}
						>
							Reject
						</button>
					</div>
				</div>`,
		)}
	</div>`;
}

function Toasts() {
	if (toasts.value.length === 0) return null;
	return html`<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
		${toasts.value.map(
			(t) =>
				html`<div
					key=${t.id}
					class="px-4 py-2 rounded-lg text-sm shadow-lg ${
						t.type === "error" ? "bg-red-600 text-white" : "bg-green-600 text-white"
					}"
				>
					${t.message}
				</div>`,
		)}
	</div>`;
}

// ── Main component ──────────────────────────────────────────

function NodesPage() {
	useEffect(() => {
		refreshAll();

		// Subscribe to presence events for live updates.
		var unsub = onEvent("presence", () => {
			refreshNodes();
		});
		var unsubPair = onEvent("node.pair.requested", () => {
			refreshPendingPairs();
		});
		var unsubResolved = onEvent("node.pair.resolved", () => {
			refreshAll();
		});
		var unsubDevice = onEvent("device.pair.resolved", () => {
			refreshAll();
		});
		// Live telemetry updates — merge into cached node list.
		var unsubTelemetry = onEvent("node.telemetry", (payload) => {
			if (!payload?.nodeId) return;
			var updated = nodes.value.map((n) => {
				if (n.nodeId !== payload.nodeId) return n;
				return Object.assign({}, n, {
					telemetry: {
						memTotal: payload.mem?.total ?? n.telemetry?.memTotal,
						memAvailable: payload.mem?.available ?? n.telemetry?.memAvailable,
						cpuCount: payload.cpuCount ?? n.telemetry?.cpuCount,
						cpuUsage: payload.cpuUsage ?? n.telemetry?.cpuUsage,
						uptimeSecs: payload.uptime ?? n.telemetry?.uptimeSecs,
						services: payload.services ?? n.telemetry?.services ?? [],
						stale: false,
					},
				});
			});
			nodes.value = updated;
		});

		return () => {
			unsub();
			unsubPair();
			unsubResolved();
			unsubDevice();
			unsubTelemetry();
		};
	}, []);

	return html`<div class="flex-1 flex flex-col min-w-0 p-4 gap-4 overflow-y-auto">
		<div class="max-w-form flex flex-col gap-4">
			<div>
				<div class="flex items-center gap-3 mb-1">
					<h2 class="text-lg font-medium text-[var(--text-strong)]">Nodes</h2>
					<button
						class="provider-btn provider-btn-secondary provider-btn-sm"
						onClick=${refreshAll}
						disabled=${loading.value}
					>
						${loading.value ? "Refreshing..." : "Refresh"}
					</button>
				</div>
				<p class="text-xs text-[var(--muted)] leading-relaxed" style="margin:0;">
					Nodes are remote execution targets. Paired nodes stream telemetry and
					capabilities back to the gateway, while configured SSH targets route
					commands through your local OpenSSH setup. The agent can choose where to
					run commands based on what is available.
				</p>
			</div>

			<${RemoteExecStatusCard} />

			<${TabBar} />

			${
				activeTab.value === "connected"
					? html`<${ConnectedNodesList} />
						${nodes.value.length > 0 ? html`<${ConnectNodeForm} />` : null}`
					: null
			}
			${activeTab.value === "paired" ? html`<${PairedDevicesList} />` : null}
			${activeTab.value === "pending" ? html`<${PendingPairsList} />` : null}
		</div>

		<${Toasts} />
		<${ConfirmDialog} />
	</div>`;
}

// ── Mount / unmount ─────────────────────────────────────────

var _mounted = false;
var containerRef = null;

export function initNodes(container) {
	_mounted = true;
	containerRef = container;
	render(html`<${NodesPage} />`, container);
}

export function teardownNodes() {
	_mounted = false;
	if (containerRef) render(null, containerRef);
	containerRef = null;
}
