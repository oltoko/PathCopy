use clap::{Parser, ValueEnum};

const WHITESPACE: &str = " ";
#[cfg(target_family = "windows")]
const LINE_BREAK: &str = "\r\n";
#[cfg(target_family = "unix")]
const LINE_BREAK: &str = "\n";
const TAB: &str = "\t";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(
    after_help = "If there are any issues, feel free to fill in a bug\nhttps://github.com/oltoko/PathCopy/issues"
)]
struct Args {
    /// The file(s) or directory(s) from which the absolute path(s) should be copied.
    path_values: Vec<String>,

    /// The Separator which is used between the results
    #[arg(short, long, default_value = "w")]
    separator: SeparatorValue,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SeparatorValue {
    /// A whitespace character [default]
    W,
    /// The system specific line break
    N,
    /// A tab character
    T,
}

fn main() {
    let args = Args::parse();

    let inputs = args.path_values;
    let separator = fetch_separator_from_input(args.separator);

    let paths = pc::to_absolut_paths(inputs);
    pc::paste_to_clipboard(paths, separator);
}

fn fetch_separator_from_input(s: SeparatorValue) -> &'static str {
    match s {
        SeparatorValue::W => WHITESPACE,
        SeparatorValue::N => LINE_BREAK,
        SeparatorValue::T => TAB,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_separator_from_input_returns_correct_values() {
        assert_eq!(fetch_separator_from_input(SeparatorValue::W), " ");
        assert_eq!(fetch_separator_from_input(SeparatorValue::N), LINE_BREAK);
        assert_eq!(fetch_separator_from_input(SeparatorValue::T), "\t");
    }
}
