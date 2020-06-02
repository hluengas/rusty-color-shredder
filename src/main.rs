use image::{DynamicImage, GenericImage, ImageFormat, Rgba};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::path::Path;

fn main() {
    // generate random color list, track generation time
    let start = std::time::Instant::now();
    let color_list: Vec<Rgba<u8>> = generate_colors(10, false);
    let duration = start.elapsed();

    println!("{:#?}", duration);

    let mut img = DynamicImage::new_rgba8(32, 32);

    for x in 15..=17 {
        for y in 8..24 {
            img.put_pixel(x, y, color_list[x as usize]);
            img.put_pixel(y, x, color_list[y as usize]);
        }
    }

    let path = Path::new("./painting.png");
    match img.save_with_format(path, ImageFormat::Png) {
        Ok(result) => result,
        Err(e) => println!("Error saving image to disk: {}", e),
    };
}

fn generate_colors(color_bit_depth: u32, shuffle_colors: bool) -> Vec<Rgba<u8>> {
    // number of values per channel based on given color bit depth
    let values_per_channel = (2u32.pow(color_bit_depth) - 1) as u32;
    let max_values_per_channel_8bit = 255f32;
    let mut color_list: Vec<Rgba<u8>> = Vec::new();

    // create ranges for each channel
    let mut channel_1_list: Vec<u32> = (0..values_per_channel).collect();
    let mut channel_2_list: Vec<u32> = (0..values_per_channel).collect();
    let mut channel_3_list: Vec<u32> = (0..values_per_channel).collect();

    // shuffle channel range vectors
    channel_1_list.shuffle(&mut thread_rng());
    channel_2_list.shuffle(&mut thread_rng());
    channel_3_list.shuffle(&mut thread_rng());

    // loop over the entire color space:
    for c1_index in 0..values_per_channel {
        for c2_index in 0..values_per_channel {
            for c3_index in 0..values_per_channel {
                // true color float representation
                let rgba_float = [
                    channel_1_list[c1_index as usize] as f32 / values_per_channel as f32,
                    channel_2_list[c2_index as usize] as f32 / values_per_channel as f32,
                    channel_3_list[c3_index as usize] as f32 / values_per_channel as f32,
                    1f32,
                ];
                // truncate to 8bit-rgba
                let rgba_u8 = [
                    (rgba_float[0] * max_values_per_channel_8bit) as u8,
                    (rgba_float[1] * max_values_per_channel_8bit) as u8,
                    (rgba_float[2] * max_values_per_channel_8bit) as u8,
                    (rgba_float[3] * max_values_per_channel_8bit) as u8,
                ];
                // add this color to the color list
                color_list.push(Rgba(rgba_u8));
            }
        }
    }

    // final shuffle, removes channel sub-grouping
    if shuffle_colors {
        color_list.shuffle(&mut thread_rng());
    }
    return color_list;
}
