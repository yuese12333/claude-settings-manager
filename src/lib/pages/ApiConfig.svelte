<script lang="ts">
  import ApiKeyInput from "$lib/components/ApiKeyInput.svelte";
  import Profiles from "$lib/components/Profiles.svelte";
  import { MODELS } from "$lib/types";
  import type { Profile, Settings } from "$lib/types";

  let { settings = $bindable() }: { settings: Settings } = $props();

  function env(): Record<string, string> {
    if (!settings.env) settings.env = {};
    return settings.env;
  }

  function get(key: string) {
    return settings.env?.[key] ?? "";
  }

  function setKey(key: string, value: string) {
    const e = env();
    if (value.trim() === "") delete e[key];
    else e[key] = value.trim();
    settings = settings;
  }

  function modelChoice() {
    const m = get("ANTHROPIC_MODEL");
    return MODELS.includes(m as (typeof MODELS)[number]) ? m : m ? "custom" : "";
  }

  function timeoutMinutes() {
    const ms = Number(get("API_TIMEOUT_MS"));
    return Number.isFinite(ms) && ms > 0 ? ms / 60000 : 0;
  }

  function applyProfile(p: Profile) {
    setKey("ANTHROPIC_BASE_URL", p.baseUrl);
    setKey("ANTHROPIC_AUTH_TOKEN", p.apiKey);
  }
</script>

<section>
  <h2>接口配置</h2>
  <p class="lead">编辑当前连接参数，或从下方配置组一键应用预设。应用后需保存才会写入磁盘。</p>

  <label for="base-url">Base URL</label>
  <input
    id="base-url"
    type="url"
    placeholder="https://api.anthropic.com"
    value={get("ANTHROPIC_BASE_URL")}
    oninput={(e) => setKey("ANTHROPIC_BASE_URL", e.currentTarget.value)}
  />

  <label for="api-key">API Key</label>
  <ApiKeyInput value={get("ANTHROPIC_AUTH_TOKEN")} onchange={(v) => setKey("ANTHROPIC_AUTH_TOKEN", v)} />

  <label for="model">Model</label>
  <div class="model">
    <select id="model" value={modelChoice()} onchange={(e) => {
      const v = e.currentTarget.value;
      if (v === "custom") {
        if (MODELS.includes(get("ANTHROPIC_MODEL") as (typeof MODELS)[number]) || !get("ANTHROPIC_MODEL")) {
          setKey("ANTHROPIC_MODEL", "");
        }
      } else {
        setKey("ANTHROPIC_MODEL", v);
      }
    }}>
      <option value="">选择模型…</option>
      {#each MODELS as m}
        <option value={m}>{m}</option>
      {/each}
      <option value="custom">自定义型号</option>
    </select>
    {#if modelChoice() === "custom"}
      <input
        type="text"
        placeholder="输入模型标识符"
        value={get("ANTHROPIC_MODEL")}
        oninput={(e) => setKey("ANTHROPIC_MODEL", e.currentTarget.value)}
      />
    {/if}
  </div>

  <label for="timeout">Timeout</label>
  <div class="timeout">
    <input
      id="timeout"
      type="number"
      min="0"
      step="1"
      value={timeoutMinutes() || ""}
      oninput={(e) => {
        const n = Number(e.currentTarget.value);
        if (!e.currentTarget.value || !Number.isFinite(n) || n <= 0) setKey("API_TIMEOUT_MS", "");
        else setKey("API_TIMEOUT_MS", String(Math.round(n * 60000)));
      }}
    />
    <span>分钟</span>
  </div>

  <Profiles
    currentUrl={get("ANTHROPIC_BASE_URL")}
    currentKey={get("ANTHROPIC_AUTH_TOKEN")}
    onapply={applyProfile}
  />
</section>

<style>
  h2 {
    margin: 0;
    font-family: var(--display);
    font-size: 28px;
    font-weight: 600;
    letter-spacing: -0.03em;
  }
  .lead {
    margin: 6px 0 28px;
    color: var(--muted);
    font-size: 14px;
  }
  label {
    display: block;
    margin: 18px 0 4px;
    font-size: 12px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--muted);
  }
  input,
  select {
    width: 100%;
    border: 0;
    border-bottom: 1px solid color-mix(in srgb, var(--ink) 28%, transparent);
    background: transparent;
    padding: 8px 0;
    font: inherit;
    font-size: 15px;
    color: var(--ink);
    outline: none;
    border-radius: 0;
  }
  input:focus,
  select:focus {
    border-bottom-color: var(--pine);
  }
  .model,
  .timeout {
    display: flex;
    align-items: baseline;
    gap: 16px;
  }
  .timeout input {
    width: 96px;
  }
  .timeout span {
    color: var(--muted);
    font-size: 13px;
  }
</style>
