/** Semantic highlighter for Claude Code settings.json — custom, not a stock theme. */

function esc(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function span(kind: string, s: string): string {
  return `<span class="t-${kind}">${esc(s)}</span>`;
}

function readString(src: string, i: number): { end: number; raw: string; text: string } {
  let j = i + 1;
  while (j < src.length) {
    const c = src[j];
    if (c === "\\") {
      j += 2;
      continue;
    }
    if (c === '"') {
      const raw = src.slice(i, j + 1);
      return { end: j + 1, raw, text: raw.slice(1, -1) };
    }
    j += 1;
  }
  const raw = src.slice(i);
  return { end: src.length, raw, text: raw.slice(1) };
}

function isKeyString(src: string, after: number): boolean {
  let k = after;
  while (k < src.length && /\s/.test(src[k]!)) k += 1;
  return src[k] === ":";
}

type Frame = { kind: "object" | "array"; key: string | null };

/** Semantic role for a field name under the current parent path. */
export function fieldRole(name: string, parentKey: string | null): string {
  if (parentKey === "env") {
    if (name === "ANTHROPIC_BASE_URL") return "url";
    if (name === "ANTHROPIC_AUTH_TOKEN") return "secret";
    if (name === "ANTHROPIC_MODEL") return "model";
    if (name === "API_TIMEOUT_MS") return "timeout";
    if (
      name === "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC" ||
      name === "CLAUDE_CODE_ATTRIBUTION_HEADER"
    ) {
      return "flag";
    }
    return "env";
  }
  if (parentKey === "enabledPlugins" || parentKey === "extraKnownMarketplaces") {
    return "plugin";
  }
  if (name === "enabledPlugins" || name === "extraKnownMarketplaces") return "plugin";
  if (name === "env") return "env";
  if (name === "theme") return "theme";
  if (name === "statusLine") return "meta";
  return "plain";
}

/**
 * Path-aware tokenizer: colors Base URL / API Key / plugins differently.
 * Broken JSON still gets best-effort coloring.
 */
export function highlightJson(src: string): string {
  if (!src) return "";
  let out = "";
  let i = 0;
  const stack: Frame[] = [];
  let pendingKey: string | null = null;
  let expectKey = true;

  const parentKey = () => stack[stack.length - 1]?.key ?? null;

  while (i < src.length) {
    const c = src[i]!;

    if (/\s/.test(c)) {
      let j = i + 1;
      while (j < src.length && /\s/.test(src[j]!)) j += 1;
      out += esc(src.slice(i, j));
      i = j;
      continue;
    }

    if (c === '"') {
      const s = readString(src, i);
      const asKey = s.raw.endsWith('"') && isKeyString(src, s.end);
      const top = stack[stack.length - 1];

      if (asKey && (!top || top.kind === "object") && expectKey) {
        const role = fieldRole(s.text, parentKey());
        out += span(`k-${role}`, s.raw);
        pendingKey = s.text;
        i = s.end;
        continue;
      }

      const role = fieldRole(pendingKey ?? "", parentKey());
      out += span(`v-${role}`, s.raw);
      pendingKey = null;
      expectKey = true;
      i = s.end;
      continue;
    }

    if (c === "-" || (c >= "0" && c <= "9")) {
      const m = src.slice(i).match(/^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?/);
      if (m) {
        const role = fieldRole(pendingKey ?? "", parentKey());
        out += span(role === "timeout" || role === "plain" ? "num" : `v-${role}`, m[0]);
        pendingKey = null;
        expectKey = true;
        i += m[0].length;
        continue;
      }
    }

    if (src.startsWith("true", i) && !/\w/.test(src[i + 4] ?? "")) {
      const role = fieldRole(pendingKey ?? "", parentKey());
      out += span(role === "plugin" ? "v-plugin" : "kw", "true");
      pendingKey = null;
      expectKey = true;
      i += 4;
      continue;
    }
    if (src.startsWith("false", i) && !/\w/.test(src[i + 5] ?? "")) {
      const role = fieldRole(pendingKey ?? "", parentKey());
      out += span(role === "plugin" ? "v-plugin" : "kw", "false");
      pendingKey = null;
      expectKey = true;
      i += 5;
      continue;
    }
    if (src.startsWith("null", i) && !/\w/.test(src[i + 4] ?? "")) {
      out += span("kw", "null");
      pendingKey = null;
      expectKey = true;
      i += 4;
      continue;
    }

    if (c === "{") {
      stack.push({ kind: "object", key: pendingKey });
      pendingKey = null;
      expectKey = true;
      out += span("p", c);
      i += 1;
      continue;
    }
    if (c === "[") {
      stack.push({ kind: "array", key: pendingKey });
      pendingKey = null;
      expectKey = false;
      out += span("p", c);
      i += 1;
      continue;
    }
    if (c === "}" || c === "]") {
      stack.pop();
      pendingKey = null;
      expectKey = true;
      out += span("p", c);
      i += 1;
      continue;
    }
    if (c === ":") {
      expectKey = false;
      out += span("p", c);
      i += 1;
      continue;
    }
    if (c === ",") {
      const top = stack[stack.length - 1];
      expectKey = !top || top.kind === "object";
      pendingKey = null;
      out += span("p", c);
      i += 1;
      continue;
    }

    out += span("x", c);
    i += 1;
  }
  return out;
}
