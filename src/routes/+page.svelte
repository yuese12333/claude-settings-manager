<script lang="ts">
  import PathBar from "$lib/components/PathBar.svelte";
  import ApiConfig from "$lib/pages/ApiConfig.svelte";
  import Plugins from "$lib/pages/Plugins.svelte";
  import OtherSettings from "$lib/pages/OtherSettings.svelte";
  import JsonEditor from "$lib/pages/JsonEditor.svelte";
  import {
    detectSettingsPath,
    loadSettings,
    pickSettingsFile,
    saveSettings,
    siblingSettingsPath,
  } from "$lib/api";
  import { checkAppUpdate } from "$lib/update";
  import { getVersion } from "@tauri-apps/api/app";
  import type { Settings } from "$lib/types";

  type Page = "api" | "plugins" | "other" | "json";

  let page = $state<Page>("api");
  let path = $state<string | null>(null);
  let sibling = $state<string | null>(null);
  let settings = $state<Settings | null>(null);
  let snapshot = $state("");
  let error = $state("");
  let busy = $state(false);
  let notice = $state("");
  let version = $state("");
  let updateHint = $state("");
  let updating = $state(false);
  let jsonDirty = $state(false);

  const dirty = $derived(!!settings && JSON.stringify(settings) !== snapshot);
  const leavingDirty = $derived(page === "json" ? jsonDirty : dirty);

  async function openPath(p: string) {
    error = "";
    notice = "";
    busy = true;
    try {
      const s = await loadSettings(p);
      path = p;
      settings = s;
      snapshot = JSON.stringify(s);
      sibling = await siblingSettingsPath(p);
    } catch (e) {
      error = String(e);
      settings = null;
      snapshot = "";
    } finally {
      busy = false;
    }
  }

  async function onCheckUpdate() {
    if (updating) return;
    updating = true;
    updateHint = "正在检查…";
    try {
      updateHint = await checkAppUpdate((msg) => (updateHint = msg));
    } catch {
      updateHint = "暂时无法检查更新";
    } finally {
      updating = false;
    }
  }

  async function boot() {
    try {
      version = await getVersion();
      const found = await detectSettingsPath();
      if (found) await openPath(found);
    } catch (e) {
      error = String(e);
    }
    if (!import.meta.env.DEV) void onCheckUpdate();
  }

  async function pick() {
    try {
      const p = await pickSettingsFile();
      if (p) await openPath(p);
    } catch (e) {
      error = String(e);
    }
  }

  async function switchFile(p: string) {
    if (leavingDirty && !confirm("有未保存修改，切换文件将丢弃。继续？")) return;
    await openPath(p);
  }

  function go(next: Page) {
    if (next === page) return;
    if (leavingDirty && !confirm("有未保存修改，切换页面将丢弃。继续？")) return;
    if (page !== "json" && dirty && snapshot) {
      settings = JSON.parse(snapshot) as Settings;
    }
    if (page === "json") jsonDirty = false;
    notice = "";
    page = next;
  }

  function reset() {
    if (!snapshot) return;
    settings = JSON.parse(snapshot) as Settings;
    notice = "";
  }

  async function save() {
    if (!path || !settings) return;
    error = "";
    busy = true;
    try {
      await saveSettings(path, settings);
      snapshot = JSON.stringify(settings);
      notice = "已保存（原文件备份为 .bak）";
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  function onJsonSaved(s: Settings) {
    settings = s;
    snapshot = JSON.stringify(s);
    jsonDirty = false;
    notice = "已保存（原文件备份为 .bak）";
  }

  boot();
</script>

<div class="shell">
  <PathBar {path} {sibling} onpick={pick} onswitch={switchFile} />

  <div class="body">
    <nav>
      <button class:on={page === "api"} onclick={() => go("api")}>API 配置</button>
      <button class:on={page === "plugins"} onclick={() => go("plugins")}>插件管理</button>
      <button class:on={page === "other"} onclick={() => go("other")}>其他设置</button>
      <button class:on={page === "json"} onclick={() => go("json")}>JSON 编辑</button>
      <div class="nav-foot">
        {#if version}<p>v{version}</p>{/if}
        <button type="button" disabled={updating} onclick={onCheckUpdate}>检查更新</button>
        {#if updateHint}<p>{updateHint}</p>{/if}
      </div>
    </nav>

    <main>
      {#if error}
        <p class="err">{error}</p>
      {/if}

      {#if !settings}
        <div class="empty">
          <h2>没有打开配置文件</h2>
          <p>启动时会探测 %APPDATA%\Claude 和 %USERPROFILE%\.claude。找不到就手动选。</p>
          <button type="button" onclick={pick}>选择 settings.json</button>
        </div>
      {:else}
        <div class="pane">
          {#if page === "api"}
            <ApiConfig bind:settings />
          {:else if page === "plugins"}
            <Plugins bind:settings />
          {:else if page === "other"}
            <OtherSettings bind:settings />
          {:else if path}
            <JsonEditor {path} onsaved={onJsonSaved} ondirty={(d) => (jsonDirty = d)} />
          {/if}
        </div>
        {#if page !== "json"}
          <footer>
            {#if notice}<span class="ok">{notice}</span>{/if}
            <button type="button" disabled={!dirty || busy} onclick={reset}>重置</button>
            <button class="save" type="button" disabled={!dirty || busy} onclick={save}>保存</button>
          </footer>
        {/if}
      {/if}
    </main>
  </div>
</div>

<style>
  .shell {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }
  .body {
    flex: 1;
    display: grid;
    grid-template-columns: 200px 1fr;
    min-height: 0;
  }
  nav {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 28px 18px 16px;
    background: var(--pine);
    color: #e8efe9;
  }
  nav button {
    text-align: left;
    border: 0;
    background: transparent;
    color: inherit;
    font: inherit;
    font-size: 15px;
    padding: 10px 12px;
    cursor: pointer;
    opacity: 0.7;
    border-left: 3px solid transparent;
  }
  nav button.on {
    opacity: 1;
    border-left-color: var(--amber);
  }
  .nav-foot {
    margin-top: auto;
    padding: 12px 0 0;
    opacity: 0.85;
  }
  .nav-foot p {
    margin: 6px 0 0;
    font-size: 12px;
    opacity: 0.8;
  }
  .nav-foot button {
    opacity: 1;
    padding: 8px 0;
    border-left: 0;
  }
  main {
    position: relative;
    padding: 28px 36px 24px;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .pane {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
    animation: in 180ms ease;
  }
  footer {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 16px;
    padding-top: 20px;
  }
  footer button,
  .empty button {
    border: 0;
    background: transparent;
    font: inherit;
    cursor: pointer;
    padding: 6px 0;
    color: var(--ink);
  }
  footer button:disabled {
    opacity: 0.35;
    cursor: default;
  }
  .save {
    border-bottom: 2px solid var(--amber);
  }
  .empty {
    margin: auto;
    max-width: 420px;
  }
  .empty h2 {
    font-family: var(--display);
    margin: 0 0 8px;
  }
  .empty p {
    color: var(--muted);
  }
  .empty button {
    margin-top: 12px;
    border-bottom: 2px solid var(--amber);
  }
  .err {
    color: #9b2c1a;
    margin: 0 0 12px;
    font-size: 13px;
  }
  .ok {
    margin-right: auto;
    color: var(--pine);
    font-size: 13px;
  }
  @keyframes in {
    from { opacity: 0; transform: translateY(6px); }
  }
</style>
