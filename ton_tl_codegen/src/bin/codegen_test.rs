use std::io::Read;
use std::path::Path;
use std::{fs, path};

const OUTPUT_DIR: &str = "ton_api/src/ton";
const TL_DIR: &str = "ton_api/tl";

fn main() {
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
        fs::File::open(&file).unwrap().read_to_string(&mut input).unwrap();
    }
    ton_tl_codegen::generate_code_for(&input, Path::new(OUTPUT_DIR));
}
