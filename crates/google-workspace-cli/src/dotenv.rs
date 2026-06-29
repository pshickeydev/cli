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

use std::path::Path;

/// Load environment variables from a `.env` file.
/// Silently does nothing if the file is absent or unreadable.
/// Existing environment variables are never overwritten.
pub fn load(path: &Path) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };
    parse_and_set(&content);
}

fn parse_and_set(content: &str) {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };
        let key = key.trim();
        if key.is_empty() {
            continue;
        }
        let value = strip_quotes(value.trim());
        if std::env::var_os(key).is_none() {
            std::env::set_var(key, value);
        }
    }
}

fn strip_quotes(s: &str) -> &str {
    if s.len() >= 2
        && ((s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')))
    {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn clear_test_vars(vars: &[&str]) {
        for v in vars {
            std::env::remove_var(v);
        }
    }

    #[test]
    #[serial]
    fn parses_key_value() {
        clear_test_vars(&["DOTENV_TEST_A", "DOTENV_TEST_B"]);
        parse_and_set("DOTENV_TEST_A=hello\nDOTENV_TEST_B=world");
        assert_eq!(std::env::var("DOTENV_TEST_A").unwrap(), "hello");
        assert_eq!(std::env::var("DOTENV_TEST_B").unwrap(), "world");
        clear_test_vars(&["DOTENV_TEST_A", "DOTENV_TEST_B"]);
    }

    #[test]
    #[serial]
    fn skips_comments_and_blank_lines() {
        clear_test_vars(&["DOTENV_TEST_C"]);
        parse_and_set("# this is a comment\n\n   \nDOTENV_TEST_C=yes");
        assert_eq!(std::env::var("DOTENV_TEST_C").unwrap(), "yes");
        clear_test_vars(&["DOTENV_TEST_C"]);
    }

    #[test]
    #[serial]
    fn strips_double_quotes() {
        clear_test_vars(&["DOTENV_TEST_DQ"]);
        parse_and_set("DOTENV_TEST_DQ=\"quoted value\"");
        assert_eq!(std::env::var("DOTENV_TEST_DQ").unwrap(), "quoted value");
        clear_test_vars(&["DOTENV_TEST_DQ"]);
    }

    #[test]
    #[serial]
    fn strips_single_quotes() {
        clear_test_vars(&["DOTENV_TEST_SQ"]);
        parse_and_set("DOTENV_TEST_SQ='single quoted'");
        assert_eq!(std::env::var("DOTENV_TEST_SQ").unwrap(), "single quoted");
        clear_test_vars(&["DOTENV_TEST_SQ"]);
    }

    #[test]
    #[serial]
    fn does_not_overwrite_existing() {
        clear_test_vars(&["DOTENV_TEST_EXIST"]);
        std::env::set_var("DOTENV_TEST_EXIST", "original");
        parse_and_set("DOTENV_TEST_EXIST=overwritten");
        assert_eq!(std::env::var("DOTENV_TEST_EXIST").unwrap(), "original");
        clear_test_vars(&["DOTENV_TEST_EXIST"]);
    }

    #[test]
    #[serial]
    fn handles_empty_value() {
        clear_test_vars(&["DOTENV_TEST_EMPTY"]);
        parse_and_set("DOTENV_TEST_EMPTY=");
        assert_eq!(std::env::var("DOTENV_TEST_EMPTY").unwrap(), "");
        clear_test_vars(&["DOTENV_TEST_EMPTY"]);
    }

    #[test]
    #[serial]
    fn handles_whitespace_around_equals() {
        clear_test_vars(&["DOTENV_TEST_WS"]);
        parse_and_set("  DOTENV_TEST_WS  =  spaced  ");
        assert_eq!(std::env::var("DOTENV_TEST_WS").unwrap(), "spaced");
        clear_test_vars(&["DOTENV_TEST_WS"]);
    }

    #[test]
    fn missing_file_is_silent() {
        load(Path::new("/nonexistent/.env"));
    }

    #[test]
    fn skips_lines_without_equals() {
        clear_test_vars(&["DOTENV_TEST_NOEQ"]);
        parse_and_set("no_equals_here\nDOTENV_TEST_NOEQ=ok");
        assert_eq!(std::env::var("DOTENV_TEST_NOEQ").unwrap(), "ok");
        clear_test_vars(&["DOTENV_TEST_NOEQ"]);
    }

    #[test]
    fn skips_empty_key() {
        parse_and_set("=value_only");
        // Should not panic or set anything
    }
}
