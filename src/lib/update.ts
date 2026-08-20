import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

// ponytail: updater is the only network path; never attach settings/API keys to check()
export async function checkAppUpdate(onProgress: (msg: string) => void): Promise<string> {
  const update = await check({ timeout: 10_000 });
  if (!update) return "已是最新版本";

  const notes = update.body?.trim() ? `\n\n${update.body.trim()}` : "";
  if (!confirm(`发现新版本 ${update.version}。是否立即下载并安装？${notes}`)) {
    return `发现可用更新 ${update.version}（已跳过）`;
  }

  let downloaded = 0;
  let total = 0;
  await update.downloadAndInstall((event) => {
    if (event.event === "Started") total = event.data.contentLength ?? 0;
    if (event.event === "Progress") {
      downloaded += event.data.chunkLength;
      onProgress(total ? `正在下载 ${Math.round((downloaded / total) * 100)}%` : "正在下载…");
    }
  });
  onProgress("安装完成，即将重启…");
  await relaunch();
  return "更新已完成";
}
