// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;

/// Returns the user's home directory.
///
/// On Unix: `$HOME`
/// On Windows: `%USERPROFILE%` (falling back to `%HOMEDRIVE%%HOMEPATH%`)
pub fn home_dir() -> Option<PathBuf> {
    #[cfg(unix)]
    {
        std::env::var_os("HOME")
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
    }
    #[cfg(windows)]
    {
        std::env::var_os("USERPROFILE")
            .filter(|v| !v.is_empty())
            .or_else(|| {
                let drive = std::env::var_os("HOMEDRIVE")?;
                let path = std::env::var_os("HOMEPATH")?;
                let mut home = std::ffi::OsString::from(drive);
                home.push(path);
                Some(home)
            })
            .map(PathBuf::from)
    }
}

/// Returns the OS-specific configuration directory.
///
/// On Linux: `$XDG_CONFIG_HOME` or `~/.config`
/// On macOS: `~/Library/Application Support`
/// On Windows: `%APPDATA%`
pub fn config_dir() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME") {
            let path = PathBuf::from(&xdg);
            if !xdg.is_empty() && path.is_absolute() {
                return Some(path);
            }
        }
        home_dir().map(|h| h.join(".config"))
    }
    #[cfg(target_os = "macos")]
    {
        home_dir().map(|h| h.join("Library").join("Application Support"))
    }
    #[cfg(windows)]
    {
        std::env::var_os("APPDATA")
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn home_dir_returns_some_when_set() {
        let original = std::env::var_os("HOME");
        std::env::set_var("HOME", "/tmp/test-home");
        let result = home_dir();
        if let Some(orig) = original {
            std::env::set_var("HOME", orig);
        } else {
            std::env::remove_var("HOME");
        }
        assert_eq!(result, Some(PathBuf::from("/tmp/test-home")));
    }

    #[test]
    #[serial]
    fn home_dir_returns_none_when_unset() {
        let original = std::env::var_os("HOME");
        std::env::remove_var("HOME");
        let result = home_dir();
        if let Some(orig) = original {
            std::env::set_var("HOME", orig);
        }
        assert_eq!(result, None);
    }

    #[test]
    #[serial]
    fn home_dir_returns_none_when_empty() {
        let original = std::env::var_os("HOME");
        std::env::set_var("HOME", "");
        let result = home_dir();
        if let Some(orig) = original {
            std::env::set_var("HOME", orig);
        } else {
            std::env::remove_var("HOME");
        }
        assert_eq!(result, None);
    }

    #[cfg(target_os = "linux")]
    #[test]
    #[serial]
    fn config_dir_respects_xdg() {
        let orig_xdg = std::env::var_os("XDG_CONFIG_HOME");
        let orig_home = std::env::var_os("HOME");
        std::env::set_var("XDG_CONFIG_HOME", "/tmp/xdg-config");
        std::env::set_var("HOME", "/tmp/test-home");
        let result = config_dir();
        if let Some(v) = orig_xdg {
            std::env::set_var("XDG_CONFIG_HOME", v);
        } else {
            std::env::remove_var("XDG_CONFIG_HOME");
        }
        if let Some(v) = orig_home {
            std::env::set_var("HOME", v);
        }
        assert_eq!(result, Some(PathBuf::from("/tmp/xdg-config")));
    }

    #[cfg(target_os = "linux")]
    #[test]
    #[serial]
    fn config_dir_falls_back_to_home_dot_config() {
        let orig_xdg = std::env::var_os("XDG_CONFIG_HOME");
        let orig_home = std::env::var_os("HOME");
        std::env::remove_var("XDG_CONFIG_HOME");
        std::env::set_var("HOME", "/tmp/test-home");
        let result = config_dir();
        if let Some(v) = orig_xdg {
            std::env::set_var("XDG_CONFIG_HOME", v);
        }
        if let Some(v) = orig_home {
            std::env::set_var("HOME", v);
        }
        assert_eq!(result, Some(PathBuf::from("/tmp/test-home/.config")));
    }

    #[cfg(target_os = "linux")]
    #[test]
    #[serial]
    fn config_dir_rejects_relative_xdg() {
        let orig_xdg = std::env::var_os("XDG_CONFIG_HOME");
        let orig_home = std::env::var_os("HOME");
        std::env::set_var("XDG_CONFIG_HOME", "relative/path");
        std::env::set_var("HOME", "/tmp/test-home");
        let result = config_dir();
        if let Some(v) = orig_xdg {
            std::env::set_var("XDG_CONFIG_HOME", v);
        } else {
            std::env::remove_var("XDG_CONFIG_HOME");
        }
        if let Some(v) = orig_home {
            std::env::set_var("HOME", v);
        }
        assert_eq!(result, Some(PathBuf::from("/tmp/test-home/.config")));
    }
}
