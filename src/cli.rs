use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliArgs {
    /// The path to the initial video file
    pub vid_in: PathBuf,

    /// The path to where you want the resulting spritesheet to go
    pub png_out: PathBuf,

    /// The dimensions of each created tile in the spritesheet
    #[arg(short = 't', long, value_delimiter = ',', num_args = 2, default_values_t = vec![64, 64])]
    pub tile_size: Vec<u32>,

    /// The amount of columns to spread the image into
    #[arg(short = 'c', long)]
    pub columns: Option<u32>,

    /// The fps of the resulting spritesheet (defaults to 10)
    #[arg(short = 'f', long)]
    pub fps: Option<u32>,

    /// Whether to show extra info after completing the conversion
    #[arg(short = 'i', long)]
    pub info: Option<bool>,
}
