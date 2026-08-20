import { highlightJson } from "./jsonHighlight";

function assert(cond: boolean, msg: string) {
  if (!cond) throw new Error(msg);
}

const sample = `{
  "env": {
    "ANTHROPIC_MODEL": "claude-sonnet-4-6",
    "API_TIMEOUT_MS": "600000"
  },
  "theme": "light",
  "ok": true,
  "n": null,
  "count": 3
}`;

const html = highlightJson(sample);
assert(html.includes("t-key"), "keys highlighted");
assert(html.includes("t-str"), "strings highlighted");
assert(html.includes("t-num"), "numbers highlighted");
assert(html.includes("t-kw"), "keywords highlighted");
assert(!html.includes("<script"), "escaped");
assert(highlightJson("") === "", "empty");

console.log("jsonHighlight ok");
