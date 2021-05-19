mod color_list;
use std::env;
//use std::fmt::Write;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

fn get_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn parse_path(path: &str) -> Vec<String> {
    let mut files = vec![];
    let md = fs::metadata(path).unwrap();
    
    //TODO: FIND BETTER WAY TO CHECK VALID EXTENSIONS
    let mut valid_extensions = HashSet::new();
    valid_extensions.insert("png");

    //TODO: MAKE THE FOLLOWING CODE LESS SPAGHETTI
    if md.is_dir() {
        let directory = fs::read_dir(path).expect("Could not read directory");

        for entry in directory {
            let entry_name = entry
                .expect("Could not read file from directory")
                .file_name()
                .into_string()
                .expect("Filename could not be converted into string");

            let entry_name = format!("{}/{}", path, entry_name);

            //Recursive call, if the current path is a directory
            //pass each of its children to another call of parse_path
            let mut subdir_files = parse_path(&entry_name);

            files.append(&mut subdir_files);
        }
    } else {
        if let Some(ext) = get_extension(path) {
            if valid_extensions.contains(ext) {
                files.push(path.to_string());
            }
        }
    }

    files
}

//As a first argument it expects the name of the file for which you want the palette
//As a second argument is the name you want to give to the palette

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let palette_name = &args[2];

    let files = parse_path(path);
    println!("{:?}", files);

    let mut rgb_list = color_list::ColorList {
        colors_set: HashSet::new(),
    };

    for filename in files {
        rgb_list.add_file(String::from(filename));
    }

    let gimp_format = rgb_list.to_gpl(String::from(palette_name));
    fs::write(palette_name, gimp_format).expect("Unable to write file");
}
