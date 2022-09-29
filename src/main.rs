use image::{ImageFormat, Rgb, RgbImage};
use rand::random;

fn main() {
    // intitialize counters
    let x_dim: i32 = 64;
    let y_dim: i32 = 64;

    //  let previous_pixels_placed_count: i32 = 0;
    let total_pixel_count: i32 = (x_dim + 1i32) * (y_dim + 1i32);

    let x_start: i32 = 32;
    let y_start: i32 = 32;

    let mut boundry_region: Vec<(i32, i32)> = Vec::new();

    let mut canvas = RgbImage::new(x_dim as u32, y_dim as u32);

    place_pixel(
        x_start,
        y_start,
        x_dim,
        y_dim,
        Rgb([random::<u8>(), random::<u8>(), random::<u8>()]),
        &mut canvas,
        &mut boundry_region,
    );
    canvas
        .save_with_format("./output/painting.png", ImageFormat::Png)
        .unwrap();

    for current_pixels_placed_count in 0..total_pixel_count {
        let target_color = Rgb([random::<u8>(), random::<u8>(), random::<u8>()]);
        let best_position =
            get_best_position_for_color(&target_color, &canvas, x_dim, y_dim, &mut boundry_region);
        place_pixel(
            best_position.0,
            best_position.1,
            x_dim,
            y_dim,
            target_color,
            &mut canvas,
            &mut boundry_region,
        );
        if current_pixels_placed_count % 25 == 0 {
            canvas
                .save_with_format("./output/painting.png", ImageFormat::Png)
                .unwrap();
        }
    }
    canvas
        .save_with_format("./output/painting.png", ImageFormat::Png)
        .unwrap();
}

fn place_pixel(
    x_coord: i32,
    y_coord: i32,
    x_dim: i32,
    y_dim: i32,
    rgb_pixel: Rgb<u8>,
    canvas: &mut RgbImage,
    boundry_region: &mut Vec<(i32, i32)>,
) {
    canvas.put_pixel(x_coord as u32, y_coord as u32, rgb_pixel);
    for i in 0..3 {
        for j in 0..3 {
            let i_coord: i32 = x_coord + i - 1;
            let j_coord: i32 = y_coord + j - 1;

            let neighbor_color: Rgb<u8> = *canvas.get_pixel(i_coord as u32, j_coord as u32);

            if (i_coord >= 0) && (j_coord >= 0) && (i_coord < x_dim) && (j_coord < y_dim) && (neighbor_color == Rgb([0, 0, 0])){
                boundry_region.push((i_coord, j_coord));
            }
        }
    }
}

fn get_best_position_for_color(
    target_color: &Rgb<u8>,
    canvas: &RgbImage,
    x_dim: i32,
    y_dim: i32,
    boundry_region: &mut Vec<(i32, i32)>,
) -> (i32, i32) {
    let mut min_color_difference: u64 = u64::max_value();
    let mut best_position_index: usize = 0;

    // loop over every available position
    for index in 0..boundry_region.len() {
        let mut color_difference_sum: u64 = 0;
        let mut neighbor_count: u64 = 0;
        // loop over neighbors
        for i in 0..3 {
            for j in 0..3 {
                // calculate neighbor coordinates
                let i_coord: i32 = boundry_region[index].0 + i - 1;
                let j_coord: i32 = boundry_region[index].1 + j - 1;

                // skip self
                if i == 1 && j == 1 {
                    continue;
                }
                // skip out-of-bounds
                if i_coord < 0 || j_coord < 0 {
                    continue;
                }
                // skip out-of-bounds
                if i_coord >= x_dim || j_coord >= y_dim {
                    continue;
                }

                // skip un-colored
                let neighbor_color: Rgb<u8> = *canvas.get_pixel(i_coord as u32, j_coord as u32);
                 if neighbor_color == Rgb([0, 0, 0]) {
                     continue;
                 }

                //compute color diiference
                for i in 0..3 {
                    color_difference_sum +=
                        (target_color[i] as i32 - neighbor_color[i] as i32).pow(2) as u64;
                }
                neighbor_count += 1;
            }
        }

        let avg_color_difference = color_difference_sum / neighbor_count;
        if avg_color_difference < min_color_difference {
            min_color_difference = avg_color_difference;
            best_position_index = index;
        }
    }
    return boundry_region.swap_remove(best_position_index);
}
