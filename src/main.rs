use clap::{crate_version, App, Arg};

const PATH_NAME: &str = "path";
const SEPARATOR_NAME: &str = "separator";

#[cfg(any(target_family = "windows"))]
const LINE_BREAK: &str = "\r\n";
#[cfg(any(target_family = "unix"))]
const LINE_BREAK: &str = "\n";

fn main() {
    let matches = App::new("PathCopy")
        .version(crate_version!()).version_short("v")
        .about("Copies the absolute Path of the given file or directory to the clipboard. It follows symlinks and uses the absolute path of the linked file.")
        .after_help("If there are any issues, feel free to fill in a bug\nhttps://github.com/oltoko/PathCopy/issues")
        .arg(Arg::with_name(PATH_NAME)
            .help("The file(s) or directory(s) from which the absolute path(s) should be copied.")
            .multiple(true)
            .required(true)
            .index(1))
        .arg(Arg::with_name(SEPARATOR_NAME)
            .help("The separator which should be used when multiple paths are put into the clipboard.\n\
                    You can choose between 3 separators:\n\
                    \t%w - A whitespace character [default]\n\
                    \t%n - The system specific line break\n\
                    \t%t - A tab character")
            .short("s")
            .long("separator")
            .possible_values(&["%w", "%n", "%t"])
            .default_value("%w")
            .hide_default_value(true)
            .hide_possible_values(true))
        .get_matches();

    let inputs = matches.values_of(PATH_NAME).unwrap();
    let sep = matches.value_of(SEPARATOR_NAME).unwrap();
    let sep = fetch_separator_from_input(sep);

    let paths = pc::to_absolut_paths(inputs);
    pc::paste_to_clipboard(paths, sep);
}

fn fetch_separator_from_input(s: &str) -> &str {
    match s {
        "%w" => " ",
        "%n" => LINE_BREAK,
        "%t" => "\t",
        _ => {
            eprintln!("Unexpected separator! This should not happen, please fill in a Bug.");
            std::process::exit(5);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_separator_from_input_returns_correct_values() {
        assert_eq!(fetch_separator_from_input("%w"), " ");
        assert_eq!(fetch_separator_from_input("%n"), LINE_BREAK);
        assert_eq!(fetch_separator_from_input("%t"), "\t");
    }
}
