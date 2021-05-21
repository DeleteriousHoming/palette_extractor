use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,

    pub h: f64,
    /*pub s: f64,*/
    pub v: f64,

    //A combination of hsv that yields a key upon which
    //ColorList can be sorted
    pub sort_key : (u32,u32,u32),
}

/*Implementing hash and partial eq by excluding hue from the implementations
r,g,b completely identifies the color, hue is only needed to sort the resulting
palette in a pleasing way*/

impl Hash for Rgb {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        (&self.r, &self.g, &self.g).hash(state);
    }
}

impl PartialEq for Rgb {
    fn eq(&self, other: &Self) -> bool {
        (&self.r, &self.g, &self.b) == (&other.r, &other.g, &other.b)
    }
}

impl Eq for Rgb {}

impl Rgb {
    fn hue_formula(c1: f64, c2: f64, delta: f64, summand: f64) -> f64 {
        (60.0 * ((c1 - c2) / delta) + summand) % 360.0
    }

    //Returns a tuple. Sorting ColorList on this tuple returns a cleaner and
    // tidier looking palette 
    //Repetitions should be put to 8 to dampen the noise of floating point values
    pub fn step(r :u8,
         g : u8,
         b : u8,
         h : f64,
         //s : f64,
         v : f64, 
         repetitions : u8) -> (u32,u32,u32) {

        let lum : f64 = (0.241 * r as f64 + 
            0.691 * g as f64 + 
            0.068 * b as f64).sqrt();

        let h2 : u32 = (h * repetitions as f64) as u32;
        let lum2 : u32 = (lum * repetitions as f64) as u32;
        let v2 : u32 = (v * repetitions as f64) as u32;
        
        (h2,lum2,v2)
    }

    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        let (h,v) = Rgb::calculate_hv(r, g, b);
        let key = Rgb::step(r, g, b, h, /*s,*/ v, 8);
        Rgb {
            r: r,
            g: g,
            b: b,

            h: h,
            /*s: s,*/
            v: v,
            sort_key: key,
        }
    }

    /*Calculates hue and value given rgb, saturation is not needed */
    pub fn calculate_hv(r: u8, g: u8, b: u8) -> (f64,f64) {
        let rp = r as f64 / 255.0;
        let gp = g as f64 / 255.0;
        let bp = b as f64 / 255.0;
        let mut hue: f64 = 0.0;
        //let s: f64;
        let v: f64;

        let cmax = rp.max(gp.max(bp)); //Maximum value out of r,g,b
        let cmin = rp.min(gp.min(bp));
        let delta = cmax - cmin;

        if cmax == cmin {
            hue = 0.0;
        } else if cmax == rp {
            hue = Rgb::hue_formula(gp, bp, delta, 360.0);
        } else if cmax == gp {
            hue = Rgb::hue_formula(bp, rp, delta, 120.0);
        } else if cmax == bp {
            hue = Rgb::hue_formula(rp, gp, delta, 240.0);
        }

        /*
        if cmax == 0.0 {
            s = 0.0;
        } else {
            s = (delta / cmax) * 100.0;
        }*/

        v = cmax * 100.0;

        (hue /*,s*/ ,v)
    }
}
