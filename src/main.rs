use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat, Rgba};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

fn main() {
    // save image every print_interval
    let print_interval = Duration::new(1, 0);
    // generate random color list, track generation time
    let mut start = std::time::Instant::now();
    let color_list: Vec<Rgba<u8>> = generate_colors(8, true);
    let mut color_list_index: usize = 0;
    let mut duration = start.elapsed();
    println!("Colors generated in: {:#?}", duration);

    // create list of available locations
    let mut available_list: HashMap<(u32, u32), (u32, u32)> = HashMap::new();

    // create dynamic image
    let mut canvas_rgba8 = DynamicImage::new_rgba8(256, 256);

    // get starting pixel
    let start_coord: (u32, u32) = (0, 0);
    let start_color: Rgba<u8> = color_list[color_list_index];
    color_list_index += 1;
    available_list.insert(start_coord, start_coord);

    // paint first pixel
    paint_pixel(
        start_coord,
        start_color,
        &mut canvas_rgba8,
        &mut available_list,
    );

    start = std::time::Instant::now();
    // while there are available positions and colors to be placed
    while !available_list.is_empty() && color_list_index < color_list.len() {
        // select color
        let target_color = color_list[color_list_index];
        color_list_index += 1;

        // get position
        let target_coordinates =
            get_best_position_for_color(target_color, &mut canvas_rgba8, &mut available_list);
        // paint pixel
        paint_pixel(
            target_coordinates,
            target_color,
            &mut canvas_rgba8,
            &mut available_list,
        );

        // print if interval surpassed
        duration = start.elapsed();
        if duration > print_interval {
            // save image file
            let path = Path::new("./output/painting.png");
            match canvas_rgba8.save_with_format(path, ImageFormat::Png) {
                Ok(result) => result,
                Err(e) => println!("Error saving image to disk: {}", e),
            };
            start = std::time::Instant::now();
        }
    }
    // final print
    let path = Path::new("./output/painting.png");
    match canvas_rgba8.save_with_format(path, ImageFormat::Png) {
        Ok(result) => result,
        Err(e) => println!("Error saving image to disk: {}", e),
    };
}

fn paint_pixel(
    target_coordinate: (u32, u32),
    target_color: Rgba<u8>,
    canvas: &mut DynamicImage,
    available_list: &mut HashMap<(u32, u32), (u32, u32)>,
) {
    // check availability
    if !available_list.contains_key(&target_coordinate) {
        return;
    }
    // paint pixel
    canvas.put_pixel(target_coordinate.0, target_coordinate.1, target_color);
    available_list.remove(&target_coordinate);

    // mark available neighbors
    for i in 0..3 {
        for j in 0..3 {
            // calculate neighbor coordinates
            let neighbor_coordinates =
                ((target_coordinate.0 - 1 + i), (target_coordinate.1 - 1 + j));
            // skip self
            if i == 1 && j == 1 {
                continue;
            }
            // skip out-of-bounds
            if !canvas.in_bounds(neighbor_coordinates.0, neighbor_coordinates.1) {
                continue;
            }
            // skip already colored
            let neighbor_color: Rgba<u8> =
                canvas.get_pixel(neighbor_coordinates.0, neighbor_coordinates.1);
            if neighbor_color != Rgba([0, 0, 0, 0]) {
                continue;
            }
            // pixel must be available if not skipped so far
            available_list.insert(neighbor_coordinates, neighbor_coordinates);
        }
    }
}

fn get_best_position_for_color(
    target_color: Rgba<u8>,
    canvas: &mut DynamicImage,
    available_list: &mut HashMap<(u32, u32), (u32, u32)>,
) -> (u32, u32) {
    let mut min_color_difference: u32 = u32::max_value();
    let mut best_position: (u32, u32) = (u32::max_value(), u32::max_value());

    // loop over every available position
    for available_coordinates in available_list.keys() {
        let mut color_difference_sum: u32 = 0;
        let mut neighbor_count: u32 = 0;
        // loop over neighbors
        for i in 0..3 {
            for j in 0..3 {
                // calculate neighbor coordinates
                let neighbor_coordinates = (
                    (available_coordinates.0 - 1 + i),
                    (available_coordinates.1 - 1 + j),
                );
                // skip self
                if i == 1 && j == 1 {
                    continue;
                }
                // skip out-of-bounds
                if !canvas.in_bounds(neighbor_coordinates.0, neighbor_coordinates.1) {
                    continue;
                }
                // skip un-colored
                let neighbor_color: Rgba<u8> =
                    canvas.get_pixel(neighbor_coordinates.0, neighbor_coordinates.1);
                if neighbor_color == Rgba([0, 0, 0, 0]) {
                    continue;
                }

                //compute color diiference
                for i in 0..4 {
                    color_difference_sum +=
                        (target_color[i] as i32 - neighbor_color[i] as i32).pow(2) as u32;
                }
                neighbor_count += 1;
            }
        }
        let avg_color_difference = color_difference_sum / neighbor_count;
        if avg_color_difference < min_color_difference {
            min_color_difference = avg_color_difference;
            best_position = *available_coordinates;
        }
    }
    return best_position;
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
