use image::{GrayImage, ImageFormat, Luma, Rgb, RgbImage};
use rand::random;
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
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
    // set output constraints, initialize canvas, get starting positions
    let mut working_canvas: Painting = initialize_canvas();

    // initial update of the output files
    write_output_files(&working_canvas);

    // create a timer to update at regular intervals
    let mut current_time = Instant::now();

    // run the simulation loop as long as there are available positions in the boundry region
    while working_canvas.boundry_region_list.len() > 0 {
        // choose a random color
        let target_color = Rgb([random::<u8>(), random::<u8>(), random::<u8>()]);

        // determine best location
        let target_pixel = get_best_position_for_color(target_color, &mut working_canvas);

        // update the canvas
        place_pixel(&target_pixel, &mut working_canvas);

        // update output files after given interval
        if current_time.elapsed().as_secs_f32() > (1f32 / 2f32) {
            current_time = Instant::now();
            write_output_files(&working_canvas);
        }
    }

    // final update of the output files
    write_output_files(&working_canvas);
}

fn initialize_canvas() -> Painting {
    // hold the output image dimensions
    let working_constraints: Constraints = Constraints {
        x_size: 256u32,
        y_size: 256u32,
    };

    // hold running stats
    let working_stats: Stats = Stats {
        current_pixels_placed_count: 0u64,
        // previous_pixels_placed_count: 0u64,
        // total_pixel_count: (working_constraints.x_size as u64 * working_constraints.y_size as u64),
    };

    // hold all info required for painting
    let mut working_canvas: Painting = Painting {
        image: RgbImage::new(working_constraints.x_size, working_constraints.y_size),
        boundry_region_image: GrayImage::new(
            working_constraints.x_size,
            working_constraints.y_size,
        ),
        starting_locations: get_initial_locations(&working_constraints),
        canvas_constraints: working_constraints,
        canvas_stats: working_stats,
        boundry_region_list: Vec::new(),
    };

    // loop over starting positions and place random colors at each
    for index in 0..working_canvas.starting_locations.len() {
        let target_pixel = Pixel {
            position: working_canvas.starting_locations[index].clone(),
            color: Rgb([random::<u8>(), random::<u8>(), random::<u8>()]),
        };
        place_pixel(&target_pixel, &mut working_canvas)
    }

    return working_canvas;
}

fn get_initial_locations(working_constraints: &Constraints) -> Vec<Coordinate> {
    // hold starting locations
    let mut starting_points = Vec::new();

    // CUSTOMIZED:
    // for i in 0..working_constraints.x_size {
    //     if i % 64 == 0 {
    //         starting_points.push(Coordinate {
    //             x: working_constraints.x_size / 2,
    //             y: i,
    //         });
    //     }
    // }
    starting_points.push(Coordinate {
        x: working_constraints.x_size / 2,
        y: working_constraints.y_size / 2,
    });
    return starting_points;
}

// write the output PNG files
fn write_output_files(working_canvas: &Painting) {
    // write the RGB painting file
    working_canvas
        .image
        .save_with_format("./output/painting.png", ImageFormat::Png)
        .unwrap();

    // write the boundry region image
    working_canvas
        .boundry_region_image
        .save_with_format("./output/boundry.png", ImageFormat::Png)
        .unwrap();
}

