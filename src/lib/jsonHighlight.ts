/** Custom settings.json highlighter — not a stock theme pack. */

function esc(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

function span(kind: string, s: string): string {
  return `<span class="t-${kind}">${esc(s)}</span>`;
}

function readString(src: string, i: number): { end: number; raw: string } | null {
  if (src[i] !== '"') return null;
  let j = i + 1;
  while (j < src.length) {
    const c = src[j];
    if (c === "\\") {
      j += 2;
      continue;
    }
    if (c === '"') return { end: j + 1, raw: src.slice(i, j + 1) };
    j += 1;
  }
  return { end: src.length, raw: src.slice(i) };
}

function isKeyString(src: string, after: number): boolean {
  let k = after;
  while (k < src.length && /\s/.test(src[k]!)) k += 1;
  return src[k] === ":";
}

/**
 * Tokenize JSON-ish text into colored spans.
 * Broken JSON still gets best-effort coloring so editing stays readable.
 */
export function highlightJson(src: string): string {
  if (!src) return "";
  let out = "";
  let i = 0;
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
      const s = readString(src, i)!;
      const kind = s.raw.endsWith('"') && isKeyString(src, s.end) ? "key" : "str";
      out += span(kind, s.raw);
      i = s.end;
      continue;
    }

    if (c === "-" || (c >= "0" && c <= "9")) {
      const m = src.slice(i).match(/^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?/);
      if (m) {
        out += span("num", m[0]);
        i += m[0].length;
        continue;
      }
    }

    if (src.startsWith("true", i) && !/\w/.test(src[i + 4] ?? "")) {
      out += span("kw", "true");
      i += 4;
      continue;
    }
    if (src.startsWith("false", i) && !/\w/.test(src[i + 5] ?? "")) {
      out += span("kw", "false");
      i += 5;
      continue;
    }
    if (src.startsWith("null", i) && !/\w/.test(src[i + 4] ?? "")) {
      out += span("kw", "null");
      i += 4;
      continue;
    }

    if ("{}[]:,".includes(c)) {
      out += span("p", c);
      i += 1;
      continue;
    }

    out += span("x", c);
    i += 1;
  }
  return out;
}
