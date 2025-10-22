const COUNT_IMAGES: u32 = 100;

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
    camera.node_map()?.float_node("ExposureTime")?.set_value(1000.0)?;


    // camera.enum_node("PixelFormat")?.set_value("RGB8")?;

    // Start the grabbing of COUNT_IMAGES_TO_GRAB images.
    // The camera device is parameterized with a default configuration which
    // sets up free-running continuous acquisition.
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
    

    // Camera.StopGrabbing() is called automatically by the RetrieveResult() method
    // when c_countOfImagesToGrab images have been retrieved.
    while camera.is_grabbing() {
        // Wait for an image and then retrieve it. A timeout of 5000 ms is used.
        camera.retrieve_result(
            5000,
            &mut grab_result,
            pylon_cxx::TimeoutHandling::ThrowException,
        )?;

        // Image grabbed successfully?
        if grab_result.grab_succeeded()? {
            // Access the image data.
//            println!("SizeX: {}", grab_result.width()?);
//           println!("SizeY: {}", grab_result.height()?);

            let image_buffer = grab_result.buffer()?;
//          println!("Value of first pixel: {}\n", image_buffer[0]);
 
//          let w = grab_result.width()?;
//          let h = grab_result.height()?;
            /*
            for y in 0..h {
                for x in 0..w {
                    let offset = (y * w + x) as usize;
                    print!("{:3} ", image_buffer[offset]);
                }
                println!();
            }*/

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

    for img_count in 0..COUNT_IMAGES {
        let filename = format!("rec/image_{:03}.pgm", img_count);
        if let Err(e) = save_pgm_p2(&filename, width as usize, height as usize, 255u32, img_buf[img_count as usize].as_slice()) {
            eprintln!("Failed to save PGM {}: {}", filename, e);
        } else {
            println!("Saved {}", filename);
        }
//      img_count = img_count.wrapping_add(1);
    }

    Ok(())
}

fn save_pgm_p2(path: &str, width: usize, height: usize, maxval: u32, data: &[u8]) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(path)?;
    // Header
    // P2 indicates ASCII PGM
    writeln!(file, "P2")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "{}", maxval)?;

    // Write pixels in row-major order as ASCII numbers. We'll write up to 17 values per line
    // to keep lines reasonably sized.
    let mut count_in_line = 0;
    for (i, &v) in data.iter().take(width * height).enumerate() {
        let val = v as u32;
        write!(file, "{}", val)?;
        count_in_line += 1;
        if i + 1 < width * height {
            if count_in_line >= 17 {
                writeln!(file)?;
                count_in_line = 0;
            } else {
                write!(file, " ")?;
            }
        }
    }
    writeln!(file)?;

    Ok(())
}