// update a pixel on the canvas and add its neighbors to the boundry region
fn place_pixel(target: &Pixel, working_canvas: &mut Painting) {
    // update a pixel on the canvas
    working_canvas
        .image
        .put_pixel(target.position.x, target.position.y, target.color);

    // update counter
    working_canvas.canvas_stats.current_pixels_placed_count += 1;

    // loop over neighbors in a 3x3 grid around the target
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

            // calculate the neighbor's coordinate
            let neighbor_x_coord: u32 = target.position.x + i - 1;
            let neighbor_y_coord: u32 = target.position.y + j - 1;

            // get the neigbor's luma (boundry region value)
            let neighbor_luma: Luma<u8> = *working_canvas
                .boundry_region_image
                .get_pixel(neighbor_x_coord, neighbor_y_coord);

            // get the neighbor's color
            let neighbor_color: Rgb<u8> = *working_canvas
                .image
                .get_pixel(neighbor_x_coord, neighbor_y_coord);

            // ensure locations are not added to the boundry region as duplicates
            if neighbor_luma != Luma([0u8]) || neighbor_color != Rgb([0u8, 0u8, 0u8]) {
                continue;
            }

            // add this neighbor to the boundry region LIST
            working_canvas.boundry_region_list.push(Coordinate {
                x: neighbor_x_coord,
                y: neighbor_y_coord,
            });
            // add this neighbor to the boundry region IMAGE (luma)
            working_canvas.boundry_region_image.put_pixel(
                neighbor_x_coord,
                neighbor_y_coord,
                Luma([255u8]),
            );
        }
    }
}

fn get_best_position_for_color(target_color: Rgb<u8>, working_canvas: &mut Painting) -> Pixel {
    let (_best_value, best_position, best_position_index) = working_canvas
        .boundry_region_list
        .par_iter()
        .enumerate()
        .map(|available_location| {
            evaluate_position(
                available_location.1,
                available_location.0,
                &target_color,
                &working_canvas.image,
                &working_canvas.canvas_constraints,
            )
        })
        .reduce_with(|a, b| {
            if a.0 < b.0 {
                return a;
            } else if a.0 == b.0 && random::<bool>() {
                return a;
            } else {
                return b;
            }
        })
        .unwrap();

    // remove target pixel from boundrry region IMAGE
    working_canvas
        .boundry_region_image
        .put_pixel(best_position.x, best_position.y, Luma([0u8]));

    // if this is not the last location of the boundry region
    // swap remove the target pixel location from the boundry region LIST
    // (swap remove is much faster)
    if working_canvas.boundry_region_list.len() > 1 {
        return Pixel {
            color: target_color,
            position: working_canvas
                .boundry_region_list
                .swap_remove(best_position_index),
        };
    }
    // for the last elemet remove normally
    else {
        return Pixel {
            color: target_color,
            position: working_canvas
                .boundry_region_list
                .remove(best_position_index),
        };
    }
}

fn evaluate_position(
    target_location: &Coordinate,
    target_index: usize,
    target_color: &Rgb<u8>,
    canvas_image: &RgbImage,
    canvas_constraints: &Constraints,
) -> (f32, Coordinate, usize) {
    // accumulation variables
    let mut color_difference_sum: f32 = 0f32;
    let mut neighbor_count: u64 = 0;

    // loop over neighbors in a 3x3 grid around the target
    for i in 0..3 {
        for j in 0..3 {
            // skip self
            if i == 1 && j == 1 {
                continue;
            }
            // prevent less than zero out-of-bounds
            if target_location.x == 0 && i == 0 {
                continue;
            }
            if target_location.y == 0 && j == 0 {
                continue;
            }
            // prevent greater than dimensions out-of-bounds
            if target_location.x == (canvas_constraints.x_size - 1) && i == 2 {
                continue;
            }
            if target_location.y == (canvas_constraints.y_size - 1) && j == 2 {
                continue;
            }

            // calculate neighbor coordinates
            let neighbor_x_coord: u32 = target_location.x + i - 1;
            let neighbor_y_coord: u32 = target_location.y + j - 1;

            // get color at neighbor's coordinates
            let neighbor_color: Rgb<u8> =
                *canvas_image.get_pixel(neighbor_x_coord, neighbor_y_coord);

            // skip un-colored
            if neighbor_color == Rgb([0u8, 0u8, 0u8]) {
                continue;
            }

            //compute color diference
            for i in 0..3 {
                color_difference_sum +=
                    (target_color[i] as f32 - neighbor_color[i] as f32).powf(2f32) as f32;
            }
            neighbor_count += 1;
        }
    }

    let target_value: f32 = color_difference_sum / neighbor_count as f32;
    return (target_value, *target_location, target_index);
}
