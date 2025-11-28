use show_image::{event, ImageView, ImageInfo, create_window};
use clap::Parser;

/// Command-line options
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Image width
    #[arg(short='W', long, default_value_t = 320)]
    width: usize,

    /// Image height
    #[arg(short='H',long, default_value_t = 320)]
    height: usize,

    /// Number of frames to record
    #[arg(short='r',long, default_value_t = 1000)]
    rec_frames: usize,

    /// Exposure time
    #[arg(short='e', long, default_value_t = 800.0)]
    exposure: f64,
}

#[show_image::main]
fn main() -> anyhow::Result<()> {

    let args = Args::parse();
    let width : usize = args.width;
    let height : usize = args.height;
    let rec_frames : usize = args.rec_frames;
    let exposure : f64 = args.exposure;

    let pylon = pylon_cxx::Pylon::new();
    let camera = pylon_cxx::TlFactory::instance(&pylon).create_first_device()?;
    println!("Using device {:?}", camera.device_info().model_name()?);
    camera.open()?;
    camera.node_map()?.integer_node("Width")?.set_value(width as i64)?;
    camera.node_map()?.integer_node("Height")?.set_value(height as i64)?;
    camera.node_map()?.enum_node("PixelFormat")?.set_value("Mono8")?;
//  camera.node_map()?.enum_node("PixelFormat")?.set_value("Mono10")?;
    camera.node_map()?.float_node("Gain")?.set_value(1.0)?;
    camera.node_map()?.float_node("ExposureTime")?.set_value(exposure)?;   // 露光時間でフレームレートが変わる

    match camera.node_map()?.enum_node("PixelFormat") {
        Ok(node) => println!(
            "pixel format: {}",
            node.value().unwrap_or("could not read value".to_string())
        ),
        Err(e) => eprintln!("Ignoring error getting PixelFormat node: {}", e),
    };

    let window = create_window("image", Default::default())?;


    let dumy_img: Vec<u8> = vec![0u8; width * height];
    let image = ImageView::new(ImageInfo::mono8(width as u32, height as u32), &dumy_img);
    window.set_image("image", image)?;

    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
//          println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                break;
            }

            if event.input.key_code == Some(event::VirtualKeyCode::R) && event.input.state.is_pressed() {
                // 録画
                println!("Recording Start");
                let img_buf = grab_image(&camera, rec_frames, (width * height) as usize)?;
                println!("Recording End");

                // Save images to timestamped directory
                let now = chrono::Local::now();
                let dir_name = now.format("rec/%Y%m%d_%H%M%S").to_string();
                println!("Write files {}", dir_name);
                std::fs::create_dir_all(&dir_name)?;
                for img_count in 0..rec_frames {
                    let filename = format!("{}/image_{:04}.pgm", dir_name, img_count);
                    save_pgm_p2(&filename, width as usize, height as usize, 255u32, img_buf[img_count as usize].as_slice())?;
                }
                println!("Write files done.");
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        let img_buf = grab_image(&camera, 1, (width * height) as usize)?;
        let image_view = ImageView::new(
            ImageInfo::mono8(width as u32, height as u32),
            &img_buf[0],
        );
        window.set_image("image", image_view)?;
    }

    Ok(())
}


/// Grab images from the camera.
fn grab_image(camera : &pylon_cxx::InstantCamera, frames: usize, frame_size: usize) -> anyhow::Result<Vec<Vec<u8>>> {
    let mut grab_result = pylon_cxx::GrabResult::new()?;


    let mut img_buf = vec![vec![0u8; frame_size]; frames];
    let mut frame_index: usize = 0;


    camera.start_grabbing(&pylon_cxx::GrabOptions::default().count(img_buf.len() as u32))?;
    while camera.is_grabbing() {
        camera.retrieve_result(
            5000,
            &mut grab_result,
            pylon_cxx::TimeoutHandling::ThrowException,
        )?;

        // Image grabbed successfully?
        if grab_result.grab_succeeded()? {
            // Access the image data.
            let image_buffer = grab_result.buffer()?;
            for i in 0..img_buf[frame_index].len() {
                img_buf[frame_index][i as usize] = image_buffer[i as usize];
            }
            frame_index = frame_index.wrapping_add(1);
        } else {
            println!(
                "Error: {} {}",
                grab_result.error_code()?,
                grab_result.error_description()?
            );
        }
    }

    Ok(img_buf)
}


// Save a PGM P2 (ASCII) file.
fn save_pgm_p2(path: &str, width: usize, height: usize, maxval: u32, data: &[u8]) -> std::io::Result<()> {
    use std::fs::{create_dir_all, File};
    use std::io::{BufWriter, Write};
    use std::path::Path;

    // Ensure parent directory exists (if any)
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            create_dir_all(parent)?;
        }
    }

    let file = File::create(path)?;
    let mut w = BufWriter::with_capacity(64 * 1024, file);

    // Header (P2 = ASCII PGM)
    writeln!(w, "P2")?;
    writeln!(w, "{} {}", width, height)?;
    writeln!(w, "{}", maxval)?;

    // Format each row into a single String to minimize number of write calls.
    // Pre-allocate a buffer large enough for a row: estimate up to 4 chars per pixel ("255 "),
    // but cap at a reasonable size.
    let mut row_buf = String::with_capacity(std::cmp::min(width * 4, 32_768));

    for y in 0..height {
        row_buf.clear();
        let row_start = y * width;
        for x in 0..width {
            if x > 0 {
                row_buf.push(' ');
            }
            let v = data[row_start + x] as u32;
            // write! into String is relatively efficient and avoids per-pixel syscall
            use std::fmt::Write as FmtWrite;
            let _ = write!(row_buf, "{}", v);
        }
        row_buf.push('\n');
        w.write_all(row_buf.as_bytes())?;
    }

    // Flush buffered writer
    w.flush()?;

    Ok(())
}
