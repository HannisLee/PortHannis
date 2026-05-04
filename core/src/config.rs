use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{ConfigFile, EntryRequest, ForwardingEntry};

/// 配置存储器——管理 JSON 配置文件中的端口转发条目集合。
///
/// 所有写入操作均为原子操作（先写 .tmp，再 rename）。
pub struct ConfigStore {
    path: PathBuf,
    data: ConfigFile,
}

impl ConfigStore {
    /// 从指定路径加载配置。若文件不存在则创建默认空配置。
    pub fn load(path: PathBuf) -> Result<Self> {
        let data = if path.exists() {
            let content = fs::read_to_string(&path).map_err(|e| Error::Io(e))?;
            if content.trim().is_empty() {
                ConfigFile {
                    entries: Vec::new(),
                }
            } else {
                serde_json::from_str(&content).map_err(Error::Json)?
            }
        } else {
            let cfg = ConfigFile {
                entries: Vec::new(),
            };
            let parent = path
                .parent()
                .ok_or_else(|| Error::Validation("config path has no parent".into()))?;
            fs::create_dir_all(parent).map_err(|e| Error::Io(e))?;
            let content = serde_json::to_string_pretty(&cfg).map_err(Error::Json)?;
            fs::write(&path, &content).map_err(|e| Error::Io(e))?;
            cfg
        };

        Ok(Self { path, data })
    }

    /// 原子保存：序列化到 .tmp 文件，然后重命名为目标路径。
    pub fn save(&self) -> Result<()> {
        let tmp_path = self.path.with_extension("tmp");
        let content = serde_json::to_string_pretty(&self.data).map_err(Error::Json)?;
        fs::write(&tmp_path, &content).map_err(|e| Error::Io(e))?;
        fs::rename(&tmp_path, &self.path).map_err(|e| Error::Io(e))?;
        Ok(())
    }

    /// 获取所有条目的不可变引用。
    pub fn entries(&self) -> &[ForwardingEntry] {
        &self.data.entries
    }

    /// 获取所有条目的克隆（用于 API 响应）。
    pub fn entries_cloned(&self) -> Vec<ForwardingEntry> {
        self.data.entries.clone()
    }

    /// 根据 ID 查找条目。
    pub fn find_entry(&self, id: Uuid) -> Result<&ForwardingEntry> {
        self.data
            .entries
            .iter()
            .find(|e| e.id == id)
            .ok_or(Error::NotFound(id))
    }

    /// 根据 ID 查找条目的可变引用。
    pub fn find_entry_mut(&mut self, id: Uuid) -> Result<&mut ForwardingEntry> {
        self.data
            .entries
            .iter_mut()
            .find(|e| e.id == id)
            .ok_or(Error::NotFound(id))
    }

    /// 添加新条目，自动填充 ID 和时间戳。返回克隆的条目。
    pub fn add_entry(&mut self, req: EntryRequest) -> Result<ForwardingEntry> {
        let now = Utc::now();
        let log_directory = req
            .log_directory
            .unwrap_or_else(|| default_log_dir(&self.path, &req.name));

        let entry = ForwardingEntry {
            id: Uuid::new_v4(),
            name: req.name,
            source_address: req.source_address,
            source_port: req.source_port,
            target_address: req.target_address,
            target_port: req.target_port,
            log_directory,
            enabled: req.enabled,
            created_at: now,
            updated_at: now,
        };

        self.data.entries.push(entry.clone());
        self.save()?;
        Ok(entry)
    }

    /// 更新已有条目。
    pub fn update_entry(&mut self, id: Uuid, req: EntryRequest) -> Result<ForwardingEntry> {
        let entry = self.find_entry_mut(id)?;

        entry.name = req.name;
        entry.source_address = req.source_address;
        entry.source_port = req.source_port;
        entry.target_address = req.target_address;
        entry.target_port = req.target_port;
        if let Some(dir) = req.log_directory {
            entry.log_directory = dir;
        }
        entry.enabled = req.enabled;
        entry.updated_at = Utc::now();

        let entry = entry.clone();
        self.save()?;
        Ok(entry)
    }

