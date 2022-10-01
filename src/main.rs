use image::{GrayImage, ImageFormat, Luma, Rgb, RgbImage};
use rand::random;
use std::time::Instant;

struct Painting {
    image: RgbImage,
    boundry_region_image: GrayImage,
    boundry_region_list: Vec<Coordinate>,
    starting_locations: Vec<Coordinate>,
    canvas_constraints: Constraints,
    canvas_stats: Stats,
}
struct Constraints {
    x_size: u32,
    y_size: u32,
}
struct Stats {
    current_pixels_placed_count: u64,
    // previous_pixels_placed_count: u64,
    // total_pixel_count: u64,
}
struct Pixel {
    position: Coordinate,
    color: Rgb<u8>,
}
#[derive(Copy, Clone)]
struct Coordinate {
    x: u32,
    y: u32,
}

fn main() {
    let working_constraints: Constraints = Constraints {
        x_size: 256u32,
        y_size: 256u32,
    };
    let working_stats: Stats = Stats {
        // total_pixel_count: (working_constraints.x_size as u64 * working_constraints.y_size as u64),
        current_pixels_placed_count: 0u64,
        // previous_pixels_placed_count: 0u64,
    };
    let mut starting_points = Vec::new();
    for i in 0..working_constraints.x_size {
        if i % 64 == 0 {
            starting_points.push(Coordinate { x: 0, y: i });
            starting_points.push(Coordinate { x: working_constraints.x_size - 1, y: i });

            starting_points.push(Coordinate { x: i, y: 0 });
            starting_points.push(Coordinate { x: i, y: working_constraints.x_size - 1 });
        }
    }
    let mut working_canvas: Painting = Painting {
        image: RgbImage::new(working_constraints.x_size, working_constraints.y_size),
        boundry_region_image: GrayImage::new(
            working_constraints.x_size,
            working_constraints.y_size,
        ),
        // starting_locations: vec![Coordinate {
        //     x: working_constraints.x_size / 2u32,
        //     y: working_constraints.y_size / 2u32,
        // }],
        starting_locations: starting_points,
        canvas_constraints: working_constraints,
        canvas_stats: working_stats,
        boundry_region_list: Vec::new(),
    };

    for index in 0..working_canvas.starting_locations.len() {
        let target_pixel = Pixel {
            position: working_canvas.starting_locations[index].clone(),
            color: Rgb([random::<u8>(), random::<u8>(), random::<u8>()]),
        };
        place_pixel(&target_pixel, &mut working_canvas)
    }

    working_canvas
        .image
        .save_with_format("./output/painting.png", ImageFormat::Png)
        .unwrap();

    working_canvas
        .boundry_region_image
        .save_with_format("./output/boundry.png", ImageFormat::Png)
        .unwrap();

    let mut current_time = Instant::now();
    while working_canvas.boundry_region_list.len() > 0 {
        let target_color = Rgb([random::<u8>(), random::<u8>(), random::<u8>()]);
        let target_pixel = get_best_position_for_color(target_color, &mut working_canvas);

        place_pixel(&target_pixel, &mut working_canvas);

        if current_time.elapsed().as_secs_f32() > (1f32 / 2f32) {
            current_time = Instant::now();
            working_canvas
                .image
                .save_with_format("./output/painting.png", ImageFormat::Png)
                .unwrap();

            working_canvas
                .boundry_region_image
                .save_with_format("./output/boundry.png", ImageFormat::Png)
                .unwrap();
        }
    }
    working_canvas
        .image
        .save_with_format("./output/painting.png", ImageFormat::Png)
        .unwrap();

    working_canvas
        .boundry_region_image
        .save_with_format("./output/boundry.png", ImageFormat::Png)
        .unwrap();
}

