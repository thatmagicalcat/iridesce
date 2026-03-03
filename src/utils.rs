use macroquad::color::Color;

const GAMMA: f64 = 0.80;
const INTENSITY_MAX: f64 = 255.0;

/// Max wavelength is 780nm, MIN is 380nm
pub fn wavelength_to_rgb(wavelength: f64) -> Color {
    let (red, green, blue) = match wavelength {
        w if (380.0..440.0).contains(&w) => (-(w - 440.0) / (440.0 - 380.0), 0.0, 1.0),
        w if (440.0..490.0).contains(&w) => (0.0, (w - 440.0) / (490.0 - 440.0), 1.0),
        w if (490.0..510.0).contains(&w) => (0.0, 1.0, -(w - 510.0) / (510.0 - 490.0)),
        w if (510.0..580.0).contains(&w) => ((w - 510.0) / (580.0 - 510.0), 1.0, 0.0),
        w if (580.0..645.0).contains(&w) => (1.0, -(w - 645.0) / (645.0 - 580.0), 0.0),
        w if (645.0..781.0).contains(&w) => (1.0, 0.0, 0.0),
        _ => (0.0, 0.0, 0.0),
    };

    // Let the intensity fall off near the vision limits
    let factor = match wavelength {
        w if (380.0..420.0).contains(&w) => 0.3 + 0.7 * (w - 380.0) / (420.0 - 380.0),
        w if (420.0..701.0).contains(&w) => 1.0,
        w if (701.0..781.0).contains(&w) => 0.3 + 0.7 * (780.0 - w) / (780.0 - 700.0),
        _ => 0.0,
    };

    let adjust = |color: f64| -> f32 {
        (if color == 0.0 {
            0
        } else {
            (INTENSITY_MAX * (color * factor).powf(GAMMA)).round() as u8
        }) as f32
            / 255.0
    };

    Color::new(adjust(red), adjust(green), adjust(blue), 0.4)
}

pub trait ColorIntensity {
    fn intensity(&self, intensity: f32) -> Self;
}

impl ColorIntensity for Color {
    fn intensity(&self, intensity: f32) -> Self {
        let factor = (intensity as f64).clamp(0.0, 1.0);
        Color::new(
            ((self.r as f64) * factor) as _,
            ((self.g as f64) * factor) as _,
            ((self.b as f64) * factor) as _,
            self.a,
        )
    }
}
