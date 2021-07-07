use clap::{crate_version, App, Arg};
use std::path::Path;
use clipboard::{ClipboardContext, ClipboardProvider};

const PATH_NAME: &str = "path";

fn main() {

    let matches = App::new("PathCopy")
        .version(crate_version!()).version_short("v")
        .about("Copies the absolute Path of the given file or directory to the clipboard. It follows symlinks and uses the absolute path of the linked file.")
        .after_help("If there are any issues, feel free to fill in a bug\nhttps://github.com/oltoko/PathCopy/issues")
        .arg(Arg::with_name(PATH_NAME)
            .help("The file or directory from which the absolute path should be copied.")
            .required(true)
            .index(1))
        .get_matches();

    let input_path = matches.value_of(PATH_NAME).unwrap();
    let path = Path::new(input_path);

    if !path.exists() {
        eprintln!("The given path doesn't exist!");
        std::process::exit(1);
    }

    let path = match path.canonicalize() {
        Ok(buf) => buf,
        Err(e) => {
            eprintln!("Failed to get absolute path of {}: {}", input_path, e);
            std::process::exit(2);
        }
    };

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let content = match path.into_os_string().into_string() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Failed to place the given path into the clipboard. The absolute version might contain some unsupported characters.");
            std::process::exit(3);
        },
    };

    match ctx.set_contents(content) {
        Ok(()) => ()/* Everything is fine 😌 */,
        Err(e) => {
            eprintln!("Failed to place the absolute path of {} into the clipboard: {}", input_path, e);
            std::process::exit(4);
        },
    };
}
