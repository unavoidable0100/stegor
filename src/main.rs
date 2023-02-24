// Define my module
mod cli;
mod utils;

// My Modules
use cli::*;
use utils::*;

// External crates
use clap::Parser;
use image::io::Reader as ImageReader;
use image::ImageError;
use std::path::Path;

fn main() -> Result<(), ImageError> {
    let args = StegoArgs::parse();

    match args.mode {
        Mode::Embed => {
            println!("Embedding...");

            let image = ImageReader::open(&args.image)?.decode()?;

            // Embed the message in the image
            let modified_image =
                embed_message_in_red_ch(image, args.message.expect("Message argument is required"));

            // Save the modified image to a file
            if let Some(output_path) = args.output {
                let path = Path::new(&output_path);
                // Save the output image to the specified path
                modified_image.save(path)?;
            } else {
                modified_image.save("output.png")?;
            }
            
        }
        Mode::Extract => {
            println!("Extracting...");

            let image = image::open(args.image)?;

            if let Some(message) = extract_message_from_red_ch(image) {
                println!("Extracted message: {}", message);
            } else {
                println!("No message found in image");
            }
        }
    }

    Ok(())
}
