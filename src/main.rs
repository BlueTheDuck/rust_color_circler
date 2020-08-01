extern crate image;
extern crate structopt;
#[macro_use]
extern crate log;

use image::GenericImageView;
use std::path::PathBuf;
use structopt::StructOpt;

// Index of each color
const RED: usize = 0;
const GREEN: usize = 1;
const BLUE: usize = 2;

struct Point {
    x: f64,
    y: f64,
}
impl Point {
    /// Calculate the distance between this an another point
    fn distance(&self, p: &Point) -> f64 {
        let a = (self.x - p.x).powi(2);
        let b = (self.y - p.y).powi(2);
        let c = (a + b).sqrt();

        c
    }
}
// Convert any tuple into a Point
impl<N: Into<f64>> std::convert::From<(N, N)> for Point {
    fn from(p: (N, N)) -> Self {
        Self {
            x: p.0.into(),
            y: p.1.into(),
        }
    }
}

/// The kind of shape to draw
#[derive(Debug, PartialEq)]
enum Mode {
    Quad,
    Circle,
}
// Required for structopt
impl std::str::FromStr for Mode {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Quad" | "quad" => Ok(Self::Quad),
            "Circle" | "circle" => Ok(Self::Circle),
            _ => Err("Couldn't convert to Mode"),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "Color circler")]
struct Args {
    #[structopt(short, long, parse(from_os_str), help = "File to read")]
    input: std::path::PathBuf,

    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "File to write. If empty, uses 'output-[input].png'"
    )]
    output: Option<std::path::PathBuf>,

    #[structopt(long, default_value = "Circle", help = "Kind of shape to use")]
    mode: Mode,

    /// Size of the shape
    #[structopt(long, default_value = "5", help = "Size of the shape (diameter)")]
    size: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize program
    env_logger::init();
    let cfg: Args = Args::from_args();

    let img = image::open(&cfg.input).expect("Couldn't open file");
    let (width, height) = (img.width(), img.height());

    info!(
        "{:?} is {}x{}. Processing with {} as size. Using {:?}",
        &cfg.input.to_str(),
        width,
        height,
        cfg.size,
        cfg.mode
    );

    // Store each sector's color
    let mut map: Vec<[u32; 3]> = vec![];

    // "Divide" the picture into sectors and iterate through each of them
    for y in (0..height).step_by(cfg.size as usize) {
        // Break if we get outside of the image
        if (y + cfg.size) > height {
            break;
        }
        for x in (0..width).step_by(cfg.size as usize) {
            // Break if we get outside of the image
            if (x + cfg.size) > width {
                break;
            }

            // Average color of this sector
            let mut avg: [u32; 3] = [0, 0, 0];

            // Generate the color of this sector
            for s_y in 0..cfg.size {
                for s_x in 0..cfg.size {
                    let pixel = img.get_pixel(x + s_x, y + s_y);
                    avg[RED] += pixel[RED] as u32;
                    avg[GREEN] += pixel[GREEN] as u32;
                    avg[BLUE] += pixel[BLUE] as u32;
                }
            }
            avg[RED] /= cfg.size.pow(2);
            avg[GREEN] /= cfg.size.pow(2);
            avg[BLUE] /= cfg.size.pow(2);
            map.push(avg);
        }
        debug!("{} rows of {} rows", height, y);
    }
    info!("Finished processing image. {} sectors", map.len());

    let img = image::RgbaImage::from_fn(width, height, |x, y| {
        let sector_index = (x / cfg.size) + (width / cfg.size) * (y / cfg.size);
        let sector_index = sector_index as usize;
        let sector_center: Point = (
            (x - x % cfg.size + cfg.size / 2),
            (y - y % cfg.size + cfg.size / 2),
        )
            .into();

        // In circle mode we have to
        // check the distance to the center of this circle
        // if we are too far away then we just draw transparent
        if cfg.mode == Mode::Circle && sector_center.distance(&(x, y).into()) > cfg.size as f64 / 2.
        {
            return image::Rgba([0xFF, 0xFF, 0xFF, 0x00]);
        }
        image::Rgba([
            map[sector_index][RED] as u8,
            map[sector_index][GREEN] as u8,
            map[sector_index][BLUE] as u8,
            0xFF,
        ])
    });

    // If we didn't receive an --output, then we have to create the output file using the input as reference
    let output: PathBuf = match cfg.output {
        Some(v) => v,
        None => {
            let mut output = cfg.input.clone();
            let filename = cfg
                .input
                .file_name()
                .expect("Input has no filename")
                .to_str()
                .expect("Filename is not valid UTF-8");
            let filename = format!("output-{}", filename);
            output.set_file_name(filename);
            output.set_extension("png");
            output
        }
    };
    info!("Saving image to {:#?}", &output);
    img.save(output).expect("Couldn't save file");

    Ok(())
}
