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
    onsaved: (settings: Settings | null) => void;
    ondirty: (dirty: boolean) => void;
  } = $props();

  let text = $state("");
  let snapshot = $state("");
  let validMsg = $state("正在校验…");
  let ok = $state(false);
  let errLine = $state<number | null>(null);
  let busy = $state(false);
  let notice = $state("");
  let error = $state("");
  let preEl = $state<HTMLPreElement | null>(null);
  let taEl = $state<HTMLTextAreaElement | null>(null);
  let gutterEl = $state<HTMLDivElement | null>(null);

  let findOpen = $state(false);
  let findQ = $state("");
  let replaceQ = $state("");
  let findHint = $state("");

  const dirty = $derived(text !== snapshot);
  const lineCount = $derived(Math.max(1, text.split("\n").length));
  const lineNos = $derived(Array.from({ length: lineCount }, (_, i) => i + 1));
  const html = $derived(highlightJson(text) + "\n");

  $effect(() => {
    ondirty(dirty);
  });

  function syncScroll() {
    if (!preEl || !taEl) return;
    preEl.scrollTop = taEl.scrollTop;
    preEl.scrollLeft = taEl.scrollLeft;
    if (gutterEl) gutterEl.scrollTop = taEl.scrollTop;
  }

  function goToLine(line: number) {
    if (!taEl || line < 1) return;
    const parts = text.split("\n");
    let pos = 0;
    const target = Math.min(line, parts.length);
    for (let i = 0; i < target - 1; i++) pos += parts[i].length + 1;
    const end = pos + (parts[target - 1]?.length ?? 0);
    taEl.focus();
    taEl.setSelectionRange(pos, end);
    const lh = parseFloat(getComputedStyle(taEl).lineHeight) || 19.5;
    taEl.scrollTop = Math.max(0, (target - 1) * lh - taEl.clientHeight / 3);
    syncScroll();
  }

  async function check(content: string) {
    try {
      const v = await validateSettingsJson(content);
      ok = v.ok;
      validMsg = v.message;
      errLine = v.ok ? null : (v.line ?? null);
    } catch (e) {
      ok = false;
      validMsg = String(e);
      errLine = null;
    }
  }

  function reset() {
    text = snapshot;
    notice = "";
    void check(text);
  }

  async function save() {
    if (!dirty) return;
    error = "";
    busy = true;
    try {
      const out = await saveSettingsRaw(path, text);
      snapshot = text.endsWith("\n") ? text : `${text}\n`;
      text = snapshot;
      notice = ok
        ? "已写入磁盘，原文件已备份为 .bak"
        : "已写入磁盘（当前内容校验未通过，Claude Code 可能无法正常使用）";
      onsaved(out.settings);
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }

  function findFrom(start: number, backward = false): boolean {
    const q = findQ;
    if (!q || !taEl) {
      findHint = q ? "" : "请输入查找内容";
      return false;
    }
    const hay = text;
    let idx = -1;
    if (backward) {
      idx = hay.lastIndexOf(q, Math.max(0, start - 1));
    } else {
      idx = hay.indexOf(q, start);
      if (idx < 0 && start > 0) idx = hay.indexOf(q, 0);
    }
    if (idx < 0) {
      findHint = "未找到";
      return false;
    }
    taEl.focus();
    taEl.setSelectionRange(idx, idx + q.length);
    const before = hay.slice(0, idx);
    const line = before.split("\n").length;
    goToLine(line);
    taEl.setSelectionRange(idx, idx + q.length);
    findHint = `第 ${line} 行`;
    return true;
  }

  function findNext() {
    const start = taEl ? taEl.selectionEnd : 0;
    findFrom(start, false);
  }

  function findPrev() {
    const start = taEl ? taEl.selectionStart : 0;
    findFrom(start, true);
  }

  function replaceOne() {
    if (!taEl || !findQ) return;
    const { selectionStart: a, selectionEnd: b } = taEl;
    if (text.slice(a, b) === findQ) {
      text = text.slice(0, a) + replaceQ + text.slice(b);
      notice = "";
      void check(text);
      queueMicrotask(() => {
        if (!taEl) return;
        const pos = a + replaceQ.length;
        taEl.setSelectionRange(pos, pos);
        findFrom(pos, false);
      });
    } else {
      findNext();
    }
  }

  function replaceAll() {
    if (!findQ) return;
    const n = text.split(findQ).length - 1;
    if (n <= 0) {
      findHint = "未找到";
      return;
    }
    text = text.split(findQ).join(replaceQ);
    findHint = `已替换 ${n} 处`;
    notice = "";
    void check(text);
  }

  function onKey(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === "f") {
      e.preventDefault();
      findOpen = true;
      queueMicrotask(() => document.getElementById("find-q")?.focus());
    }
    if (e.key === "Escape" && findOpen) {
      findOpen = false;
      taEl?.focus();
    }
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

<svelte:window onkeydown={onKey} />

<section>
  <h2>源文件</h2>
  <p class="lead">编辑 Claude Code 的 settings.json。字段问题仅作提示，不阻止保存；格式有误时保存后可能无法正常使用。</p>

  <div class="toolbar">
    <button type="button" onclick={() => (findOpen = !findOpen)}>查找/替换</button>
    {#if errLine}
      <button type="button" class="jump" onclick={() => goToLine(errLine!)}>定位到第 {errLine} 行</button>
    {/if}
  </div>

  {#if findOpen}
    <div class="find">
      <input id="find-q" placeholder="查找" bind:value={findQ} onkeydown={(e) => e.key === "Enter" && (e.shiftKey ? findPrev() : findNext())} />
      <input placeholder="替换为" bind:value={replaceQ} onkeydown={(e) => e.key === "Enter" && replaceOne()} />
      <button type="button" onclick={findPrev}>上一个</button>
      <button type="button" onclick={findNext}>下一个</button>
      <button type="button" onclick={replaceOne}>替换</button>
      <button type="button" onclick={replaceAll}>全部替换</button>
      {#if findHint}<span class="hint">{findHint}</span>{/if}
    </div>
  {/if}

  <div class="status" class:bad={!ok} class:good={ok}>
    {validMsg}
  </div>

  {#if error}<p class="err">{error}</p>{/if}

  <div class="editor" class:invalid={!ok}>
    <div class="gutter" bind:this={gutterEl} aria-hidden="true">
      {#each lineNos as n}
        <span class:hot={errLine === n}>{n}</span>
      {/each}
    </div>
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
    {#if notice}<span class:ok={ok} class:warn={!ok}>{notice}</span>{/if}
    <button type="button" disabled={!dirty || busy} onclick={reset}>还原</button>
    <button class="save" type="button" disabled={!dirty || busy} onclick={save}>保存更改</button>
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
    margin: 6px 0 12px;
    color: var(--muted);
    font-size: 14px;
  }
  .toolbar {
    display: flex;
    gap: 16px;
    margin-bottom: 8px;
  }
  .toolbar button,
  .find button {
    border: 0;
    background: transparent;
    font: inherit;
    cursor: pointer;
    padding: 2px 0;
    color: var(--ink);
    border-bottom: 2px solid var(--amber);
  }
  .jump {
    color: #9b2c1a;
    border-bottom-color: color-mix(in srgb, #9b2c1a 50%, transparent);
  }
  .find {
    display: flex;
    flex-wrap: wrap;
    gap: 8px 12px;
    align-items: center;
    margin-bottom: 10px;
    font-size: 13px;
  }
  .find input {
    width: 160px;
    border: 0;
    border-bottom: 1px solid color-mix(in srgb, var(--ink) 28%, transparent);
    background: transparent;
    padding: 4px 0;
    font: inherit;
    outline: none;
  }
  .hint {
    color: var(--muted);
    font-family: var(--mono);
    font-size: 12px;
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

  .editor {
    --ed-pad-y: 12px;
    --ed-pad-x: 14px;
    --ed-gutter: 44px;
    --ed-rail: 10px;
    position: relative;
    flex: 1;
    min-height: 320px;
    border: 1px solid color-mix(in srgb, var(--ink) 16%, transparent);
    background:
      linear-gradient(
        90deg,
        color-mix(in srgb, var(--pine) 10%, transparent) 0,
        color-mix(in srgb, var(--pine) 10%, transparent) var(--ed-gutter),
        color-mix(in srgb, var(--pine) 5%, transparent) var(--ed-gutter),
        color-mix(in srgb, var(--pine) 5%, transparent) calc(var(--ed-gutter) + var(--ed-rail)),
        transparent calc(var(--ed-gutter) + var(--ed-rail))
      ),
      color-mix(in srgb, #f4f7f2 88%, white);
    overflow: hidden;
  }
  .editor.invalid {
    border-color: color-mix(in srgb, #9b2c1a 45%, transparent);
  }
  .gutter {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    width: var(--ed-gutter);
    padding: var(--ed-pad-y) 8px var(--ed-pad-y) 0;
    box-sizing: border-box;
    overflow: hidden;
    text-align: right;
    font-family: var(--mono);
    font-size: 13px;
    line-height: 1.5;
    color: color-mix(in srgb, var(--ink) 38%, transparent);
    user-select: none;
    z-index: 2;
    pointer-events: none;
  }
  .gutter span {
    display: block;
  }
  .gutter span.hot {
    color: #9b2c1a;
    font-weight: 700;
  }
  .rail {
    position: absolute;
    top: 0;
    bottom: 0;
    left: calc(var(--ed-gutter) + var(--ed-rail) - 1px);
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
    padding: var(--ed-pad-y) var(--ed-pad-x) var(--ed-pad-y)
      calc(var(--ed-pad-x) + var(--ed-gutter) + var(--ed-rail));
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
  .warn {
    margin-right: auto;
    color: #9b2c1a;
    font-size: 13px;
  }
</style>
