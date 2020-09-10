use crate::{ Rgb, FromRgb, ToRgb, Cmy };

#[derive(Copy, Clone, Debug, Default)]
pub struct Cmyk {
    pub c: f64,
    pub m: f64,
    pub y: f64,
    pub k: f64,
}

impl Cmyk {
    pub fn new(c: f64, m: f64, y: f64, k: f64) -> Self {
        Self { c, m, y, k }
    }
}

impl FromRgb for Cmyk {
    fn from_rgb(rgb: &Rgb) -> Self {
        let cmy = Cmy::from_rgb(rgb);
        let k = cmy.c.min(cmy.m.min(cmy.y.min(1.0)));
        match (k - 1.0) < 1e-3 {
            true => Self::new(0.0, 0.0, 0.0, k),
            false => Self::new(
                (cmy.c - k) / (1.0 - k),
                (cmy.m - k) / (1.0 - k),
                (cmy.y - k) / (1.0 - k),
                k
            )
        }
    }
}

impl ToRgb for Cmyk {
    fn to_rgb(&self) -> Rgb {
        Cmy::new(
            self.c * (1.0 - self.k) + self.k,
            self.m * (1.0 - self.k) + self.k,
            self.y * (1.0 - self.k) + self.k
        ).to_rgb()
    }
}