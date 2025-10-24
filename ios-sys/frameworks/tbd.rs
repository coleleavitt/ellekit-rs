//! TBD (Text-Based Dylib) file parser
//!
//! Parses Apple's TBD stub library format to extract exported symbols and Objective-C classes.
//! Supports both TBD v3 and v4 formats.

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Information extracted from a TBD file
#[derive(Debug, Default)]
pub struct TbdInfo {
    pub symbols: Vec<String>,
    pub objc_classes: Vec<String>,
    pub objc_ivars: Vec<String>,
}

/// TBD file structure (v3 format - most common in Theos SDKs)
#[derive(Debug, Deserialize)]
struct TbdFileV3 {
    #[serde(rename = "exports")]
    exports: Option<Vec<TbdExport>>,
}

#[derive(Debug, Deserialize)]
struct TbdExport {
    #[serde(rename = "symbols", default)]
    symbols: Vec<String>,

    #[serde(rename = "objc-classes", default)]
    objc_classes: Vec<String>,

    #[serde(rename = "objc-ivars", default)]
    objc_ivars: Vec<String>,
}

/// TBD file structure (v4 format - newer iOS SDKs)
#[derive(Debug, Deserialize)]
struct TbdFileV4 {
    #[serde(rename = "exports")]
    exports: Option<Vec<TbdExportV4>>,
}

#[derive(Debug, Deserialize)]
struct TbdExportV4 {
    #[serde(rename = "symbols", default)]
    symbols: Vec<String>,

    #[serde(rename = "objc-classes", default)]
    objc_classes: Vec<String>,

    #[serde(rename = "objc-ivars", default)]
    objc_ivars: Vec<String>,
}

/// Parse a TBD file and extract symbols and Objective-C class information
pub fn parse_tbd_file(path: &Path) -> Option<TbdInfo> {
    let content = fs::read_to_string(path).ok()?;

    // Try parsing as v3 first (most common)
    if content.contains("!tapi-tbd-v3") || content.starts_with("--- !tapi-tbd") {
        if let Ok(tbd) = serde_yaml::from_str::<TbdFileV3>(&content) {
            let mut info = TbdInfo::default();

            if let Some(exports) = tbd.exports {
                for export in exports {
                    info.symbols.extend(export.symbols);
                    info.objc_classes.extend(export.objc_classes);
                    info.objc_ivars.extend(export.objc_ivars);
                }
            }

            return Some(info);
        }
    }

    // Try parsing as v4
    if content.contains("!tapi-tbd-v4") {
        if let Ok(tbd) = serde_yaml::from_str::<TbdFileV4>(&content) {
            let mut info = TbdInfo::default();

            if let Some(exports) = tbd.exports {
                for export in exports {
                    info.symbols.extend(export.symbols);
                    info.objc_classes.extend(export.objc_classes);
                    info.objc_ivars.extend(export.objc_ivars);
                }
            }

            return Some(info);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_parse_tbd_v3() {
        let tbd_content = r#"
--- !tapi-tbd-v3
archs: [arm64]
install-name: /System/Library/Frameworks/TestFramework.framework/TestFramework
exports:
  - archs: [arm64]
    symbols: [_TestFunction, _kTestConstant]
    objc-classes: [TestClass, TestViewController]
"#;

        let temp_file = std::env::temp_dir().join("test_v3.tbd");
        fs::write(&temp_file, tbd_content).unwrap();

        let info = parse_tbd_file(&temp_file).unwrap();
        assert_eq!(info.symbols.len(), 2);
        assert_eq!(info.objc_classes.len(), 2);

        fs::remove_file(temp_file).ok();
    }
}
