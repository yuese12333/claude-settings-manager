<script lang="ts">
  import PluginRow from "$lib/components/PluginRow.svelte";
  import type { Settings } from "$lib/types";

  let { settings = $bindable() }: { settings: Settings } = $props();
  let open = $state(false);
  let pluginId = $state("");
  let marketplace = $state("");
  let repo = $state("");

  const rows = $derived(
    Object.entries(settings.enabledPlugins ?? {}).map(([id, enabled]) => ({
      id,
      enabled,
      source: id.includes("@") ? id.slice(id.lastIndexOf("@") + 1) : "—",
    })),
  );

  function toggle(id: string) {
    if (!settings.enabledPlugins) settings.enabledPlugins = {};
    settings.enabledPlugins[id] = !settings.enabledPlugins[id];
    settings = settings;
  }

  function remove(id: string) {
    if (!settings.enabledPlugins) return;
    delete settings.enabledPlugins[id];
    const market = id.includes("@") ? id.slice(id.lastIndexOf("@") + 1) : "";
    if (market && settings.extraKnownMarketplaces) {
      delete settings.extraKnownMarketplaces[market];
    }
    settings = settings;
  }

  function add() {
    const id = pluginId.trim();
    const m = marketplace.trim();
    const r = repo.trim();
    if (!id || !m || !r || !r.includes("/")) return;
    if (!settings.enabledPlugins) settings.enabledPlugins = {};
    if (!settings.extraKnownMarketplaces) settings.extraKnownMarketplaces = {};
    settings.enabledPlugins[id] = true;
    settings.extraKnownMarketplaces[m] = { source: { source: "github", repo: r } };
    settings = settings;
    pluginId = "";
    marketplace = "";
    repo = "";
    open = false;
  }
</script>

<section>
  <h2>插件管理</h2>
  <p class="lead">启用、停用或从配置里删掉插件条目。</p>

  <button class="add" type="button" onclick={() => (open = true)}>+ 添加插件</button>

  {#if rows.length === 0}
    <p class="empty">还没有插件条目。</p>
  {:else}
    <ul>
      {#each rows as row (row.id)}
        <PluginRow
          id={row.id}
          source={row.source}
          enabled={row.enabled}
          ontoggle={() => toggle(row.id)}
          onremove={() => remove(row.id)}
        />
      {/each}
    </ul>
  {/if}
</section>

{#if open}
  <div class="mask" onclick={() => (open = false)} role="presentation"></div>
  <form class="dlg" onsubmit={(e) => { e.preventDefault(); add(); }}>
    <h3>添加插件</h3>
    <label for="pid">插件 ID</label>
    <input id="pid" placeholder="caveman@caveman" bind:value={pluginId} />
    <label for="mid">Marketplace</label>
    <input id="mid" placeholder="caveman" bind:value={marketplace} />
    <label for="repo">GitHub Repo</label>
    <input id="repo" placeholder="JuliusBrussee/caveman" bind:value={repo} />
    <div class="acts">
      <button type="button" onclick={() => (open = false)}>取消</button>
      <button type="submit">添加</button>
    </div>
  </form>
{/if}

<style>
  h2 {
    margin: 0;
    font-family: var(--display);
    font-size: 28px;
    font-weight: 600;
    letter-spacing: -0.03em;
  }
  .lead {
    margin: 6px 0 20px;
    color: var(--muted);
    font-size: 14px;
  }
  .add {
    border: 0;
    border-bottom: 2px solid var(--amber);
    background: transparent;
    padding: 4px 0;
    font: inherit;
    cursor: pointer;
    color: var(--ink);
  }
  ul {
    list-style: none;
    margin: 12px 0 0;
    padding: 0;
  }
  .empty {
    color: var(--muted);
    margin-top: 32px;
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
  }
  h3 {
    margin: 0 0 12px;
    font-family: var(--display);
    font-size: 20px;
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
  .acts button {
    border: 0;
    background: transparent;
    font: inherit;
    cursor: pointer;
    padding: 4px 0;
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
