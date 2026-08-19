import { spawn } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";

const key = join(homedir(), ".tauri", "claude-settings-manager.key");
if (!existsSync(key)) {
  console.error(`缺少更新签名私钥: ${key}`);
  process.exit(1);
}

const child = spawn("npx", ["tauri", "build", ...process.argv.slice(2)], {
  stdio: "inherit",
  shell: true,
  env: {
    ...process.env,
    TAURI_SIGNING_PRIVATE_KEY: readFileSync(key, "utf8").trim(),
    TAURI_SIGNING_PRIVATE_KEY_PASSWORD: process.env.TAURI_SIGNING_PRIVATE_KEY_PASSWORD ?? "",
  },
});
child.on("exit", (code) => process.exit(code ?? 1));
