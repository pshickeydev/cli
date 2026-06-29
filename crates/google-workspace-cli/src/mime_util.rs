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

/// Guess the MIME type from a file path's extension.
/// Returns `None` for unrecognized extensions.
pub fn guess_mime(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?;
    Some(match ext.to_ascii_lowercase().as_str() {
        // Text
        "txt" | "text" | "log" => "text/plain",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "csv" => "text/csv",
        "xml" => "text/xml",
        "md" | "markdown" => "text/markdown",
        "yaml" | "yml" => "text/yaml",
        "ics" => "text/calendar",
        "rtf" => "text/rtf",

        // Application
        "json" => "application/json",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "gz" | "gzip" => "application/gzip",
        "tar" => "application/x-tar",
        "7z" => "application/x-7z-compressed",
        "rar" => "application/vnd.rar",
        "js" | "mjs" => "application/javascript",
        "wasm" => "application/wasm",
        "xhtml" => "application/xhtml+xml",
        "eml" => "message/rfc822",

        // Microsoft Office (modern)
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
        // Microsoft Office (legacy)
        "doc" => "application/msword",
        "xls" => "application/vnd.ms-excel",
        "ppt" => "application/vnd.ms-powerpoint",

        // Images
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "bmp" => "image/bmp",
        "tiff" | "tif" => "image/tiff",

        // Audio
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "flac" => "audio/flac",
        "aac" => "audio/aac",

        // Video
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "mkv" => "video/x-matroska",

        // Fonts
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "otf" => "font/otf",

        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn common_text_types() {
        assert_eq!(guess_mime(Path::new("file.txt")), Some("text/plain"));
        assert_eq!(guess_mime(Path::new("page.html")), Some("text/html"));
        assert_eq!(guess_mime(Path::new("data.csv")), Some("text/csv"));
        assert_eq!(guess_mime(Path::new("notes.md")), Some("text/markdown"));
        assert_eq!(guess_mime(Path::new("style.css")), Some("text/css"));
    }

    #[test]
    fn common_application_types() {
        assert_eq!(guess_mime(Path::new("doc.pdf")), Some("application/pdf"));
        assert_eq!(guess_mime(Path::new("data.json")), Some("application/json"));
        assert_eq!(
            guess_mime(Path::new("archive.zip")),
            Some("application/zip")
        );
    }

    #[test]
    fn office_documents() {
        assert_eq!(
            guess_mime(Path::new("report.docx")),
            Some("application/vnd.openxmlformats-officedocument.wordprocessingml.document")
        );
        assert_eq!(
            guess_mime(Path::new("sheet.xlsx")),
            Some("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        );
    }

    #[test]
    fn image_types() {
        assert_eq!(guess_mime(Path::new("photo.png")), Some("image/png"));
        assert_eq!(guess_mime(Path::new("photo.jpg")), Some("image/jpeg"));
        assert_eq!(guess_mime(Path::new("photo.jpeg")), Some("image/jpeg"));
        assert_eq!(guess_mime(Path::new("icon.svg")), Some("image/svg+xml"));
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(guess_mime(Path::new("FILE.PDF")), Some("application/pdf"));
        assert_eq!(guess_mime(Path::new("photo.Jpg")), Some("image/jpeg"));
        assert_eq!(guess_mime(Path::new("DATA.JSON")), Some("application/json"));
    }

    #[test]
    fn unknown_extension_returns_none() {
        assert_eq!(guess_mime(Path::new("file.zzqqxx")), None);
        assert_eq!(guess_mime(Path::new("file.unknown")), None);
    }

    #[test]
    fn no_extension_returns_none() {
        assert_eq!(guess_mime(Path::new("Makefile")), None);
        assert_eq!(guess_mime(Path::new("/path/to/noext")), None);
    }

    #[test]
    fn full_paths_work() {
        assert_eq!(
            guess_mime(Path::new("/home/user/docs/file.txt")),
            Some("text/plain")
        );
        assert_eq!(
            guess_mime(Path::new("relative/path/image.png")),
            Some("image/png")
        );
    }
}
