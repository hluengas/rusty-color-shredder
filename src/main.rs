struct ImageConfig {
    x_size: u16,
    y_size: u16,
    starting_positions: Vec<Coordinate>,
}
struct ColorConfig {
    use_hsv: bool,
    r: ColorChannelConfig,
    g: ColorChannelConfig,
    b: ColorChannelConfig,
    reference_colors: Vec<Color>,
}
struct Coordinate {
    x: u16,
    y: u16,
}
struct Color {
    r: u8,
    g: u8,
    b: u8,
}
struct ColorChannelConfig {
    locked: bool,
    min: f32,
    max: f32,
}

fn main() {
    return;
}
