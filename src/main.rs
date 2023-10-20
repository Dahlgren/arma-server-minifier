use glob::glob;
use hemtt_pbo::{ReadablePbo, WritablePbo};
use std::env;
use std::fs::{create_dir_all, File};
use std::io::{Cursor, Read};
use std::path::Path;

fn help() {
    println!("usage:
arma-server-minifier <input-path> <output-path>
    Optimizes PBOs at input path into output path.");
}

fn minify_pbo(input_file: &Path, output_file: &Path) {
    let input_pbo_file = File::open(input_file).expect("Could not open input file");
    let mut input_pbo = ReadablePbo::from(input_pbo_file).expect("could not open pbo");

    let mut output_pbo: WritablePbo<Cursor<Vec<u8>>> = WritablePbo::new();
    for (key, value) in input_pbo.properties() {
        output_pbo.add_property(key, value);
    }

    for file_entry in input_pbo.files() {
        let mut data = Vec::new();

        if file_entry.filename().ends_with(".paa") {
            output_pbo.add_file_with_header(file_entry, Cursor::new(data));
            continue
        }

        let mut file = input_pbo.file(file_entry.filename()).unwrap().unwrap();
        file.read_to_end(&mut data).unwrap();
        output_pbo.add_file_with_header(file_entry, Cursor::new(data)).expect("failed to add file");
    }

    let mut output_pbo_file = &mut File::create(output_file).expect("Could not open output file");
    output_pbo.write(&mut output_pbo_file, true).expect("Failed to write PBO");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let input_folder = Path::new(&args[1]);
            let output_folder = Path::new(&args[2]);
            let glob_pattern = format!("{}/**/*.pbo", &args[1]);

            for entry in glob(glob_pattern.as_str()).expect("Failed to read glob pattern") {
                match entry {
                    Ok(input_path) => {
                        let file_path = input_path.strip_prefix(Path::new(input_folder)).expect("Not a prefix");
                        let output_path = output_folder.join(file_path);
                        create_dir_all(output_path.parent().expect("no parent")).expect("failed to create folders");
                        minify_pbo(input_path.as_path(), output_path.as_path());
                    },
                    Err(e) => println!("{:?}", e),
                }
            }
        },
        _ => {
            help();
        }
    }
}
