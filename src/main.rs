use image::{ImageFormat, Rgb, RgbImage};
use rand::random;

struct Painting {
    image: RgbImage,
    boundry_region: Vec<Coordinate>,
    starting_locations: Vec<Coordinate>,
    canvas_constraints: Constraints,
    canvas_stats: Stats,
}
struct Constraints {
    x_size: u32,
    y_size: u32,
}
struct Stats {
    total_pixel_count: u64,
    current_pixels_placed_count: u64,
    previous_pixels_placed_count: u64,
}
struct Pixel {
    position: Coordinate,
    color: Rgb<u8>,
}
struct Coordinate {
    x: u32,
    y: u32,
}

fn main() {
    let working_constraints: Constraints = Constraints {
        x_size: 128u32,
        y_size: 128u32,
    };
    let mut working_stats: Stats = Stats {
        total_pixel_count: (128u64 * 128u64),
        current_pixels_placed_count: 0u64,
        previous_pixels_placed_count: 0u64,
    };
    let mut working_canvas: Painting = Painting {
        image: RgbImage::new(working_constraints.x_size, working_constraints.y_size),
        canvas_constraints: working_constraints,
        canvas_stats: working_stats,
        boundry_region: Vec::new(),
        starting_locations: vec![
            Coordinate { x: 0, y: 0 },
            Coordinate {
                x: working_constraints.x_size - 1,
                y: working_constraints.y_size - 1,
            },
        ],
    };

    for index in 0..working_canvas.starting_locations.len() {
        place_pixel(&mut working_canvas)
    }

    canvas
        .save_with_format("./output/painting.png", ImageFormat::Png)
        .unwrap();

    while boundry_region.len() > 0 {
        let target_color = Rgb([random::<u8>(), random::<u8>(), random::<u8>()]);
        let best_position = get_best_position_for_color();
        place_pixel();
        if current_pixels_placed_count % 50 == 0 {
            canvas
                .save_with_format("./output/painting.png", ImageFormat::Png)
                .unwrap();
        }
    }
    canvas
        .save_with_format("./output/painting.png", ImageFormat::Png)
        .unwrap();
}

fn place_pixel(target: Pixel, working_canvas: &mut Painting) {

    working_canvas.image.put_pixel(target.x, target.y, rgb_pixel);
    *current_pixels_placed_count += 1;
    for i in 0..3 {
        'inner: for j in 0..3 {
            // skip self
            if i == 1 && j == 1 {
                continue;
            }
            // prevent less than zero out-of-bounds
            if x_coord == 0 && i == 0 {
                continue;
            }
            if y_coord == 0 && j == 0 {
                continue;
            }
            // prevent greater than dimensions out-of-bounds
            if x_coord == (x_dim - 1) && i == 2 {
                continue;
            }
            if y_coord == (y_dim - 1) && j == 2 {
                continue;
            }

            let neighbor_x_coord: u32 = x_coord + i - 1;
            let neighbor_y_coord: u32 = y_coord + j - 1;

            for index in 0..boundry_region.len() {
                if boundry_region[index] == (neighbor_x_coord, neighbor_y_coord) {
                    continue 'inner;
                }
            }

            let neighbor_color: Rgb<u8> = *canvas.get_pixel(neighbor_x_coord, neighbor_y_coord);

            if neighbor_color == Rgb([0, 0, 0]) {
                // println!(
                //     "Adding {},{} from boundry region.",
                //     neighbor_x_coord, neighbor_y_coord
                // );
                boundry_region.push((neighbor_x_coord, neighbor_y_coord));
            }
        }
    }
}

fn get_best_position_for_color(
    target_color: &Rgb<u8>,
    canvas: &RgbImage,
    x_dim: u32,
    y_dim: u32,
    boundry_region: &mut Vec<(u32, u32)>,
) -> (u32, u32) {
    let mut min_color_difference: f32 = f32::MAX;
    let mut best_position_index: usize = 0;

    // loop over every available position
    for index in 0..boundry_region.len() {
        let mut color_difference_sum: f32 = 0f32;
        let mut neighbor_count: u64 = 0;
        let x_coord: u32 = boundry_region[index].0;
        let y_coord: u32 = boundry_region[index].1;

        // loop over neighbors
        for i in 0..3 {
            for j in 0..3 {
                // skip self
                if i == 1 && j == 1 {
                    continue;
                }
                // prevent less than zero out-of-bounds
                if x_coord == 0 && i == 0 {
                    continue;
                }
                if y_coord == 0 && j == 0 {
                    continue;
                }
                // prevent greater than dimensions out-of-bounds
                if x_coord == (x_dim - 1) && i == 2 {
                    continue;
                }
                if y_coord == (y_dim - 1) && j == 2 {
                    continue;
                }

                let neighbor_x_coord: u32 = x_coord + i - 1;
                let neighbor_y_coord: u32 = y_coord + j - 1;

                // skip un-colored
                let neighbor_color: Rgb<u8> = *canvas.get_pixel(neighbor_x_coord, neighbor_y_coord);
                if neighbor_color == Rgb([0, 0, 0]) {
                    continue;
                }

                //compute color diiference
                for i in 0..3 {
                    color_difference_sum +=
                        (target_color[i] as f32 - neighbor_color[i] as f32).powf(2f32) as f32;
                }
                neighbor_count += 1;
            }
        }

        let avg_color_difference = color_difference_sum / neighbor_count as f32;
        if avg_color_difference < min_color_difference {
            min_color_difference = avg_color_difference;
            best_position_index = index;
        } else if avg_color_difference == min_color_difference && random::<bool>() {
            min_color_difference = avg_color_difference;
            best_position_index = index;
        }
    }

    if boundry_region.len() > 1 {
        return boundry_region.swap_remove(best_position_index);
    } else {
        return boundry_region.remove(best_position_index);
    }
}
