use image::{DynamicImage, GenericImage, GenericImageView, ImageFormat, Rgba};
use palette::{Hsl, Hsv, Srgb};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

struct Config {
    print_interval: Duration,
    color_bit_depth: u32,
    color_space: u8,      // 0, 1, 2 -- RGB, HSV, HSL
    group_by_channel: u8, // 1, 2, 3 -- 1st, 2nd, or 3rd Channel : only applies when shuffle is false
    shuffle_colors: bool,
    canvas_dimensions: (u32, u32),
    start_coordinates: (u16, u16),
    filename: &'static str,
}

impl Config {
    fn default() -> Config {
        return Config {
            print_interval: Duration::new(0, 500000000),
            color_bit_depth: 8,
            color_space: 0,
            group_by_channel: 1,
            shuffle_colors: true,
            canvas_dimensions: (128, 128),
            start_coordinates: (64, 64),
            filename: "painting",
        };
    }
}

fn main() {
    // load deafult config
    let configuration = Config::default();

    // intitialize counters
    let mut colored_pixel_count: u64 = 0;
    let mut previous_colored_pixel_count: u64 = 0;
    let mut color_list_index: usize = 0;

    // generate random color list
    let color_list: Vec<Rgba<u8>> = generate_colors(&configuration);

    // create list of available locations
    let mut available_list: HashMap<(u16, u16), (u16, u16)> = HashMap::new();

    // create dynamic image
    let mut canvas_rgba8 = DynamicImage::new_rgba8(
        configuration.canvas_dimensions.0,
        configuration.canvas_dimensions.1,
    );

    // place first pixel
    begin_painting(
        &color_list,
        &mut color_list_index,
        &mut available_list,
        &configuration,
        &mut canvas_rgba8,
        &mut colored_pixel_count,
    );

    // place until no more colors or positions are available
    continue_painting(
        &color_list,
        &mut color_list_index,
        &mut available_list,
        &configuration,
        &mut canvas_rgba8,
        &mut colored_pixel_count,
        &mut previous_colored_pixel_count,
    );

    // final print
    print_canvas(
        &configuration,
        &mut canvas_rgba8,
        &mut colored_pixel_count,
        &mut previous_colored_pixel_count,
    );
}

fn generate_colors(configuration: &Config) -> Vec<Rgba<u8>> {
    // time generation
    let start = std::time::Instant::now();
    // number of values per channel based on given color bit depth
    let values_per_channel = (2u32.pow(configuration.color_bit_depth as u32) - 1) as u32;
    let max_values_per_channel_8bit = 255f32;
    let mut color_list: Vec<Rgba<u8>> = Vec::new();

    // create ranges for each channel
    let mut channel_1_list: Vec<u32> = (0..values_per_channel).collect();
    let mut channel_2_list: Vec<u32> = (0..values_per_channel).collect();
    let mut channel_3_list: Vec<u32> = (0..values_per_channel).collect();

    // shuffle each channel range
    channel_1_list.shuffle(&mut thread_rng());
    channel_2_list.shuffle(&mut thread_rng());
    channel_3_list.shuffle(&mut thread_rng());

    // apply on offset for indexing into the color, this allows a channel to be selected as grouped when shuffle is off
    let channel_1 = ((2 + configuration.group_by_channel) % 3) as usize;
    let channel_2 = ((3 + configuration.group_by_channel) % 3) as usize;
    let channel_3 = ((4 + configuration.group_by_channel) % 3) as usize;

    // holds currently generated color, when used as a hue value is expected in degrees from -180f to +180f, all other uses expect from 0.0f to 1.0f
    let mut color_value = [0f32, 0f32, 0f32];
    let mut color_degree = [0f32, 0f32, 0f32];

    // loop over the entire color space:
    // loop over first channel
    for c1_index in 0..values_per_channel as usize {

        // set first channel values
        color_value[0] = channel_1_list[c1_index] as f32 / values_per_channel as f32;
        color_degree[0] = (color_value[0] - 0.5f32) * 360f32;

        // loop over second channel
        for c2_index in 0..values_per_channel as usize {
            
            // set second channel values
            color_value[1] = channel_2_list[c2_index] as f32 / values_per_channel as f32;
            color_degree[1] = (color_value[1] - 0.5f32) * 360f32;

            // loop over 3rd channel
            for c3_index in 0..values_per_channel as usize {
                
                // set third channel values
                color_value[2] = channel_3_list[c3_index] as f32 / values_per_channel as f32;
                color_degree[2] = (color_value[2] - 0.5f32) * 360f32;

                // interpret in various color spaces
                let color_hsv = Hsv::new(
                    color_degree[channel_1],
                    color_value[channel_2],
                    color_value[channel_3],
                );
                let color_hsl = Hsl::new(
                    color_degree[channel_1],
                    color_value[channel_2],
                    color_value[channel_3],
                );
                let mut color_srgb = Srgb::new(
                    color_value[channel_1],
                    color_value[channel_2],
                    color_value[channel_3],
                );

                // convert back to RGB 
                if configuration.color_space == 1 {
                    color_srgb = Srgb::from(color_hsv);
                } else if configuration.color_space == 2 {
                    color_srgb = Srgb::from(color_hsl);
                }

                // truncate to u8 and add alpha channel
                let color_truncated_srgba = [
                    (color_srgb.red * max_values_per_channel_8bit) as u8,
                    (color_srgb.green * max_values_per_channel_8bit) as u8,
                    (color_srgb.blue * max_values_per_channel_8bit) as u8,
                    255u8,
                ];

                // add this color to the color list
                color_list.push(Rgba(color_truncated_srgba));
            }
        }
    }

    // final shuffle, removes channel sub-grouping
    if configuration.shuffle_colors {
        color_list.shuffle(&mut thread_rng());
    }

    // print geeeneration time
    let duration = start.elapsed();
    println!("Colors generated in: {:#?}", duration);
    return color_list;
}

