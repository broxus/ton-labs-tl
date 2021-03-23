use std::io::Read;
use std::path::Path;
use std::{fs, path};

const OUTPUT_DIR: &str = "src/ton";
const TL_DIR: &str = "tl";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", TL_DIR);

    let mut files = fs::read_dir(TL_DIR)
        .unwrap_or_else(|_| panic!("Unable to read directory contents: {}", TL_DIR))
        .filter_map(Result::ok)
        .map(|d| d.path())
        .filter(|path| path.to_str().unwrap().ends_with(".tl"))
        .collect::<Vec<path::PathBuf>>();
    assert!(!files.is_empty());
    files.sort();
    let mut input = String::new();
    for file in files {
        if !input.is_empty() {
            input += "---types---\n";
        }
        fs::File::open(&file)
            .unwrap_or_else(|_| panic!("Unable to open file for reading: {}", file.to_string_lossy()))
            .read_to_string(&mut input)
            .unwrap_or_else(|_| panic!("Unable to read file contents: {}", file.to_string_lossy()));
        println!("cargo:rerun-if-changed={}", file.to_string_lossy());
    }
    ton_tl_codegen::generate_code_for(&input, Path::new(OUTPUT_DIR));
}
