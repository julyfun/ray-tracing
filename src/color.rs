use crate::vec3;

// `输入 color 就行`
pub fn write_color(color: vec3::Color) {
    let (ir, ig, ib) = (
        (255.999 * color.x()) as i32,
        (255.999 * color.y()) as i32,
        (255.999 * color.z()) as i32,
    );
    println!("{} {} {}", ir, ig, ib);
}
