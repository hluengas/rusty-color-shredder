#[derive(Debug)]
struct ColorF32 {
    channel_1: f32,
    channel_2: f32,
    channel_3: f32,
}

fn main() {
    // generate random color list, track generation time
    let start = std::time::Instant::now();
    let _color_list: Vec<ColorF32> = generate_colors(10, false);
    let duration = start.elapsed();

    println!("{:#?}", duration);
}

fn generate_colors(color_bit_depth: u32, shuffle_colors: bool) -> Vec<ColorF32> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    // don't try to do too big of a color space
    if color_bit_depth > 8 {
        println!("Limiting color bit-depth to 8.")
    }

    // number of values per channel based on given color bit depth
    let values_per_channel: u32 = 2u32.pow(std::cmp::min(color_bit_depth, 8));
    let mut color_list: Vec<ColorF32> = Vec::new();

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
                // add this color to the color list
                color_list.push(ColorF32 {
                    channel_1: (channel_1_list[c1_index as usize] as f32)
                        / ((values_per_channel - 1) as f32),
                    channel_2: (channel_2_list[c2_index as usize] as f32)
                        / ((values_per_channel - 1) as f32),
                    channel_3: (channel_3_list[c3_index as usize] as f32)
                        / ((values_per_channel - 1) as f32),
                });
            }
        }
    }

    // final shuffle, removes channel sub-grouping
    if shuffle_colors {
        color_list.shuffle(&mut thread_rng());
    }
    return color_list;
}
