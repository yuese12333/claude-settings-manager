import { fieldRole, highlightJson } from "./jsonHighlight";

function assert(cond: boolean, msg: string) {
  if (!cond) throw new Error(msg);
}

assert(fieldRole("ANTHROPIC_BASE_URL", "env") === "url", "url role");
assert(fieldRole("ANTHROPIC_AUTH_TOKEN", "env") === "secret", "secret role");
assert(fieldRole("caveman@caveman", "enabledPlugins") === "plugin", "plugin role");
assert(fieldRole("theme", null) === "theme", "theme role");

const sample = `{
  "env": {
    "ANTHROPIC_BASE_URL": "https://api.example.com",
    "ANTHROPIC_AUTH_TOKEN": "sk-test",
    "ANTHROPIC_MODEL": "claude-sonnet-4-6",
    "API_TIMEOUT_MS": "600000"
  },
  "enabledPlugins": { "caveman@caveman": true },
  "theme": "light",
  "count": 3
}`;

const html = highlightJson(sample);
assert(html.includes("t-k-url") && html.includes("t-v-url"), "base url colors");
assert(html.includes("t-k-secret") && html.includes("t-v-secret"), "api key colors");
assert(html.includes("t-k-model") && html.includes("t-v-model"), "model colors");
assert(html.includes("t-k-plugin") && html.includes("t-v-plugin"), "plugin colors");
assert(html.includes("t-k-theme") && html.includes("t-v-theme"), "theme colors");
assert(html.includes("t-num"), "numbers");
assert(!html.includes("<script"), "escaped");
assert(highlightJson("") === "", "empty");

console.log("jsonHighlight ok");
