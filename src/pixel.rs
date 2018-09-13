extern crate num_traits;

use std::fmt::UpperHex;
use self::num_traits::{AsPrimitive, Unsigned, Bounded, Zero, One};
use std::marker::Sized;

#[derive(Clone)]
pub struct Pixel<T: Unsigned + Bounded> {
    r: T,
    g: T,
    b: T,
    a: T
}

pub trait PixelMath<T: 'static +  Unsigned + Bounded + Copy> {

    fn default() -> Self;
    fn from_hsb(hue: f64, saturation: f64, brightness: f64) -> Result<Self, String>
        where Self: Sized, f64: From<T> + AsPrimitive<T>, T: Into<f64>;

    fn new(r: T, g: T, b: T) -> Self;
    fn new_rgba(r: T, g: T, b: T, a: T) -> Self;

    fn set_alpha(&mut self, a: T) -> &Self;
    fn set_r(&mut self, r: T) -> &Self;
    fn set_g(&mut self, g: T) -> &Self;
    fn set_b(&mut self, b: T) -> &Self;

    fn set_rgb(&mut self, r: T, g: T, b: T) -> &Self;
    fn set_rgba(&mut self, r: T, g: T, b: T, a: T) -> &Self;

    fn get_tuple(&self) -> (T, T, T, T);
    fn get_vector(&self) -> Vec<T>;
    fn get_slice(&self) -> [T; 4];

    fn to_hex(&self) -> String;
    fn to_hsv(&self) -> (T, T, T);
}

impl<T: 'static + Unsigned + Bounded + UpperHex + Zero + One + Copy + Into<f64>> PixelMath<T> for Pixel<T> {
    
    fn default() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    /// hue is in degrees, saturation and brightness are between 0 and 1
    fn from_hsb(hue: f64, saturation: f64, brightness: f64) -> Result<Self, String>
        where f64: From<T> + AsPrimitive<T>, T: Into<f64>
    {
        if saturation > 1f64 || brightness > 1f64 {
            return Err(String::from(format!("Invalid HSB values: {} {} {}", hue, saturation, brightness)));
        }

        let hh: f64; let p: f64; let q: f64; let t: f64; let ff: f64; let v: f64;
        let i: u64;
        // let r: f64, g: f64, b: f64;

        let (r, g, b) = if saturation <= 0f64 {
            (brightness, brightness, brightness)
        } else {
            hh = (hue % 360f64) / 60f64;
            // hh = if hue >= 360f64 { 0f64 } else { (hue/60f64) /*% 1f64*/ };
            // hh = (hue - hue.floor()) * 6f64;
            i = hh as u64;
            // ff = hh - (i as f64);
            ff = hh - hh.floor();
            p = brightness * (1f64 - saturation);
            q = brightness * (1f64 - (saturation * ff));
            t = brightness * (1f64 - (saturation * (1f64 - ff)));
            v = brightness;

            match i {
                0 => (v, t, p),
                1 => (q, v, p),
                2 => (p, v, t),
                3 => (p, q, v),
                4 => (t, p, v),
                _ => (v, p, q),
            }
        };

        let max = T::max_value().into();

        Ok(Self::new(
            (r * max).round().as_(),
            (g * max).round().as_(),
            (b * max).round().as_()))



        // fn dim_curve(x: f64) -> f64 {
        //     2.0f64.powf((x + 64f64) / 40f64 - 1f64)
        // }

        // let (max, min) = (T::max_value(), T::min_value());

        // let val = dim_curve(brightness * max.into());
        // let sat: f64 = min.into() - dim_curve((1f64 - saturation) * max.into());

        // let (r, g, b, base);

        // let (r, g, b) = if sat < T::one().into() {
        //     (val.into(), val.into(), val.into())
        // } else {

        //     let base = (((max.into() - sat) * val) as u64 >> 8) as f64;

        //     match (hue * 256f64 / 60f64).floor() {
        //         0f64 => (val.into(), ((((val - base) * hue) / 60.0f64) + base).into(), base.into()),
        //         1f64 => ()
        //         2f64 =>
        //         3f64 =>
        //         4f64 =>
        //         _ => 
        //     }
        // };

        // // = (T::min_value(), T::min_value(), T::min_value(), T::min_value());

        // // r = T::max_value();
        // // g = T::max_value();
        // // b = T::max_value();

        // Ok(Self::new(r, g, b))

        // let c = (1.0 - (2.0 * ))
    }

    fn new(r: T, g: T, b: T) -> Self {
        Self::new_rgba(r, g, b, T::max_value())
    }

    fn new_rgba(r: T, g: T, b: T, a: T) -> Self {
        Pixel{r, g, b, a}
    }

    fn set_alpha(&mut self, a: T) -> &Self {
        self.a = a;
        self
    }

    fn set_r(&mut self, r: T) -> &Self {
        self.r = r;
        self
    }

    fn set_g(&mut self, g: T) -> &Self {
        self.g = g;
        self
    }

    fn set_b(&mut self, b: T) -> &Self {
        self.b = b;
        self
    }


    fn set_rgb(&mut self, r: T, g: T, b: T) -> &Self {
        self.r = r;
        self.g = g;
        self.b = b;

        self
    }

    fn set_rgba(&mut self, r: T, g: T, b: T, a: T) -> &Self {
        self.set_rgb(r, g, b);
        self.a = a;

        self
    }


    fn get_tuple(&self) -> (T, T, T, T) {
        (self.r, self.g, self.b, self.a)
    }

    fn get_vector(&self) -> Vec<T> {
        vec![self.r, self.g, self.b, self.a]
    }

    fn get_slice(&self) -> [T; 4] {
        [self.r, self.g, self.b, self.a]
    }


    fn to_hex(&self) -> String {
        format!("{:#X}{:X}{:X}{:X}", self.r, self.g, self.b, self.a)
    }

    fn to_hsv(&self) -> (T, T, T) {
        unimplemented!();
    }
}


