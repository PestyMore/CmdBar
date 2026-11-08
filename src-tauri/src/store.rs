use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandItem {
    pub id: String,
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub workdir: String,
    #[serde(default)]
    pub run_as_admin: bool,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub last_run_at: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    pub version: u8,
    #[serde(default)]
    pub commands: Vec<CommandItem>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorePayload {
    pub commands: Vec<CommandItem>,
    pub data_dir: String,
}

#[derive(Deserialize)]
struct LegacyItem {
    #[serde(default)]
    name: String,
    #[serde(default)]
    path: String,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            version: 1,
            commands: Vec::new(),
        }
    }
}

pub fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(windows)]
fn config_root() -> Result<PathBuf, String> {
    let base = std::env::var_os("APPDATA").ok_or("无法读取 %APPDATA% 环境变量")?;
    Ok(PathBuf::from(base))
}

#[cfg(not(windows))]
fn config_root() -> Result<PathBuf, String> {
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(xdg));
    }
    let home = std::env::var_os("HOME").ok_or("无法读取 HOME 环境变量")?;
    Ok(PathBuf::from(home).join(".config"))
}

pub fn data_dir() -> Result<PathBuf, String> {
    Ok(config_root()?.join("com.cmdbar"))
}

pub fn read_store() -> Result<Store, String> {
    let path = data_dir()?.join("commands.json");
    if !path.exists() {
        return Ok(Store::default());
    }
    let text = fs::read_to_string(&path).map_err(|e| format!("读取数据文件失败：{e}"))?;
    serde_json::from_str(&text).map_err(|e| {
        format!(
            "数据文件 {} 内容损坏（{}），请修正或删除后重试",
            path.display(),
            e
        )
    })
}

pub fn write_store(store: &Store) -> Result<(), String> {
    let dir = data_dir()?;
    fs::create_dir_all(&dir).map_err(|e| format!("创建数据目录失败：{e}"))?;
    let path = dir.join("commands.json");
    let temp = dir.join("commands.json.tmp");
    let json = serde_json::to_string_pretty(store).map_err(|e| format!("序列化失败：{e}"))?;
    fs::write(&temp, json).map_err(|e| format!("写入数据文件失败：{e}"))?;
    fs::rename(&temp, &path).map_err(|e| {
        let _ = fs::remove_file(&temp);
        format!("写入数据文件失败：{e}")
    })?;
    Ok(())
}

/// 首启读取：文件不存在时用旧版 localStorage 数据做一次性迁移。
pub fn load_store(legacy: Option<String>) -> Result<StorePayload, String> {
    let dir = data_dir()?;
    if !dir.join("commands.json").exists() {
        let timestamp = now_millis();
        let commands = legacy
            .as_deref()
            .and_then(|raw| serde_json::from_str::<Vec<LegacyItem>>(raw).ok())
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .filter(|(_, item)| !item.path.trim().is_empty())
            .map(|(index, item)| CommandItem {
                id: format!("legacy-{timestamp}-{index}"),
                name: {
                    let name = item.name.trim();
                    if name.is_empty() {
                        format!("命令 {}", index + 1)
                    } else {
                        name.to_string()
                    }
                },
                command: item.path.trim().to_string(),
                workdir: String::new(),
                run_as_admin: false,
                created_at: timestamp,
                last_run_at: None,
            })
            .collect();
        write_store(&Store {
            version: 1,
            commands,
        })?;
    }

    let store = read_store()?;
    Ok(StorePayload {
        commands: store.commands,
        data_dir: dir.to_string_lossy().into_owned(),
    })
}

pub fn save_commands(commands: Vec<CommandItem>) -> Result<(), String> {
    write_store(&Store {
        version: 1,
        commands,
    })
}
