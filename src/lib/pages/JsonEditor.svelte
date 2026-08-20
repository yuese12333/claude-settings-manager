<script lang="ts">
  import {
    loadSettingsRaw,
    saveSettingsRaw,
    validateSettingsJson,
  } from "$lib/api";
  import { highlightJson } from "$lib/jsonHighlight";
  import type { Settings } from "$lib/types";

  let {
    path,
    onsaved,
    ondirty,
  }: {
    path: string;
    onsaved: (settings: Settings) => void;
    ondirty: (dirty: boolean) => void;
  } = $props();

  let text = $state("");
  let snapshot = $state("");
  let validMsg = $state("正在校验…");
  let ok = $state(false);
  let busy = $state(false);
  let notice = $state("");
  let error = $state("");
  let preEl = $state<HTMLPreElement | null>(null);
  let taEl = $state<HTMLTextAreaElement | null>(null);

  const dirty = $derived(text !== snapshot);
  // trailing newline keeps overlay height in sync when file ends with \n
  const html = $derived(highlightJson(text) + "\n");

  $effect(() => {
    ondirty(dirty);
  });

  async function check(content: string) {
    try {
      await validateSettingsJson(content);
      ok = true;
      validMsg = "结构校验通过";
    } catch (e) {
      ok = false;
      validMsg = String(e);
    }
  }

  function reset() {
    text = snapshot;
    notice = "";
    void check(text);
  }

  async function save() {
    if (!ok || !dirty) return;
    error = "";
    busy = true;
    try {
      const settings = await saveSettingsRaw(path, text);
      snapshot = text.endsWith("\n") ? text : `${text}\n`;
      text = snapshot;
      notice = "已写入磁盘，原文件已备份为 .bak";
      onsaved(settings);
    } catch (e) {
      error = String(e);
      ok = false;
      validMsg = String(e);
    } finally {
      busy = false;
    }
  }

  function syncScroll() {
    if (!preEl || !taEl) return;
    preEl.scrollTop = taEl.scrollTop;
    preEl.scrollLeft = taEl.scrollLeft;
  }

  $effect(() => {
    const p = path;
    error = "";
    notice = "";
    busy = true;
    void loadSettingsRaw(p)
      .then(async (raw) => {
        text = raw;
        snapshot = raw;
        await check(raw);
      })
      .catch((e) => {
        error = String(e);
      })
      .finally(() => {
        busy = false;
      });
  });
</script>

