use image::{GrayImage, ImageFormat, Luma, Rgb, RgbImage};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
struct ImageConfig {
    size: (u32, u32),
    starting_positions: Vec<(u32, u32)>,
}
#[derive(Serialize, Deserialize, Clone)]
struct ColorConfig {
    use_hsv: bool,
    locked_channels: (bool, bool, bool),
    reference_colors: Vec<(u8, u8, u8)>,
}

struct PaintingData {
    image: RgbImage,
    boundry_region_image: GrayImage,
    boundry_region_list: Vec<(u32, u32)>,
}

fn main() {
    let image_config: ImageConfig = init_image_config();
    let color_config: ColorConfig = init_color_config();

    let mut canvas: PaintingData = PaintingData {
        image: RgbImage::new(image_config.size.0, image_config.size.1),
        boundry_region_image: GrayImage::new(image_config.size.0, image_config.size.1),
        boundry_region_list: Vec::new(),
    };

    init_canvas(&mut canvas, &image_config, &color_config);

    return;
}

fn init_canvas(canvas: &mut PaintingData, image_config: &ImageConfig, color_config: &ColorConfig) {
    let mut color_index: usize = 0;
    for index in 0..image_config.starting_positions.len() {
        // shift from 1 indexed to 0 indexed
        let x_coord = u32::clamp(
            image_config.starting_positions[index].0 - 1,
            0,
            image_config.size.0 - 1,
        );
        let y_coord = u32::clamp(
            image_config.starting_positions[index].1 - 1,
            0,
            image_config.size.1 - 1,
        );

        canvas.image.put_pixel(
            x_coord,
            y_coord,
            Rgb([
                color_config.reference_colors[color_index].0,
                color_config.reference_colors[color_index].1,
                color_config.reference_colors[color_index].2,
            ]),
        );

        for i in -1..2 {
            for j in -1..2 {
                let neighbor_x_coord: u32 =
                    i32::clamp(x_coord as i32 + i, 0, (image_config.size.0 - 1) as i32) as u32;
                let neighbor_y_coord: u32 =
                    i32::clamp(y_coord as i32 + j, 0, (image_config.size.1 - 1) as i32) as u32;

                canvas.boundry_region_image.put_pixel(
                    neighbor_x_coord,
                    neighbor_y_coord,
                    Luma([255u8]),
                );
                canvas
                    .boundry_region_list
                    .push((neighbor_x_coord, neighbor_y_coord));
            }
        }

        color_index = (color_index + 1) % color_config.reference_colors.len();
    }
    write_output_files(&canvas);
}

fn init_image_config() -> ImageConfig {
    let image_config_string = fs::read_to_string("./config/image.json")
        .expect("[ERROR] unable to read ./config/image.json");
    let mut image_config: ImageConfig = serde_json::from_str(&image_config_string)
        .expect("[ERROR] unable to parse ./config/image.json");

    image_config.starting_positions.sort();
    image_config.starting_positions.dedup();

    println!("\n====================:");
    println!("=== Image Config ===");
    println!("====================:");
    println!("Image Size:");
    println!("x: {}", image_config.size.0.to_string());
    println!("y: {}", image_config.size.1.to_string());
    println!("\nStarting Positions:");
    for position in image_config.clone().starting_positions {
        println!(
            "x: {}, y: {}",
            position.0.to_string(),
            position.1.to_string()
        );
    }

    return image_config;
}

fn init_color_config() -> ColorConfig {
    let color_config_string = fs::read_to_string("./config/color.json")
        .expect("[ERROR] unable to read ./config/color.json");

    let mut color_config: ColorConfig = serde_json::from_str(&color_config_string)
        .expect("[ERROR] unable to parse ./config/color.json");

    color_config.reference_colors.sort();
    color_config.reference_colors.dedup();

    println!("\n====================:");
    println!("=== Color Config ===");
    println!("====================:");
    println!("Use HSV: {}", color_config.use_hsv);
    println!("\nLocked Color Channels:");
    println!("Channel 1: {}", color_config.locked_channels.0);
    println!("Channel 2: {}", color_config.locked_channels.1);
    println!("Channel 3: {}", color_config.locked_channels.2);

    println!("\nReference Colors:");
    for color in color_config.clone().reference_colors {
        println!(
            "r: {}, g: {}, b: {}",
            color.0.to_string(),
            color.1.to_string(),
            color.2.to_string()
        );
    }

    return color_config;
}

fn write_output_files(canvas: &PaintingData) {
    // write the RGB painting file
    canvas
        .image
        .save_with_format("./output/painting.png", ImageFormat::Png)
        .unwrap();

    // write the boundry region image
    canvas
        .boundry_region_image
        .save_with_format("./output/boundry.png", ImageFormat::Png)
        .unwrap();
}
