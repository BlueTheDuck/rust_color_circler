extern crate image;

use image::GenericImageView;

fn main() {
    let mut sector_size = 5;
    let mut radius = (sector_size as f64) / 2f64;
    let mut verbose = false;
    let mut quad = false;

    let mut input_file_name = String::from("input.png");
    let mut output_file_name = String::from("output.png");
    //#region Argument handling
    let mut args_list: Vec<_> = std::env::args().rev().collect();
    args_list.pop();
    while args_list.len() > 0 {
        let arg: String = args_list.pop().unwrap();

        match arg.as_ref() {
            "--in" => {
                input_file_name = args_list.pop().expect("No more arguments passed");
            }
            "--out" => {
                output_file_name = args_list.pop().expect("No more arguments passed");
            }
            "--size" => {
                sector_size = args_list
                    .pop()
                    .expect("No more arguments passed")
                    .parse::<u32>()
                    .expect("Couldn't parse input. It must be an u8");;
            }
            "--rad" => {
                radius = args_list
                    .pop()
                    .expect("No more arguments passed")
                    .parse::<f64>()
                    .expect("Couldn't parse input. It must be an f64");;
            }
            "--verbose" => {
                verbose = args_list
                    .pop()
                    .expect("No more arguments passed")
                    .parse::<bool>()
                    .expect("Couldn't parse input. It must be a bool");
            }
            "--quad" => {
                quad = match args_list.pop() {
                    Some(e) => e
                        .parse::<bool>()
                        .expect("Couldn't parse input. It must be a bool"),
                    None => true,
                }
            }
            &_ => {
                println!("Meaningless {}", arg);
            }
        }
    }
    //#endregion
    let img = image::open(&input_file_name).expect("Couldn't open file");
    let (width, height) = (img.width(), img.height());

    println!(
        "{} is {}x{}. Processing with {}/{} as size/rad. Using {}",
        &input_file_name, width, height, sector_size, radius,(match quad{true=>"quads",false=>"circles"})
    );

    let mut map: Vec<[u32; 3]> = vec![];
    let mut act_prom: [u32; 3] = [0, 0, 0];

    //#region Processing
    for y in (0..height).step_by(sector_size as usize) {
        if (y + sector_size) > height {
            break;
        }
        for x in (0..width).step_by(sector_size as usize) {
            if (x + sector_size) > width {
                break;
            }

            for s_y in y..(y + sector_size) {
                for s_x in x..(x + sector_size) {
                    for i in 0..3 {
                        act_prom[i] = act_prom[i] + img.get_pixel(s_x, s_y)[i] as u32;
                    }
                }
            }
            for i in 0..3 {
                act_prom[i] = act_prom[i] / (sector_size * sector_size);
            }
            map.push(act_prom);
            act_prom = [0, 0, 0];
        }
        if verbose {
            println!("{} rows of {} rows", height, y);
        }
    }
    println!("Finished processing image. {} sectors", map.len());
    //#endregion

    //#region Calculating pixels
    let distance_calc = |point: [i32; 2], sector_center: [i32; 2]| {
        let dx: u32 = (point[0] - sector_center[0]).abs() as u32;
        let dy: u32 = (point[1] - sector_center[1]).abs() as u32;
        let distance: f64 = ((dx.pow(2) + dy.pow(2)) as f64).sqrt();
        distance
    };

    let img = image::RgbImage::from_fn(width, height, |x, y| {
        let sector_index = (x / sector_size) + (width / sector_size) * (y / sector_size);
        let sector_index = sector_index as usize;
        let sector_center: [i32; 2] = [
            (x - x % sector_size + sector_size / 2) as i32,
            (y - y % sector_size + sector_size / 2) as i32,
        ];

        if (distance_calc(sector_center, [x as i32, y as i32]) > radius)&&!quad {
            return image::Rgb([0xFF, 0xFF, 0xFF]);
        }
        image::Rgb([
            map[sector_index][0] as u8,
            map[sector_index][1] as u8,
            map[sector_index][2] as u8,
        ])
    });
    //#endregion

    println!("Saving image to {}", output_file_name);
    img.save(output_file_name).expect("Couldn't save file");
}
