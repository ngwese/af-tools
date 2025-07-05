use std::{collections::HashMap, path::Path, path::PathBuf};

use anyhow::{Context, Result};
use regex::Regex;
use strfmt::strfmt;

const DEFAULT_FILE_TEMPLATE: &str = r"{prefix}{stem}{suffix}.{extension}";
const DEFAULT_SERIAL_REGEX: &str = r"(\d{3})";

#[derive(Debug, Clone)]
pub struct PathTemplate {
    pub file_template: String,
    pub serial_regex: Regex,

    pub parent_dir: Option<PathBuf>,
    pub sub_dir: Option<PathBuf>,
    pub prefix: Option<String>,
    pub stem: Option<String>,
    pub suffix: Option<String>,
    pub extension: Option<String>,
}

impl PathTemplate {
    pub fn new(file_template: &str) -> Self {
        Self {
            file_template: file_template.to_string(),
            serial_regex: Regex::new(DEFAULT_SERIAL_REGEX).unwrap(),

            parent_dir: None,
            sub_dir: None,
            prefix: None,
            stem: None,
            suffix: None,
            extension: None,
        }
    }

    fn extract_serial(&self, input: &Path) -> Option<String> {
        let haystack = input.file_stem().unwrap().to_str().unwrap_or_default();
        let caps = self.serial_regex.captures(haystack);
        caps?.get(0).map(|m| m.as_str().to_string())
    }

    pub fn apply(&self, input: &Path) -> Result<PathBuf> {
        // determine enclosing directory
        let mut base_path = if let Some(ref parent) = self.parent_dir {
            parent.clone()
        } else {
            input
                .parent()
                .ok_or(anyhow::anyhow!("No source directory"))?
                .to_path_buf()
        };

        // add sub-directory if specified;
        if let Some(ref sub_dir) = self.sub_dir {
            base_path.push(sub_dir);
        };

        // NOTE: Cannnot canonicalize here because the base path may not exist yet
        let mut path = base_path;

        // file name and extension
        let stem = if let Some(ref stem) = self.stem {
            stem.clone()
        } else {
            input
                .file_stem()
                .ok_or(anyhow::anyhow!("No file stem"))?
                .to_string_lossy()
                .to_string()
        };

        let extension = if let Some(ref extension) = self.extension {
            extension.clone()
        } else {
            input
                .extension()
                .ok_or(anyhow::anyhow!("No file extension"))?
                .to_string_lossy()
                .to_string()
        };

        let mut vars: HashMap<String, String> = HashMap::new();
        vars.insert("stem".to_string(), stem);
        vars.insert("extension".to_string(), extension);
        vars.insert(
            "prefix".to_string(),
            self.prefix.clone().unwrap_or_default(),
        );
        vars.insert(
            "suffix".to_string(),
            self.suffix.clone().unwrap_or_default(),
        );

        let file_name =
            strfmt(&self.file_template, &vars).with_context(|| "Failed to format file name")?;

        path.push(file_name);

        Ok(path)
    }
}

impl Default for PathTemplate {
    fn default() -> Self {
        Self::new(DEFAULT_FILE_TEMPLATE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_template_default() {
        let template = PathTemplate::default();
        assert_eq!(template.file_template, DEFAULT_FILE_TEMPLATE);
    }

    #[test]
    fn test_path_template_apply() {
        let template = PathTemplate::default();
        let input = Path::new("test.wav");
        let path = template.apply(&input).unwrap();
        assert_eq!(path, PathBuf::from("test.wav"));
    }

    #[test]
    fn test_path_template_apply_with_prefix() {
        let mut template = PathTemplate::default();
        template.prefix = Some("prefix_".to_string());

        let path = template.apply(Path::new("test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("prefix_test.wav"));

        let path = template.apply(Path::new("./test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("./prefix_test.wav"));

        let path = template.apply(Path::new("relative/test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("relative/prefix_test.wav"));

        let path = template.apply(Path::new("/absolute/test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("/absolute/prefix_test.wav"));
    }

    #[test]
    fn test_path_template_apply_with_suffix() {
        let mut template = PathTemplate::default();
        template.suffix = Some("_suffix".to_string());

        let path = template.apply(Path::new("test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("test_suffix.wav"));

        let path = template.apply(Path::new("./test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("./test_suffix.wav"));

        let path = template.apply(Path::new("relative/test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("relative/test_suffix.wav"));

        let path = template.apply(Path::new("/absolute/test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("/absolute/test_suffix.wav"));
    }

    #[test]
    fn test_path_template_apply_with_stem() {
        let mut template = PathTemplate::default();
        template.stem = Some("stem".to_string());

        let path = template.apply(Path::new("test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("stem.wav"));
    }

    #[test]
    fn test_path_template_apply_with_extension() {
        let mut template = PathTemplate::default();
        template.extension = Some("aiff".to_string());

        let path = template.apply(Path::new("test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("test.aiff"));

        let path = template.apply(Path::new("./test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("./test.aiff"));

        let path = template.apply(Path::new("relative/test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("relative/test.aiff"));

        let path = template.apply(Path::new("/absolute/test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("/absolute/test.aiff"));
    }

    #[test]
    fn test_path_template_apply_with_relative_parent_dir() {
        let mut template = PathTemplate::default();
        template.parent_dir = Some(PathBuf::from("parent"));

        let path = template.apply(Path::new("test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("parent/test.wav"));

        let path = template.apply(Path::new("./test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("parent/test.wav"));

        let path = template.apply(Path::new("relative/test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("parent/test.wav"));

        let path = template.apply(Path::new("/absolute/test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("parent/test.wav"));
    }

    #[test]
    fn test_path_template_apply_with_absolute_parent_dir() {
        let mut template = PathTemplate::default();
        template.parent_dir = Some(PathBuf::from("/absolute"));

        let path = template.apply(Path::new("test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("/absolute/test.wav"));

        let path = template.apply(Path::new("./test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("/absolute/test.wav"));

        let path = template.apply(Path::new("relative/test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("/absolute/test.wav"));

        let path = template.apply(Path::new("/old_absolute/test.wav")).unwrap();
        assert_eq!(path, PathBuf::from("/absolute/test.wav"));
    }

    #[test]
    fn test_path_template_extract_serial() {
        let template = PathTemplate::default();
        let serial = template.extract_serial(Path::new("MixPre-003_Ambix.wav"));
        assert_eq!(serial, Some("003".to_string()));

        let serial = template.extract_serial(Path::new("MixPre-03_Ambix.wav"));
        assert_eq!(serial, None);

        let serial = template.extract_serial(Path::new("/foo/something_Ambix.wav"));
        assert_eq!(serial, None);
    }

    #[test]
    fn test_path_template_extract_serial_only_in_stem() {
        let template = PathTemplate::default();

        let serial = template.extract_serial(Path::new("/this/001/MixPre-003_Ambix.wav"));
        assert_eq!(serial, Some("003".to_string()));

        let serial = template.extract_serial(Path::new("/this/001/something_Ambix.wav"));
        assert_eq!(serial, None);
    }
}
