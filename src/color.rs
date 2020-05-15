use std::f32;

pub enum DefinedColors {
    RED,
    CYAN,
}

pub struct Color {
    hue: i16, //<- 0-360
    saturation: f32,
    lightness: f32,
}

impl Color {
    pub fn new(color: DefinedColors) -> Color {
        Color {
            hue: match color {
                DefinedColors::RED => 180,
                DefinedColors::CYAN => 0,
            },
            saturation: 1.,
            lightness: 1.,
        }
    }

    pub fn to_rgb(&self) -> [f32; 4] {
        let chroma: f32 = (1. - (2. * self.lightness).abs()) * self.saturation;
        let h1: f32 = f32::from(self.hue) / 60.;
        let x = chroma * (1. - (h1 % 2. - 1.).abs());
        let bottom_rgb = match h1 {
            _ if h1 >= 0. && h1 <= 1. => [chroma, x, 0.],
            _ if h1 >= 1. && h1 <= 2. => [x, chroma, 0.],
            _ if h1 >= 2. && h1 <= 3. => [0., chroma, x],
            _ if h1 >= 3. && h1 <= 4. => [0., x, chroma],
            _ if h1 >= 4. && h1 <= 5. => [x, 0., chroma],
            _ if h1 >= 5. && h1 <= 6. => [chroma, 0., x],
            _ => [0., 0., 0.],
        };
        let m: f32 = self.lightness - chroma / 2.;
        [bottom_rgb[0] + m, bottom_rgb[1] + m, bottom_rgb[2] + m, 1.0]
    }
}