<section>
  <h2>源文件</h2>
  <p class="lead">以 JSON 原文编辑 settings.json。通过结构校验后方可保存。</p>

  <div class="status" class:bad={!ok} class:good={ok}>
    {validMsg}
  </div>

  {#if error}<p class="err">{error}</p>{/if}

  <div class="editor" class:invalid={!ok}>
    <div class="rail" aria-hidden="true"></div>
    <pre class="hl" bind:this={preEl} aria-hidden="true">{@html html}</pre>
    <textarea
      bind:this={taEl}
      spellcheck="false"
      autocomplete="off"
      wrap="off"
      value={text}
      onscroll={syncScroll}
      oninput={(e) => {
        text = e.currentTarget.value;
        notice = "";
        void check(text);
        syncScroll();
      }}
    ></textarea>
  </div>

  <footer>
    {#if notice}<span class="ok">{notice}</span>{/if}
    <button type="button" disabled={!dirty || busy} onclick={reset}>还原</button>
    <button class="save" type="button" disabled={!dirty || !ok || busy} onclick={save}>保存更改</button>
  </footer>
</section>

<style>
  section {
    display: flex;
    flex-direction: column;
    min-height: 100%;
    flex: 1;
  }
  h2 {
    margin: 0;
    font-family: var(--display);
    font-size: 28px;
    font-weight: 600;
    letter-spacing: -0.03em;
  }
  .lead {
    margin: 6px 0 16px;
    color: var(--muted);
    font-size: 14px;
  }
  .status {
    font-size: 13px;
    margin-bottom: 10px;
    font-family: var(--mono);
  }
  .status.good {
    color: var(--pine);
  }
  .status.bad {
    color: #9b2c1a;
  }
  .err {
    color: #9b2c1a;
    margin: 0 0 8px;
    font-size: 13px;
  }

  /* ink-ledger editor: paper field + margin rail, not a stock IDE theme */
  .editor {
    --ed-pad-y: 12px;
    --ed-pad-x: 14px;
    --ed-rail: 10px;
    position: relative;
    flex: 1;
    min-height: 320px;
    border: 1px solid color-mix(in srgb, var(--ink) 16%, transparent);
    background:
      linear-gradient(
        90deg,
        color-mix(in srgb, var(--pine) 7%, transparent) 0,
        color-mix(in srgb, var(--pine) 7%, transparent) var(--ed-rail),
        transparent var(--ed-rail)
      ),
      color-mix(in srgb, #f4f7f2 88%, white);
    overflow: hidden;
  }
  .editor.invalid {
    border-color: color-mix(in srgb, #9b2c1a 45%, transparent);
  }
  .rail {
    position: absolute;
    top: 0;
    bottom: 0;
    left: calc(var(--ed-rail) - 1px);
    width: 1px;
    background: color-mix(in srgb, var(--amber) 55%, transparent);
    pointer-events: none;
    z-index: 2;
  }
  .hl,
  textarea {
    margin: 0;
    position: absolute;
    inset: 0;
    box-sizing: border-box;
    padding: var(--ed-pad-y) var(--ed-pad-x) var(--ed-pad-y) calc(var(--ed-pad-x) + var(--ed-rail));
    border: 0;
    font-family: var(--mono);
    font-size: 13px;
    line-height: 1.5;
    letter-spacing: 0.01em;
    white-space: pre;
    overflow: auto;
    tab-size: 2;
  }
  .hl {
    color: color-mix(in srgb, var(--ink) 42%, transparent);
    pointer-events: none;
    z-index: 0;
  }
  textarea {
    z-index: 1;
    resize: none;
    background: transparent;
    color: transparent;
    caret-color: var(--pine);
    outline: none;
  }
  textarea::selection {
    background: color-mix(in srgb, var(--amber) 35%, transparent);
    color: transparent;
  }

  /* semantic palette: url / secret / model / plugin / … */
  .hl :global(.t-k-url),
  .hl :global(.t-v-url) {
    color: #0f5c8a;
  }
  .hl :global(.t-k-url) {
    font-weight: 700;
  }
  .hl :global(.t-v-url) {
    text-decoration: underline;
    text-decoration-color: color-mix(in srgb, #0f5c8a 40%, transparent);
    text-underline-offset: 2px;
  }

  .hl :global(.t-k-secret),
  .hl :global(.t-v-secret) {
    color: #8a2f45;
  }
  .hl :global(.t-k-secret) {
    font-weight: 700;
  }
  .hl :global(.t-v-secret) {
    background: color-mix(in srgb, #8a2f45 9%, transparent);
  }

  .hl :global(.t-k-model),
  .hl :global(.t-v-model) {
    color: #9a5b12;
  }
  .hl :global(.t-k-model) {
    font-weight: 700;
  }

  .hl :global(.t-k-timeout),
  .hl :global(.t-v-timeout) {
    color: #6a4a18;
  }
  .hl :global(.t-k-timeout) {
    font-weight: 700;
  }

  .hl :global(.t-k-plugin),
  .hl :global(.t-v-plugin) {
    color: #3a5a28;
  }
  .hl :global(.t-k-plugin) {
    font-weight: 700;
  }
  .hl :global(.t-v-plugin) {
    font-style: italic;
  }

  .hl :global(.t-k-theme),
  .hl :global(.t-v-theme) {
    color: #5a3d6e;
  }
  .hl :global(.t-k-theme) {
    font-weight: 700;
  }

  .hl :global(.t-k-env),
  .hl :global(.t-v-env),
  .hl :global(.t-k-flag),
  .hl :global(.t-v-flag) {
    color: #1a4a40;
  }
  .hl :global(.t-k-env),
  .hl :global(.t-k-flag) {
    font-weight: 600;
  }

  .hl :global(.t-k-meta),
  .hl :global(.t-v-meta) {
    color: #4a5a62;
  }
  .hl :global(.t-k-meta) {
    font-weight: 600;
  }

  .hl :global(.t-k-plain) {
    color: #1a4a40;
    font-weight: 600;
  }
  .hl :global(.t-v-plain) {
    color: #6b5a32;
  }

  .hl :global(.t-num) {
    color: #b56a1a;
  }
  .hl :global(.t-kw) {
    color: #2f6b5c;
    text-decoration: underline;
    text-decoration-color: color-mix(in srgb, var(--amber) 70%, transparent);
    text-underline-offset: 3px;
  }
  .hl :global(.t-p) {
    color: #7a8a82;
  }
  .hl :global(.t-x) {
    color: #9b2c1a;
    background: color-mix(in srgb, #9b2c1a 8%, transparent);
  }

  footer {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 16px;
    padding-top: 16px;
  }
  footer button {
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
  .ok {
    margin-right: auto;
    color: var(--pine);
    font-size: 13px;
  }
</style>
