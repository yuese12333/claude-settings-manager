<script lang="ts">
  let {
    value,
    onchange,
    id = "api-key",
  }: { value: string; onchange: (v: string) => void; id?: string } = $props();
  let shown = $state(false);
</script>

<div class="wrap">
  <input
    {id}
    type={shown ? "text" : "password"}
    autocomplete="off"
    spellcheck="false"
    {value}
    oninput={(e) => onchange(e.currentTarget.value)}
  />
  <button type="button" aria-label={shown ? "隐藏密钥" : "显示密钥"} onclick={() => (shown = !shown)}>
    {shown ? "隐藏" : "显示"}
  </button>
</div>

<style>
  .wrap {
    display: flex;
    align-items: stretch;
    gap: 8px;
  }
  input {
    flex: 1;
    min-width: 0;
    border: 0;
    border-bottom: 1px solid color-mix(in srgb, var(--ink) 28%, transparent);
    background: transparent;
    padding: 8px 0;
    font: inherit;
    font-family: var(--mono);
    font-size: 14px;
    color: var(--ink);
    outline: none;
  }
  input:focus {
    border-bottom-color: var(--pine);
  }
  button {
    border: 0;
    background: transparent;
    color: var(--muted);
    cursor: pointer;
    font: inherit;
    font-size: 12px;
    padding: 0 4px;
  }
  button:hover {
    color: var(--pine);
  }
</style>
