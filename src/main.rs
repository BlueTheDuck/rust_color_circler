extern crate image;

use image::GenericImageView;
use image::{GenericImage, ImageBuffer};

fn main() {
    let mut sector_size = 5;

    let mut input_file_name = String::from("picture.jpg");
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
                sector_size = args_list.pop().expect("No more arguments passed").parse::<u32>().expect("Couldn't parse input. It must be an u8");;
            }
            &_ => {
                println!("Meaningless {}", arg);
            }
        }
    }
    //#endregion
    let img = image::open(&input_file_name).expect("Couldn't open file");
    let (width, height) = (img.width(), img.height());

    println!("{} is {}x{}", &input_file_name, width, height);

    let mut map: Vec<[u32; 3]> = vec![];
    let mut act_prom: [u32; 3] = [0, 0, 0];

    for y in (0..height).step_by(sector_size as usize) {
        if (y + sector_size) > height {
            break;
        }
        for x in (0..width).step_by(sector_size as usize) {
            if (x + sector_size) > width {
                break;
            }
            println!("Working on ({}:{})", x, y);
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
            println!("Pushed");
            map.push(act_prom);
            act_prom = [0, 0, 0];
        }
    }
    println!("Finished processing image. {} sectors",map.len());
    println!("{:?}", map);
    /* let out_img = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = image::Rgb(map[s_y * width + s_x]);
        image::Luma([0u8]);
    }); */
    let img = image::RgbImage::from_fn(width, height, |x, y| {
        /*if (width - width%sector_size)>x || (height - height%sector_size)>y {
            return image::Rgb([255,0,255]);
        }*/
        let sector_index = (x/sector_size)+(width/sector_size)*(y/sector_size);
        let sector_index = sector_index as usize;
        println!("({};{}) = {} as {:?}",x,y,sector_index,map[sector_index]);
        let mut newrgb:[u8;3] = [0,0,0];
        for i in 0..3 {
            newrgb[i] = map[sector_index][i] as u8;
        }
        image::Rgb(newrgb)
    });

    img.save(output_file_name);

    /*for a_edge in 0..m_edge {
        let mut amount_pixel_processed: u32 = 0;
        let mut act_sum_rgb = [0, 0, 0];
        let mut act_prom_rgb = [0, 0, 0];
        for y in 0..a_edge {
            for x in 0..a_edge {
                let pixel = img.get_pixel(x, y);
                for i in 0..2 {
                    act_sum_rgb[i] = act_sum_rgb[i] + pixel[i] as u32;
                    act_prom_rgb[i] = act_sum_rgb[i] / amount_pixel_processed;
                }
                amount_pixel_processed = amount_pixel_processed + 1;
            }
        }
    }*/
}
