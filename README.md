# Claude Code Settings Manager

Windows 桌面端，编辑 Claude Code 的 `settings.json`。配置和 API Key 只留在本地；联网仅用于检查/下载应用更新。

```
npm install
npm run tauri dev
```

本地打签名安装包（需本机已有私钥 `%USERPROFILE%\.tauri\claude-settings-manager.key`）：

```
npm run tauri:build
```

## 发新版

1. 同步改 `package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json` 的版本号
2. 提交并打 tag：`git tag v0.1.2 && git push origin v0.1.2`
3. GitHub Actions 会构建 NSIS 并上传 `latest.json`
4. 已安装用户下次启动或点「检查更新」即可升级

仓库需公开（`yuese12333/claude-settings-manager`），否则更新器拉不到 `latest.json`。

GitHub Secrets：

- `TAURI_SIGNING_PRIVATE_KEY`：私钥文件全文
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`：没有密码就留空

私钥丢失则无法给旧用户推送后续更新。不要提交 `*.key`。
