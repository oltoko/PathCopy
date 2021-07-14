use clap::Values;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

pub fn to_absolut_paths(input: Values) -> Vec<PathBuf> {
    input.map(to_absolut_path).collect()
}

fn to_absolut_path<S: AsRef<OsStr> + ?Sized>(input: &S) -> PathBuf {
    let path = Path::new(input);

    if !path.exists() {
        eprintln!("The path '{}' doesn't exist!", path.display());
        std::process::exit(1);
    }

    match path.canonicalize() {
        Ok(buf) => buf,
        Err(e) => {
            eprintln!("Failed to get absolute path of {}: {}", path.display(), e);
            std::process::exit(2);
        }
    }
}

pub fn paste_to_clipboard(paths: Vec<PathBuf>, sep: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let content = concat_paths(paths, sep);

    match ctx.set_contents(content) {
        Ok(()) => (), /* Everything is fine 😌 */
        Err(e) => {
            eprintln!(
                "Failed to place the absolute path(s) into the clipboard: {}",
                e
            );
            std::process::exit(4);
        }
    };
}

fn concat_paths(paths: Vec<PathBuf>, sep: &str) -> String {
    let strings: Vec<String> = paths
        .iter()
        .map(|buf| convert_to_string(buf.as_path()))
        .collect();
    strings.join(sep)
}

fn convert_to_string(path: &Path) -> String {
    match path.to_path_buf().into_os_string().into_string() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Failed to place the given path into the clipboard! The absolute version might contain some unsupported characters.");
            std::process::exit(3);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn concat_paths_joins_two_paths_with_separator() {
        let path_1 = Path::new("./Cargo.toml").to_path_buf();
        let path_2 = Path::new("./Cargo.lock").to_path_buf();
        let actual = concat_paths(vec![path_1, path_2], "🦀");
        assert_eq!(actual, "./Cargo.toml🦀./Cargo.lock")
    }

    #[test]
    fn concat_paths_dont_add_separator_for_single_value() {
        let actual = concat_paths(vec![Path::new("./Cargo.toml").to_path_buf()], " ");
        assert_eq!(actual, "./Cargo.toml")
    }

    #[test]
    fn convert_to_string_converts_path_to_string() {
        let actual = convert_to_string(&Path::new("./Cargo.toml"));
        assert_eq!(actual, "./Cargo.toml".to_string())
    }
}
