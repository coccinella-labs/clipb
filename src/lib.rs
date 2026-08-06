use std::path::PathBuf;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Format a list of paths as newline-separated clipboard text.
pub fn format_paths(paths: &[PathBuf]) -> String {
    let mut out = String::new();
    for (index, path) in paths.iter().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        out.push_str(&path.display().to_string());
    }
    out
}

/// Resolve each path to an absolute path, leaving symlinks untouched.
pub fn resolve_paths(paths: &[PathBuf]) -> Vec<PathBuf> {
    paths
        .iter()
        .map(|path| {
            if path.is_absolute() {
                path.clone()
            } else {
                std::env::current_dir()
                    .map(|cwd| cwd.join(path))
                    .unwrap_or_else(|_| path.clone())
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_paths_newline_separated() {
        let paths = vec![PathBuf::from("a"), PathBuf::from("b c")];
        assert_eq!(format_paths(&paths), "a\nb c");
    }

    #[test]
    fn formats_single_path_without_trailing_newline() {
        let paths = vec![PathBuf::from("a")];
        assert_eq!(format_paths(&paths), "a");
    }

    #[test]
    fn resolves_relative_paths_against_cwd() {
        let resolved = resolve_paths(&[PathBuf::from("src")]);
        assert!(resolved[0].is_absolute());
    }

    #[test]
    fn keeps_absolute_paths_as_is() {
        use std::path::Path;
        let abs = Path::new("/tmp/x").to_path_buf();
        let resolved = resolve_paths(std::slice::from_ref(&abs));
        assert_eq!(resolved[0], abs);
    }
}
