use clap::Parser;
use cli::CliArgs;
use image::{AnimationDecoder, DynamicImage, Frame, RgbaImage, codecs::gif::GifDecoder};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

mod cli;

fn convert_vid_to_gif_file(
    input_vid_path: &PathBuf,
    output_gif_path: &PathBuf,
    fps: u32,
) -> anyhow::Result<()> {
    let _ = std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(input_vid_path)
        .arg("-vf")
        .arg(format!(
            "fps={},scale=320:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
            fps
        ))
        .arg(output_gif_path)
        .output()?;

    Ok(())
}

fn create_spritesheet(
    gif_path: &Path,
    tile_size: (u32, u32),
    columns: u32,
    output_png: &Path,
) -> anyhow::Result<(u32, u32, u32, u32), Box<dyn std::error::Error>> {
    let file = File::open(gif_path)?;
    let reader = BufReader::new(file);
    let decoder = GifDecoder::new(reader)?;
    let frames: Vec<Frame> = decoder.into_frames().collect_frames()?;
    let num_frames = frames.len() as u32;
    let rows = num_frames.div_ceil(columns);
    let (width, height) = tile_size;
    let mut sprite_sheet = RgbaImage::new(width * columns, height * rows);

    for (i, frame) in frames.into_iter().enumerate() {
        let i = i as u32;
        let x = (i % columns) * width;
        let y = (i / columns) * height;

        let frame_img = frame.into_buffer();
        let frame_img = DynamicImage::ImageRgba8(frame_img);
        let thumbnail = frame_img.thumbnail_exact(width, height);
        let x_offset = (width - thumbnail.width()) / 2;
        let y_offset = (height - thumbnail.height()) / 2;

        image::imageops::overlay(
            &mut sprite_sheet,
            &thumbnail,
            (x + x_offset) as i64,
            (y + y_offset) as i64,
        );
    }

    sprite_sheet.save(output_png)?;

    Ok((rows, columns, (columns - 1) * width, (rows - 1) * height))
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let tile_size = if args.tile_size.len() == 2 {
        (args.tile_size[0], args.tile_size[1])
    } else {
        (64, 64)
    };

    let temp_gif = NamedTempFile::new()?;
    let temp_gif_path = temp_gif.path();

    convert_vid_to_gif_file(&args.vid_in, &args.png_out, args.fps.unwrap_or(10))?;

    let output_png = args.vid_in.with_extension("spritesheet.png");
    let (rows, columns, last_frame_x, last_frame_y) = create_spritesheet(
        temp_gif_path,
        tile_size,
        args.columns.unwrap_or(30),
        &output_png,
    )
    .expect("Failed to convert gif to spritesheet");

    println!("Completed spritesheet generation!");

    if args.info.unwrap() {
        println!(
            "Dimensions:\n - Rows ====> {}\n - Columns => {}",
            rows, columns
        );

        println!(
            "Last frame is located at:\n- X => {}\n- Y => {}",
            last_frame_x, last_frame_y
        );
    }

    Ok(())
}
