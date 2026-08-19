<script lang="ts">
  import ApiKeyInput from "$lib/components/ApiKeyInput.svelte";
  import { loadProfiles, saveProfiles } from "$lib/api";
  import type { Profile } from "$lib/types";

  let {
    currentUrl,
    currentKey,
    onapply,
  }: {
    currentUrl: string;
    currentKey: string;
    onapply: (p: Profile) => void;
  } = $props();

  let profiles = $state<Profile[]>([]);
  let q = $state("");
  let err = $state("");
  let open = $state(false);
  let editing = $state<string | null>(null);
  let name = $state("");
  let baseUrl = $state("");
  let apiKey = $state("");

  const shown = $derived(
    profiles.filter((p) => {
      const s = q.trim().toLowerCase();
      if (!s) return true;
      return p.name.toLowerCase().includes(s) || p.baseUrl.toLowerCase().includes(s);
    }),
  );

  function host(url: string) {
    try {
      return new URL(url).host || url;
    } catch {
      return url || "—";
    }
  }

  async function persist(next: Profile[]) {
    err = "";
    await saveProfiles(next);
    profiles = next;
  }

  function startAdd() {
    editing = null;
    name = "";
    baseUrl = currentUrl;
    apiKey = currentKey;
    open = true;
  }

  function startEdit(p: Profile) {
    editing = p.id;
    name = p.name;
    baseUrl = p.baseUrl;
    apiKey = p.apiKey;
    open = true;
  }

  async function commit() {
    const n = name.trim();
    if (!n) return;
    const item: Profile = {
      id: editing ?? crypto.randomUUID(),
      name: n,
      baseUrl: baseUrl.trim(),
      apiKey: apiKey.trim(),
    };
    const next = editing
      ? profiles.map((p) => (p.id === editing ? item : p))
      : [...profiles, item];
    try {
      await persist(next);
      open = false;
    } catch (e) {
      err = String(e);
    }
  }

  async function remove(p: Profile) {
    if (!confirm(`删除配置组「${p.name}」？`)) return;
    try {
      await persist(profiles.filter((x) => x.id !== p.id));
    } catch (e) {
      err = String(e);
    }
  }

  loadProfiles()
    .then((list) => (profiles = list))
    .catch((e) => (err = String(e)));
</script>

<div class="box">
  <div class="head">
    <h3>配置组</h3>
    <button type="button" onclick={startAdd}>+ 新建</button>
  </div>
  <input class="search" type="search" placeholder="查找名称或 Base URL" bind:value={q} />
  {#if err}<p class="err">{err}</p>{/if}
  {#if shown.length === 0}
    <p class="empty">{profiles.length ? "没有匹配的配置组" : "还没有配置组"}</p>
  {:else}
    <ul>
      {#each shown as p (p.id)}
        <li>
          <div class="meta">
            <strong>{p.name}</strong>
            <span>{host(p.baseUrl)}</span>
          </div>
          <button type="button" class="use" onclick={() => onapply(p)}>套用</button>
          <button type="button" onclick={() => startEdit(p)}>修改</button>
          <button type="button" class="del" onclick={() => remove(p)}>删除</button>
        </li>
      {/each}
    </ul>
  {/if}
</div>

{#if open}
  <div class="mask" onclick={() => (open = false)} role="presentation"></div>
  <form
    class="dlg"
    onsubmit={(e) => {
      e.preventDefault();
      void commit();
    }}
  >
    <h3>{editing ? "修改配置组" : "新建配置组"}</h3>
    <label for="pname">名称</label>
    <input id="pname" placeholder="例如 官方 / 中转" bind:value={name} />
    <label for="purl">Base URL</label>
    <input id="purl" type="url" placeholder="https://api.anthropic.com" bind:value={baseUrl} />
    <label for="pkey">API Key</label>
    <ApiKeyInput id="pkey" value={apiKey} onchange={(v) => (apiKey = v)} />
    <div class="acts">
      <button type="button" onclick={() => (open = false)}>取消</button>
      <button type="submit">保存</button>
    </div>
  </form>
{/if}

<style>
  .box {
    margin: 0 0 32px;
  }
  .head {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 12px;
  }
  h3 {
    margin: 0;
    font-family: var(--display);
    font-size: 18px;
    font-weight: 600;
  }
  .head button,
  .acts button,
  li button {
    border: 0;
    background: transparent;
    font: inherit;
    cursor: pointer;
    padding: 4px 0;
    color: var(--ink);
  }
  .head button {
    border-bottom: 2px solid var(--amber);
  }
  .search {
    width: 100%;
    margin: 12px 0 8px;
    border: 0;
    border-bottom: 1px solid color-mix(in srgb, var(--ink) 28%, transparent);
    background: transparent;
    padding: 8px 0;
    font: inherit;
    outline: none;
  }
  ul {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  li {
    display: grid;
    grid-template-columns: 1fr auto auto auto;
    gap: 12px;
    align-items: center;
    padding: 10px 0;
    border-bottom: 1px solid color-mix(in srgb, var(--ink) 10%, transparent);
  }
  .meta {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .meta strong {
    font-weight: 600;
  }
  .meta span {
    color: var(--muted);
    font-size: 12px;
    font-family: var(--mono);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .use {
    border-bottom: 2px solid var(--amber);
  }
  .del:hover {
    color: #9b2c1a;
  }
  .empty,
  .err {
    font-size: 13px;
  }
  .err {
    color: #9b2c1a;
  }
  .empty {
    color: var(--muted);
  }
  .mask {
    position: fixed;
    inset: 0;
    background: color-mix(in srgb, var(--pine) 35%, transparent);
    animation: fade 160ms ease;
  }
  .dlg {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: min(420px, calc(100vw - 48px));
    background: var(--paper);
    padding: 24px 28px 20px;
    display: grid;
    gap: 4px;
    animation: pop 180ms ease;
    z-index: 1;
  }
  .dlg label {
    margin-top: 10px;
    font-size: 12px;
    color: var(--muted);
  }
  .dlg input {
    border: 0;
    border-bottom: 1px solid color-mix(in srgb, var(--ink) 28%, transparent);
    background: transparent;
    padding: 6px 0;
    font: inherit;
    outline: none;
  }
  .acts {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 18px;
  }
  .acts button[type="submit"] {
    border-bottom: 2px solid var(--amber);
  }
  @keyframes fade {
    from { opacity: 0; }
  }
  @keyframes pop {
    from { opacity: 0; transform: translate(-50%, -46%); }
  }
</style>