pub struct IntoPixel<'a, T: 'a + Unsigned + Bounded> {
    px: &'a Pixel<T>,
    remaining: u8,
}

impl<'a, T: Unsigned + Bounded> IntoPixel<'a, T> {
    pub fn new(px: &'a Pixel<T>) -> Self {
        IntoPixel { px: &px, remaining: 5 }
    }
}

impl<'a, T: Unsigned + Bounded + Copy> Iterator for IntoPixel<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.remaining -= 1;

        match self.remaining {
            r @ 1 ... 4 => Some(
                match r {
                    4 => self.px.r,
                    3 => self.px.g,
                    2 => self.px.b,
                    _ => self.px.a,
                }),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use ::pixel::{Pixel, PixelMath, IntoPixel};

    #[test]
    fn pixel_iterator() {
        let px = Pixel::new(9u8, 234, 5);

        let iter = IntoPixel::new(&px);

        for i in iter {
            println!("{:?}", i);
        }

    }

    fn test_hsb_to_rgb(h: f64, s: f64, v: f64, r: u8, g: u8, b: u8) {
        assert_eq!((r, g, b, 255u8), Pixel::<u8>::from_hsb(h, s, v).unwrap().get_tuple())
    }

    #[test]
    fn from_hsb() {
        test_hsb_to_rgb(360.0, 1.0, 1.0, 255, 0, 0);
        test_hsb_to_rgb(250.0, 1.0, 1.0, 43, 0, 255);
        test_hsb_to_rgb(360.0, 0.5, 0.5, 128, 64, 64);
        test_hsb_to_rgb(200.0, 0.5, 0.5, 64, 106, 128);
        test_hsb_to_rgb(0.0, 0.5, 0.5, 128, 64, 64);
        test_hsb_to_rgb(0.0, 0.0, 0.5, 128, 128, 128);
        test_hsb_to_rgb(0.0, 0.5, 0.0, 0, 0, 0);
        test_hsb_to_rgb(0.0, 0.0, 0.0, 0, 0, 0);
    }
}