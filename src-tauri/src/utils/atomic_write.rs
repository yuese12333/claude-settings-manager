use std::fs;
use std::path::Path;

/// Write `contents` to `path` via tmp + backup + rename.
/// On Windows, rename cannot replace an existing file.
pub fn atomic_write(path: &Path, contents: &str) -> Result<(), String> {
    let name = path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("无效文件名")?;
    let tmp = path.with_file_name(format!("{name}.tmp"));
    let bak = path.with_file_name(format!("{name}.bak"));

    fs::write(&tmp, contents).map_err(|e| format!("写入临时文件失败: {e}"))?;

    if path.exists() {
        let _ = fs::remove_file(&bak);
        fs::rename(path, &bak).map_err(|e| {
            let _ = fs::remove_file(&tmp);
            format!("备份原文件失败: {e}")
        })?;
    }

    fs::rename(&tmp, path).map_err(|e| {
        // best-effort restore
        if bak.exists() && !path.exists() {
            let _ = fs::rename(&bak, path);
        }
        let _ = fs::remove_file(&tmp);
        format!("替换文件失败: {e}")
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn scratch() -> std::path::PathBuf {
        let n = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("csm-aw-{n}"));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn backup_then_replace() {
        let dir = scratch();
        let path = dir.join("settings.json");
        fs::write(&path, "old").unwrap();
        atomic_write(&path, "new").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "new");
        assert_eq!(
            fs::read_to_string(dir.join("settings.json.bak")).unwrap(),
            "old"
        );
        assert!(!dir.join("settings.json.tmp").exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn writes_when_missing() {
        let dir = scratch();
        let path = dir.join("settings.json");
        atomic_write(&path, "{}").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "{}");
        let _ = fs::remove_dir_all(&dir);
    }
}