fn place_pixel(target: &Pixel, working_canvas: &mut Painting) {
    working_canvas
        .image
        .put_pixel(target.position.x, target.position.y, target.color);
    working_canvas.canvas_stats.current_pixels_placed_count += 1;

    for i in 0..3 {
        for j in 0..3 {
            // skip self
            if i == 1 && j == 1 {
                continue;
            }
            // prevent less than zero out-of-bounds
            if target.position.x == 0 && i == 0 {
                continue;
            }
            if target.position.y == 0 && j == 0 {
                continue;
            }
            // prevent greater than dimensions out-of-bounds
            if target.position.x == (working_canvas.canvas_constraints.x_size - 1) && i == 2 {
                continue;
            }
            if target.position.y == (working_canvas.canvas_constraints.y_size - 1) && j == 2 {
                continue;
            }

            let neighbor_x_coord: u32 = target.position.x + i - 1;
            let neighbor_y_coord: u32 = target.position.y + j - 1;

            let neighbor_luma: Luma<u8> = *working_canvas
                .boundry_region_image
                .get_pixel(neighbor_x_coord, neighbor_y_coord);
            let neighbor_color: Rgb<u8> = *working_canvas
                .image
                .get_pixel(neighbor_x_coord, neighbor_y_coord);

            // ensure locations are not added to the boundry region as duplicates
            if neighbor_luma != Luma([0u8]) || neighbor_color != Rgb([0u8, 0u8, 0u8]) {
                continue;
            }

            working_canvas.boundry_region_list.push(Coordinate {
                x: neighbor_x_coord,
                y: neighbor_y_coord,
            });
            working_canvas.boundry_region_image.put_pixel(
                neighbor_x_coord,
                neighbor_y_coord,
                Luma([255u8]),
            );
        }
    }
}

fn get_best_position_for_color(target_color: Rgb<u8>, working_canvas: &mut Painting) -> Pixel {
    let mut min_color_difference: f32 = f32::MAX;
    let mut best_position_index: usize = 0;

    // loop over every available position
    for index in 0..working_canvas.boundry_region_list.len() {
        let mut color_difference_sum: f32 = 0f32;
        let mut neighbor_count: u64 = 0;

        // loop over neighbors
        for i in 0..3 {
            for j in 0..3 {
                // skip self
                if i == 1 && j == 1 {
                    continue;
                }
                // prevent less than zero out-of-bounds
                if working_canvas.boundry_region_list[index].x == 0 && i == 0 {
                    continue;
                }
                if working_canvas.boundry_region_list[index].y == 0 && j == 0 {
                    continue;
                }
                // prevent greater than dimensions out-of-bounds
                if working_canvas.boundry_region_list[index].x
                    == (working_canvas.canvas_constraints.x_size - 1)
                    && i == 2
                {
                    continue;
                }
                if working_canvas.boundry_region_list[index].y
                    == (working_canvas.canvas_constraints.y_size - 1)
                    && j == 2
                {
                    continue;
                }

                let neighbor_x_coord: u32 = working_canvas.boundry_region_list[index].x + i - 1;
                let neighbor_y_coord: u32 = working_canvas.boundry_region_list[index].y + j - 1;

                // skip un-colored
                let neighbor_color: Rgb<u8> = *working_canvas
                    .image
                    .get_pixel(neighbor_x_coord, neighbor_y_coord);
                if neighbor_color == Rgb([0u8, 0u8, 0u8]) {
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

        // Check for MIN
        if avg_color_difference < min_color_difference {
            min_color_difference = avg_color_difference;
            best_position_index = index;
        }
        // in case of a tie, choose randomly
        else if avg_color_difference == min_color_difference && random::<bool>() {
            min_color_difference = avg_color_difference;
            best_position_index = index;
        }
    }

    if working_canvas.boundry_region_list.len() > 1 {
        working_canvas.boundry_region_image.put_pixel(
            working_canvas.boundry_region_list[best_position_index].x,
            working_canvas.boundry_region_list[best_position_index].y,
            Luma([0u8]),
        );
        return Pixel {
            color: target_color,
            position: working_canvas
                .boundry_region_list
                .swap_remove(best_position_index),
        };
    } else {
        working_canvas.boundry_region_image.put_pixel(
            working_canvas.boundry_region_list[best_position_index].x,
            working_canvas.boundry_region_list[best_position_index].y,
            Luma([0u8]),
        );
        return Pixel {
            color: target_color,
            position: working_canvas
                .boundry_region_list
                .remove(best_position_index),
        };
    }
}