fn print_canvas(
    configuration: &Config,
    canvas_rgba8: &mut DynamicImage,
    colored_pixel_count: &mut u64,
    previous_colored_pixel_count: &mut u64,
) {
    // save image file
    let colors_placed_over_interval = *colored_pixel_count - *previous_colored_pixel_count;
    let path_string = ["./output/", configuration.filename, ".png"].concat();
    let path = Path::new(&path_string);
    *previous_colored_pixel_count = *colored_pixel_count;
    match canvas_rgba8.save_with_format(path, ImageFormat::Png) {
        Ok(result) => {
            println!("Painting Rate: {} pixels/sec", colors_placed_over_interval);
            result
        }
        Err(e) => println!("Error saving image to disk: {}", e),
    };
}

fn begin_painting(
    color_list: &Vec<Rgba<u8>>,
    color_list_index: &mut usize,
    available_list: &mut HashMap<(u16, u16), (u16, u16)>,
    configuration: &Config,
    canvas_rgba8: &mut DynamicImage,
    colored_pixel_count: &mut u64,
) {
    // get starting color
    let start_color: Rgba<u8> = color_list[*color_list_index];
    *color_list_index += 1;

    // paint first pixel
    available_list.insert(
        configuration.start_coordinates,
        configuration.start_coordinates,
    );
    paint_pixel(
        &configuration.start_coordinates,
        &start_color,
        canvas_rgba8,
        available_list,
        colored_pixel_count,
    );
}

fn continue_painting(
    color_list: &Vec<Rgba<u8>>,
    color_list_index: &mut usize,
    available_list: &mut HashMap<(u16, u16), (u16, u16)>,
    configuration: &Config,
    canvas_rgba8: &mut DynamicImage,
    colored_pixel_count: &mut u64,
    previous_colored_pixel_count: &mut u64,
) {
    let mut start = std::time::Instant::now();
    // while there are available positions and colors to be placed
    while !available_list.is_empty() && *color_list_index < color_list.len() {
        // select color
        let target_color = color_list[*color_list_index];
        *color_list_index += 1;

        // get position
        let target_coordinates =
            get_best_position_for_color(&target_color, canvas_rgba8, available_list);
        // paint pixel
        paint_pixel(
            &target_coordinates,
            &target_color,
            canvas_rgba8,
            available_list,
            colored_pixel_count,
        );

        // print if interval surpassed
        let duration = start.elapsed();
        if duration > configuration.print_interval {
            print_canvas(
                configuration,
                canvas_rgba8,
                colored_pixel_count,
                previous_colored_pixel_count,
            );
            start = std::time::Instant::now();
        }
    }
}

fn paint_pixel(
    target_coordinate: &(u16, u16),
    target_color: &Rgba<u8>,
    canvas: &mut DynamicImage,
    available_list: &mut HashMap<(u16, u16), (u16, u16)>,
    colored_pixel_count: &mut u64,
) {
    // check availability
    if !available_list.contains_key(&target_coordinate) {
        return;
    }
    // paint pixel
    canvas.put_pixel(
        target_coordinate.0 as u32,
        target_coordinate.1 as u32,
        *target_color,
    );
    available_list.remove(&target_coordinate);
    *colored_pixel_count += 1;

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
            if !canvas.in_bounds(neighbor_coordinates.0 as u32, neighbor_coordinates.1 as u32) {
                continue;
            }
            // skip already colored
            let neighbor_color: Rgba<u8> =
                canvas.get_pixel(neighbor_coordinates.0 as u32, neighbor_coordinates.1 as u32);
            if neighbor_color != Rgba([0, 0, 0, 0]) {
                continue;
            }
            // pixel must be available if not skipped so far
            available_list.insert(neighbor_coordinates, neighbor_coordinates);
        }
    }
}

fn get_best_position_for_color(
    target_color: &Rgba<u8>,
    canvas: &DynamicImage,
    available_list: &HashMap<(u16, u16), (u16, u16)>,
) -> (u16, u16) {
    let mut min_color_difference: u64 = u64::max_value();
    let mut best_position: (u16, u16) = (u16::max_value(), u16::max_value());

    // loop over every available position
    for available_coordinates in available_list.keys() {
        let mut color_difference_sum: u64 = 0;
        let mut neighbor_count: u64 = 0;
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
                if !canvas.in_bounds(neighbor_coordinates.0 as u32, neighbor_coordinates.1 as u32) {
                    continue;
                }
                // skip un-colored
                let neighbor_color: Rgba<u8> =
                    canvas.get_pixel(neighbor_coordinates.0 as u32, neighbor_coordinates.1 as u32);
                if neighbor_color == Rgba([0, 0, 0, 0]) {
                    continue;
                }

                //compute color diiference
                for i in 0..4 {
                    color_difference_sum +=
                        (target_color[i] as i32 - neighbor_color[i] as i32).pow(2) as u64;
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
