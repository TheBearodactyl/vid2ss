use crate::{convert_vid_to_gif_file, create_sprite_sheet};
use clap::Parser;
use rand::Rng;
use std::error::Error;
use std::path::{Path, PathBuf};
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Parser, Clone)]
pub struct CliArgs {
    /// The path to the initial video file
    pub vid_in: PathBuf,

    /// The path to where you want the resulting sprite-sheet to go
    pub png_out: PathBuf,

    /// The dimensions of each created tile in the sprite-sheet
    #[arg(short = 't', long, value_delimiter = ',', num_args = 2, default_values_t = vec![71, 95])]
    pub tile_size: Vec<u32>,

    /// The amount of columns to spread the image into
    #[arg(short = 'c', long)]
    pub columns: Option<u32>,

    /// The fps of the resulting sprite-sheet (defaults to 10)
    #[arg(short = 'f', long)]
    pub fps: Option<u32>,

    /// The width of each frame of the outputed sprite-sheet in pixels
    #[arg(short = 's', long, value_delimiter = ',', num_args = 2)]
    pub scale: Option<Vec<i64>>,

    /// Whether to use more descriptive logging
    #[arg(short = 'v', long)]
    pub verbose: bool,

    /// Whether to keep the temporary
    /// GIF file used in the conversion process
    ///
    /// (defaults to false)
    #[arg(short = 'k', long)]
    pub keep_temp: bool,

    /// Start indexing from zero when calculating the
    /// position of the last frame in the sprite-sheet
    #[arg(short = '0', long)]
    pub zero: bool,

    /// The maximum amount of frames to be processed
    #[arg(short = 'm', long)]
    pub max_frames: Option<u32>,
}

pub fn vidtoss() -> Result<(), Box<dyn Error>> {
    let argv = CliArgs::parse();
    let log_level = if argv.verbose {
        Level::TRACE
    } else {
        Level::ERROR
    };

    let subs = FmtSubscriber::builder().with_max_level(log_level).finish();
    tracing::subscriber::set_global_default(subs).expect("setting default subscriber failed");

    let tile_size = if argv.tile_size.len() == 2 {
        (argv.tile_size[0], argv.tile_size[1])
    } else {
        (71, 95)
    };

    let rand_id = rand::rng().random::<u64>();

    let tmp_name = &format!("temp-gif_out-{}.gif", rand_id);

    convert_vid_to_gif_file(
        &argv.vid_in,
        Path::new(tmp_name),
        argv.fps.unwrap_or(10),
        argv.scale.unwrap_or(vec![320, -1]),
    )?;

    let (rows, columns, last_frame_x, last_frame_y, frame_count) = create_sprite_sheet(
        Path::new(tmp_name),
        tile_size,
        argv.columns.unwrap_or(argv.fps.unwrap_or(10) / 3),
        &argv.png_out,
    )
    .unwrap_or_else(|_| {
        panic!(
            "Failed to generate sprite-sheet from {}",
            argv.vid_in.as_path().file_name().unwrap().to_str().unwrap()
        )
    });

    if argv.verbose {
        info!(
            "Dimensions:\n - Rows ====> {}\n - Columns => {}\n - Frames ==> {}",
            rows, columns, frame_count
        );

        info!(
            "Last frame is located at:\n - X =======> {}\n - Y =======> {}",
            last_frame_x, last_frame_y
        );
    } else {
        println!("Completed sprite-sheet generation!");
    }

    if !argv.keep_temp {
        match std::fs::remove_file(tmp_name) {
            Ok(_) => info!("Successfully removed temporary file"),
            Err(e) => error!("Failed to remove temporary file: {}", e),
        }
    }

    Ok(())
}
