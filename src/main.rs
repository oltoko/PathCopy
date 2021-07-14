use clap::{crate_version, App, Arg};

const PATH_NAME: &str = "path";

fn main() {
    let matches = App::new("PathCopy")
        .version(crate_version!()).version_short("v")
        .about("Copies the absolute Path of the given file or directory to the clipboard. It follows symlinks and uses the absolute path of the linked file.")
        .after_help("If there are any issues, feel free to fill in a bug\nhttps://github.com/oltoko/PathCopy/issues")
        .arg(Arg::with_name(PATH_NAME)
            .help("The file or directory from which the absolute path should be copied.")
            .multiple(true)
            .required(true)
            .index(1))
        .get_matches();

    let inputs = matches.values_of(PATH_NAME).unwrap();
    let paths = pc::to_absolut_paths(inputs);

    pc::paste_to_clipboard(paths);
}
