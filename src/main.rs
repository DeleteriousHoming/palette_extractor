use std::fmt::Write;
use std::collections::HashSet;
use std::fs;

fn rgb_to_hex(rgb : String) -> (String,String) {
    let split = rgb.split("\t");
    let mut hex = String::from("");

    //Iterating over r, g and b
    for col in split{
        if col == "" {
            continue;
        }
        write!(hex,"{:x}",col.parse::<u32>()
                            .expect("Could not convert to hex"));
    }

    (rgb,hex)
}

fn main() {
    let img = image::open("../test.png")
                .expect("Could not read image")
                .to_rgb8();

    //Using a set so we don't insert the same color twice
    let mut colors : HashSet<String> = HashSet::new();

    //Gets values
    let raw = img.into_raw();
    let mut rgb = String::from(""); // the R G B value

    let mut i = 0;
    for val in raw {
        
        write!(rgb, "{}\t", val).expect("Could not convert");
        i = i+1;

        if i==3 {
            colors.insert(rgb); 
            rgb = String::from("");
            i=0;
        }
    }
    
    let mut output = String::from("GIMP Palette\n");

    for color in colors{
        let (color,hex) = rgb_to_hex(color);
        writeln!(output,"{}\t{}",color,hex);
    }

    println!("{}",output);
    fs::write("./palette.gpl",output).expect("Unable to write file");


}
