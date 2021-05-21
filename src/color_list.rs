/*This module is for storing a set of rgbs */
mod rgb;
use std::collections::HashSet;
use std::fmt::Write;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct ColorList {
    colors_set: HashSet<rgb::Rgb>,
    //colors_vec: Vec<rgb::Rgb>,
}

impl ColorList {
    pub fn new() -> ColorList {
        ColorList {
            colors_set: HashSet::new(),
            //colors_vec: Vec::new(),
        }
    }

    //Receives a filename and adds all of the image's colors to the hash set
    pub fn add_file(&mut self, filename: String) -> () {
        let img = image::open(filename)
            .expect("Could not read image")
            .to_rgb8();

        //Gets values
        let raw = img.into_raw();

        let mut i = 0; //Helps in counting to three
        let mut rgb_arr = [0, 0, 0]; //Holds values until the rgb array is complete
        for val in raw {
            rgb_arr[i] = val;
            i = i + 1;

            //Every third value we create a new Rgb item and add it to the hash
            if i == 3 {
                self.colors_set.insert(rgb::Rgb::new(
                    rgb_arr[0],
                    rgb_arr[1],
                    rgb_arr[2]
                ));
                i = 0;
            }
        }
    }

    /*Transorms the current hash set into a gpl format and returns the gpl string
    --name is the name that the palette will be given*/

    pub fn to_gpl(&mut self, name: String) -> String {
        let colors_len = self.colors_set.len();
        let set_clone = self.colors_set.clone();
        let mut vec = Vec::from_iter(set_clone.into_iter());
        vec.sort_by(|a,b| a.sort_key.cmp(&b.sort_key));

        let mut gpl = String::from("");
        write!(
            gpl,
            "GIMP Palette\nName: {}\n#Colors: {}\n",
            name, colors_len
        )
        .expect("Error in converting color list into gpl format");

        for (i, rgb) in vec.iter().enumerate() {
            write!(gpl, "{}\t{}\t{}\t#{}\n", rgb.r, rgb.g, rgb.b, i)
                .expect("Could note create gpl line during color_list to_gpl conversion.");
        }

        return gpl;
    }
}