    /// 删除指定条目，返回被删除的条目（None 表示不存在）。
    pub fn remove_entry(&mut self, id: Uuid) -> Result<ForwardingEntry> {
        let idx = self
            .data
            .entries
            .iter()
            .position(|e| e.id == id)
            .ok_or(Error::NotFound(id))?;

        let entry = self.data.entries.remove(idx);
        self.save()?;
        Ok(entry)
    }

    /// 返回配置文件路径。
    pub fn config_path(&self) -> &Path {
        &self.path
    }
}

/// 基于配置文件路径和条目名称生成默认日志目录。
fn default_log_dir(config_path: &Path, name: &str) -> String {
    let parent = config_path
        .parent()
        .unwrap_or_else(|| Path::new("."));
    let safe_name = sanitize_dir_name(name);
    parent
        .join("logs")
        .join(&safe_name)
        .to_string_lossy()
        .to_string()
}

fn sanitize_dir_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_load_empty_creates_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.json");
        let store = ConfigStore::load(path.clone()).unwrap();
        assert!(store.entries().is_empty());
        assert!(path.exists());
    }

    #[test]
    fn test_add_and_find_entry() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.json");
        let mut store = ConfigStore::load(path).unwrap();

        let entry = store
            .add_entry(EntryRequest {
                name: "test".into(),
                source_address: "127.0.0.1".into(),
                source_port: 8080,
                target_address: "10.0.0.1".into(),
                target_port: 80,
                log_directory: None,
                enabled: true,
            })
            .unwrap();

        assert_eq!(entry.name, "test");
        assert_eq!(entry.source_port, 8080);

        let found = store.find_entry(entry.id).unwrap();
        assert_eq!(found.name, "test");
    }

    #[test]
    fn test_update_entry() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.json");
        let mut store = ConfigStore::load(path).unwrap();

        let entry = store
            .add_entry(EntryRequest {
                name: "original".into(),
                source_address: "127.0.0.1".into(),
                source_port: 8080,
                target_address: "10.0.0.1".into(),
                target_port: 80,
                log_directory: None,
                enabled: true,
            })
            .unwrap();

        let updated = store
            .update_entry(
                entry.id,
                EntryRequest {
                    name: "updated".into(),
                    source_address: "0.0.0.0".into(),
                    source_port: 9090,
                    target_address: "10.0.0.2".into(),
                    target_port: 443,
                    log_directory: None,
                    enabled: false,
                },
            )
            .unwrap();

        assert_eq!(updated.name, "updated");
        assert_eq!(updated.source_port, 9090);
        assert!(!updated.enabled);
    }

    #[test]
    fn test_remove_entry() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.json");
        let mut store = ConfigStore::load(path).unwrap();

        let entry = store
            .add_entry(EntryRequest {
                name: "to_delete".into(),
                source_address: "127.0.0.1".into(),
                source_port: 8080,
                target_address: "10.0.0.1".into(),
                target_port: 80,
                log_directory: None,
                enabled: true,
            })
            .unwrap();

        let removed = store.remove_entry(entry.id).unwrap();
        assert_eq!(removed.id, entry.id);
        assert!(store.find_entry(entry.id).is_err());
    }

    #[test]
    fn test_atomic_save_persists() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.json");
        let mut store = ConfigStore::load(path.clone()).unwrap();

        store
            .add_entry(EntryRequest {
                name: "persist".into(),
                source_address: "127.0.0.1".into(),
                source_port: 8080,
                target_address: "10.0.0.1".into(),
                target_port: 80,
                log_directory: None,
                enabled: true,
            })
            .unwrap();

        // 重新加载
        let store2 = ConfigStore::load(path).unwrap();
        assert_eq!(store2.entries().len(), 1);
        assert_eq!(store2.entries()[0].name, "persist");
    }

    #[test]
    fn test_not_found() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.json");
        let store = ConfigStore::load(path).unwrap();

        let result = store.find_entry(Uuid::new_v4());
        assert!(result.is_err());
    }
}
