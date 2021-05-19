mod color_list;
use std::env;
//use std::fmt::Write;
use std::collections::HashSet;
use std::fs;


//As a first argument it expects the name of the file for which you want the palette
//As a second argument is the name you want to give to the palette

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let palette_name = &args[2];


    let mut rgb_list = color_list::ColorList{colors_set: HashSet::new(),};
    rgb_list.add_file(String::from(filename));

    let gimp_format = rgb_list.to_gpl(String::from(palette_name));
    fs::write(palette_name, gimp_format).expect("Unable to write file");


 }