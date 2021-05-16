use std::collections::HashSet;
use std::env;
use std::fmt::Write;
use std::fs;

//As a first argument it expects the name of the file for which you want the palette
//As a second argument is the name you want to give to the palette

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let palette_name = &args[2];

    let img = image::open(filename)
    .expect("Could not read image")
    .to_rgb8();

    //Using a set so we don't insert the same color twice
    let mut colors: HashSet<String> = HashSet::new();

    //Gets values
    let raw = img.into_raw();
    let mut rgb = String::from(""); // the R G B value

    let mut i = 0;
    for val in raw {
        write!(rgb, "{}\t", val).expect("Could not convert");
        i = i + 1;

        if i == 3 {
            colors.insert(rgb);
            rgb = String::from("");
            i = 0;
        }
    }

    let mut output = format!("GIMP Palette\n#Palette Name: {}\n#Colors: {}\n"
                            ,palette_name, colors.len());

    for color in colors {
        let hex = rgb_to_hex(&color);
        writeln!(output, "{}\t{}", color, hex).expect("Could not create line of rgb and hex");
    }

    println!("{}", output);
    let palette_name = format!("{}.gpl",palette_name);
    fs::write(palette_name, output).expect("Unable to write file");
}


fn rgb_to_hex(rgb: &String) -> String {
    let split = rgb.split("\t");
    let mut hex = String::from("");

    //Iterating over r, g and b
    for col in split {
        if col == "" {
            continue;
        }
        write!(
            hex,
            "{:x}",
            col.parse::<u32>().expect("Could not convert rgb to hex.")
        )
        .expect("Could not write hex line.");
    }

    hex
}