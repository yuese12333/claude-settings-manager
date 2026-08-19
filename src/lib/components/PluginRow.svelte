<script lang="ts">
  let {
    id,
    source,
    enabled,
    ontoggle,
    onremove,
  }: {
    id: string;
    source: string;
    enabled: boolean;
    ontoggle: () => void;
    onremove: () => void;
  } = $props();
</script>

<li>
  <span class="id">{id}</span>
  <span class="src">{source}</span>
  <button class="tog" class:on={enabled} type="button" onclick={ontoggle}>
    {enabled ? "启用" : "禁用"}
  </button>
  <button class="del" type="button" onclick={onremove}>删除</button>
</li>

<style>
  li {
    display: grid;
    grid-template-columns: 1fr 140px auto auto;
    gap: 12px;
    align-items: center;
    padding: 12px 0;
    border-bottom: 1px solid color-mix(in srgb, var(--ink) 10%, transparent);
  }
  .id {
    font-family: var(--mono);
    font-size: 13px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .src {
    color: var(--muted);
    font-size: 13px;
  }
  button {
    font: inherit;
    font-size: 12px;
    cursor: pointer;
    padding: 4px 10px;
  }
  .tog {
    border: 0;
    background: transparent;
    color: var(--muted);
  }
  .tog.on {
    color: var(--pine);
  }
  .del {
    border: 0;
    background: transparent;
    color: var(--muted);
  }
  .del:hover {
    color: #9b2c1a;
  }
</style>
