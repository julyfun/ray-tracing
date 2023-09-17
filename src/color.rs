use crate::interval;
use crate::vec3;
use std::io::{self, Write};

const INTERVAL: interval::Interval = interval::Interval::from(0.000, 0.999);

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

// `输入 color 就行`
pub fn write_color<W: Write>(
    out: &mut W,
    pixel_color: &vec3::Color,
    samples_per_pixel: i32,
) -> io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = pixel_color.x() * scale;
    let g = pixel_color.y() * scale;
    let b = pixel_color.z() * scale;

    let (r, g, b) = (linear_to_gamma(r), linear_to_gamma(g), linear_to_gamma(b));

    writeln!(
        out,
        "{} {} {}",
        (256.0 * self::INTERVAL.clamp(r)) as i32,
        (256.0 * INTERVAL.clamp(g)) as i32,
        (256.0 * INTERVAL.clamp(b)) as i32,
    )
}
