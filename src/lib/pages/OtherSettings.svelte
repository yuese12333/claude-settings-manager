<script lang="ts">
  import type { Settings } from "$lib/types";

  let { settings = $bindable() }: { settings: Settings } = $props();

  const telemetryOff = $derived(
    (settings.env?.CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC ?? "") === "1",
  );

  function setTheme(v: string) {
    settings.theme = v;
    settings = settings;
  }

  function setTelemetry(off: boolean) {
    if (!settings.env) settings.env = {};
    if (off) settings.env.CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC = "1";
    else delete settings.env.CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC;
    settings = settings;
  }
</script>

<section>
  <h2>偏好</h2>
  <p class="lead">调整 Claude Code 外观主题，以及非必要网络请求（遥测）相关选项。</p>

  <fieldset>
    <legend>外观主题</legend>
    <label>
      <input type="radio" name="theme" checked={settings.theme === "light"} onchange={() => setTheme("light")} />
      浅色
    </label>
    <label>
      <input type="radio" name="theme" checked={settings.theme === "dark"} onchange={() => setTheme("dark")} />
      深色
    </label>
  </fieldset>

  <label class="tog">
    <input type="checkbox" checked={telemetryOff} onchange={(e) => setTelemetry(e.currentTarget.checked)} />
    禁用非必要流量
  </label>
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
  fieldset {
    border: 0;
    padding: 0;
    margin: 0 0 28px;
  }
  legend {
    padding: 0;
    font-size: 12px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--muted);
    margin-bottom: 10px;
  }
  fieldset label {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    margin-right: 20px;
    cursor: pointer;
  }
  .tog {
    display: inline-flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
  }
  input[type="radio"],
  input[type="checkbox"] {
    accent-color: var(--pine);
  }
</style>
