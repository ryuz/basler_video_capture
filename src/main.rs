const COUNT_IMAGES: u32 = 1000;

fn main() -> anyhow::Result<()> {

    let width = 320;
    let height = 320;

    let pylon = pylon_cxx::Pylon::new();
    let camera = pylon_cxx::TlFactory::instance(&pylon).create_first_device()?;
    println!("Using device {:?}", camera.device_info().model_name()?);
    camera.open()?;
    camera.node_map()?.integer_node("Width")?.set_value(width)?;
    camera.node_map()?.integer_node("Height")?.set_value(height)?;
    camera.node_map()?.enum_node("PixelFormat")?.set_value("Mono8")?;
    camera.node_map()?.float_node("Gain")?.set_value(1.0)?;
    camera.node_map()?.float_node("ExposureTime")?.set_value(700.0)?;   // 露光時間でフレームレートが変わる

    camera.start_grabbing(&pylon_cxx::GrabOptions::default().count(COUNT_IMAGES))?;

    match camera.node_map()?.enum_node("PixelFormat") {
        Ok(node) => println!(
            "pixel format: {}",
            node.value().unwrap_or("could not read value".to_string())
        ),
        Err(e) => eprintln!("Ignoring error getting PixelFormat node: {}", e),
    };

    let mut grab_result = pylon_cxx::GrabResult::new()?;
    let mut img_count: u32 = 0;

    let mut img_buf = vec![vec![0u8; (width * height) as usize]; COUNT_IMAGES as usize];
    

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
            for i in 0..width * height {
                img_buf[img_count as usize][i as usize] = image_buffer[i as usize];
            }
            img_count = img_count.wrapping_add(1);
        } else {
            println!(
                "Error: {} {}",
                grab_result.error_code()?,
                grab_result.error_description()?
            );
        }
    }

    // Save images to timestamped directory
    let now = chrono::Local::now();
    let dir_name = now.format("rec/%Y%m%d_%H%M%S").to_string();
    std::fs::create_dir_all(&dir_name)?;

    for img_count in 0..COUNT_IMAGES {
        let filename = format!("{}/image_{:03}.pgm", dir_name, img_count);
        if let Err(e) = save_pgm_p2(&filename, width as usize, height as usize, 255u32, img_buf[img_count as usize].as_slice()) {
            eprintln!("Failed to save PGM {}: {}", filename, e);
        } else {
            println!("Saved {}", filename);
        }
    }

    Ok(())
}

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
