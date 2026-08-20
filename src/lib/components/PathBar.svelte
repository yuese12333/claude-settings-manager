<script lang="ts">
  let {
    path,
    sibling,
    onpick,
    onswitch,
  }: {
    path: string | null;
    sibling: string | null;
    onpick: () => void;
    onswitch: (p: string) => void;
  } = $props();

  function nameOf(p: string) {
    return p.replace(/^.*[\\/]/, "");
  }
</script>

<header class="bar">
  <div class="brand">
    <span class="mark">CC</span>
    <div>
      <h1>Claude Code Settings</h1>
      <p>本地管理 Claude Code 配置</p>
    </div>
  </div>

  <div class="file">
    {#if path && sibling}
      <div class="tabs">
        <button class="tab on" type="button">{nameOf(path)}</button>
        <button class="tab" type="button" onclick={() => onswitch(sibling)}>{nameOf(sibling)}</button>
      </div>
    {/if}
    <p class="path" title={path ?? ""}>{path ?? "尚未定位到配置文件"}</p>
    <button class="ghost" type="button" onclick={onpick}>选择文件</button>
  </div>
</header>

<style>
  .bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 24px;
    padding: 18px 28px 16px;
    border-bottom: 1px solid color-mix(in srgb, var(--ink) 12%, transparent);
  }
  .brand {
    display: flex;
    align-items: center;
    gap: 14px;
    min-width: 240px;
  }
  .mark {
    width: 44px;
    height: 44px;
    display: grid;
    place-items: center;
    background: var(--pine);
    color: #f3ebe0;
    font-family: var(--display);
    font-size: 18px;
    letter-spacing: 0.04em;
  }
  h1 {
    margin: 0;
    font-family: var(--display);
    font-size: 22px;
    font-weight: 600;
    letter-spacing: -0.02em;
    line-height: 1.1;
  }
  .brand p {
    margin: 2px 0 0;
    font-size: 12px;
    color: var(--muted);
  }
  .file {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
    flex: 1;
    justify-content: flex-end;
  }
  .path {
    margin: 0;
    max-width: 380px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 12px;
    color: var(--muted);
    font-family: var(--mono);
  }
  .tabs {
    display: flex;
    gap: 2px;
  }
  .tab {
    border: 0;
    background: transparent;
    color: var(--muted);
    padding: 4px 8px;
    font: inherit;
    font-size: 12px;
    cursor: pointer;
    border-bottom: 2px solid transparent;
  }
  .tab.on {
    color: var(--ink);
    border-bottom-color: var(--amber);
  }
  .ghost {
    border: 1px solid color-mix(in srgb, var(--ink) 22%, transparent);
    background: transparent;
    color: var(--ink);
    padding: 6px 12px;
    font: inherit;
    font-size: 13px;
    cursor: pointer;
  }
  .ghost:hover {
    border-color: var(--pine);
    color: var(--pine);
  }
</style>
