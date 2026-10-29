// platform/windows/path_impl.rs
// Windows path provider implementation.

use crate::platform::PlatformPath;
use std::path::PathBuf;

pub struct PathImpl;

fn get_portable_dir() -> Option<PathBuf> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let data_dir = parent.join("Data");
            if data_dir.exists() {
                return Some(data_dir);
            }
        }
    }
    None
}

impl PlatformPath for PathImpl {
    fn app_data_dir(&self) -> PathBuf {
        if let Some(p) = get_portable_dir() {
            return p;
        }
        appdata().join("files-explorer")
    }

    fn app_cache_dir(&self) -> PathBuf {
        if let Some(p) = get_portable_dir() {
            return p.join("cache");
        }
        localappdata().join("files-explorer").join("cache")
    }

    fn app_config_dir(&self) -> PathBuf {
        if let Some(p) = get_portable_dir() {
            return p.join("config");
        }
        appdata().join("files-explorer").join("config")
    }

    fn app_log_dir(&self) -> PathBuf {
        if let Some(p) = get_portable_dir() {
            return p.join("logs");
        }
        appdata().join("files-explorer").join("logs")
    }

    fn app_resource_dir(&self) -> PathBuf {
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."))
    }

    fn home_dir(&self) -> PathBuf {
        std::env::var("USERPROFILE")
            .map(PathBuf::from)
            .or_else(|_| {
                std::env::var("HOMEDRIVE").and_then(|d| {
                    std::env::var("HOMEPATH").map(|p| PathBuf::from(format!("{d}{p}")))
                })
            })
            .unwrap_or_else(|_| PathBuf::from("."))
    }

    fn desktop_dir(&self) -> PathBuf {
        dirs::desktop_dir().unwrap_or_else(|| self.home_dir().join("Desktop"))
    }

    fn documents_dir(&self) -> PathBuf {
        dirs::document_dir().unwrap_or_else(|| self.home_dir().join("Documents"))
    }

    fn downloads_dir(&self) -> PathBuf {
        dirs::download_dir().unwrap_or_else(|| self.home_dir().join("Downloads"))
    }

    fn pictures_dir(&self) -> PathBuf {
        dirs::picture_dir().unwrap_or_else(|| self.home_dir().join("Pictures"))
    }

    fn music_dir(&self) -> PathBuf {
        dirs::audio_dir().unwrap_or_else(|| self.home_dir().join("Music"))
    }

    fn videos_dir(&self) -> PathBuf {
        dirs::video_dir().unwrap_or_else(|| self.home_dir().join("Videos"))
    }

    fn temp_dir(&self) -> PathBuf {
        std::env::temp_dir().join("files-explorer-preview")
    }
}

fn appdata() -> PathBuf {
    std::env::var("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home().join("AppData").join("Roaming"))
}

fn localappdata() -> PathBuf {
    std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| home().join("AppData").join("Local"))
}

fn home() -> PathBuf {
    std::env::var("USERPROFILE")
        .map(PathBuf::from)
        .or_else(|_| {
            std::env::var("HOMEDRIVE")
                .and_then(|d| std::env::var("HOMEPATH").map(|p| PathBuf::from(format!("{d}{p}"))))
        })
        .unwrap_or_else(|_| PathBuf::from("."))
}

impl PathImpl {
    pub fn instance() -> &'static PathImpl {
        &PathImpl
    }
}
