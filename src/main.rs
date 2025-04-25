use {
    clap::Parser,
    cli::CliArgs,
    cli::vidtoss,
    image::{AnimationDecoder, RgbaImage, codecs::gif::GifDecoder, imageops},
    std::{error::Error, fs::File, io::BufReader, path::Path, process::Stdio},
};

mod cli;

fn last_frame_pos(c: u32, r: u32, t: u32) -> (u32, u32) {
    let lfi = (t - 1) % (c * r);
    let x = lfi % c;
    let y = lfi / c;

    if CliArgs::parse().zero {
        (x, y)
    } else {
        (x + 1, y + 1)
    }
}

fn convert_vid_to_gif_file(
    input_vid_path: &Path,
    output_gif_path: &Path,
    fps: u32,
    scale: Vec<i64>,
) -> anyhow::Result<(), Box<dyn Error>> {
    let status = std::process::Command::new("ffmpeg")
        .arg("-i")
        .arg(input_vid_path)
        .arg("-vf")
        .arg(format!(
            "fps={},scale={}:{}:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse",
            fps, scale[0], scale[1]
        ))
        .arg(output_gif_path)
        .stdout(Stdio::null())
        .output()?;

    if !status.status.success() {
        return Err("Failed to convert video to GIF".into());
    }

    Ok(())
}

type SpriteSheetInfo = (u32, u32, u32, u32, u32);

fn create_sprite_sheet(
    gif_path: &Path,
    tile_size: (u32, u32),
    columns: u32,
    output_png: &Path,
) -> anyhow::Result<SpriteSheetInfo, Box<dyn Error>> {
    let argv = CliArgs::parse();
    let file = File::open(gif_path)?;
    let reader = BufReader::new(file);
    let decoder = GifDecoder::new(reader)?;
    let frames = decoder.into_frames().collect_frames()?;
    let num_frames = frames.len() as u32;
    let mut frame_count: u32 = 0;
    let rows = num_frames.div_ceil(columns);
    let (tile_width, tile_height) = tile_size;

    let mut ss = RgbaImage::new(tile_width * columns, tile_height * rows);

    for (i, frame) in frames.into_iter().enumerate() {
        let frame = frame.into_buffer();
        let i = i as u32;
        let x = (i % columns) * tile_width;
        let y = (i / columns) * tile_height;

        let width_ratio = tile_width as f32 / frame.width() as f32;
        let height_ratio = tile_height as f32 / frame.height() as f32;
        let scale_fac = width_ratio.min(height_ratio);
        let scaled_width = (frame.width() as f32 * scale_fac) as u32;
        let scaled_height = (frame.height() as f32 * scale_fac) as u32;

        let scaled = imageops::resize(&frame, scaled_width, scaled_height, imageops::Lanczos3);

        let frame_center_x = (tile_width - scaled_width) / 2;
        let frame_center_y = (tile_height - scaled_height) / 2;

        imageops::overlay(
            &mut ss,
            &scaled,
            x as i64 + frame_center_x as i64,
            y as i64 + frame_center_y as i64,
        );

        frame_count = frame_count.wrapping_add(1);

        if argv.max_frames.is_some() && frame_count >= argv.max_frames.unwrap() {
            break;
        }
    }

    ss.save(output_png)?;

    let (lfx, lfy) = last_frame_pos(columns, rows, frame_count);

    Ok((rows, columns, lfx, lfy, frame_count))
}

fn main() -> anyhow::Result<(), Box<dyn Error>> {
    vidtoss()
}
