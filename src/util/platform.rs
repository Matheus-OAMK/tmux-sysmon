// Platform utilities: default disk path and WSL detection.

use std::env;
use std::path::Path;

/// Detect WSL on Linux using multiple heuristics.
pub fn is_wsl() -> bool {
    #[cfg(target_os = "linux")]
    {
        let env_hit =
            env::var_os("WSL_DISTRO_NAME").is_some() || env::var_os("WSL_INTEROP").is_some();

        let path_hit = ["/usr/lib/wsl", "/proc/sys/fs/binfmt_misc/WSLInterop"]
            .iter()
            .any(|p| Path::new(p).exists());

        let version_hit = std::fs::read_to_string("/proc/version")
            .map(|s| s.to_lowercase().contains("microsoft"))
            .unwrap_or(false);

        return env_hit || path_hit || version_hit;
    }
    #[cfg(not(target_os = "linux"))]
    {
        false
    }
}

/// Resolve default disk path per OS with WSL special-case on Linux.
pub fn default_disk_path() -> String {
    #[cfg(target_os = "windows")]
    {
        return "C:".to_string();
    }
    #[cfg(target_os = "macos")]
    {
        return "/System/Volumes/Data".to_string();
    }
    #[cfg(target_os = "linux")]
    {
        if is_wsl() {
            "/usr/lib/wsl/drivers".to_string()
        } else {
            "/".to_string()
        }
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        "/".to_string()
    }
}

/// Pick user-provided path or fall back to OS default.
pub fn pick_path(user: Option<String>) -> String {
    match user {
        Some(p) if !p.trim().is_empty() => p,
        _ => default_disk_path(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_user_path_when_provided() {
        assert_eq!(pick_path(Some("/data".into())), "/data");
        assert_eq!(pick_path(Some(" C: ".into())), " C: ");
    }

    #[test]
    fn default_path_sane_per_os() {
        #[cfg(target_os = "windows")]
        assert_eq!(default_disk_path(), "C:");

        #[cfg(target_os = "macos")]
        assert_eq!(default_disk_path(), "/System/Volumes/Data");

        #[cfg(target_os = "linux")]
        {
            let p = default_disk_path();
            assert!(p == "/" || p == "/usr/lib/wsl/drivers");
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        assert_eq!(default_disk_path(), "/");
    }
}
