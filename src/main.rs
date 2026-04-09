use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use image;

use std::process::Command;
use std::error::Error;
use envfile::EnvFile;
#[allow(unused)]
use std::path::Path;
use std::path::PathBuf;

use screenshots::Screen;
use std::time::Instant;

/// Given a file path relative to the crate root, return the absolute path.
fn file_path(path: &str) -> PathBuf {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(path);
    abs_path
}
fn main() -> Result<(), Box<dyn Error>> {
    let detection_model_path = file_path("text-detection.rten");
    let rec_model_path = file_path("text-recognition.rten");

    let detection_model = Model::load_file(detection_model_path)?;
    let recognition_model = Model::load_file(rec_model_path)?;
    
    // initialize OcrEngine
    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;

    loop {
        let region1 = true; 
        let ocr_input;

        let start = Instant::now();
        
        let screens = Screen::all().expect("Failed to find screen");
        if screens.is_empty() {
            panic!("Failed to find screens")
        }

        let screen = screens
        .iter()
        .find(|s| s.display_info.is_primary)
        .unwrap_or_else(|| {
            screens.get(0).expect("No screens detected at all")
        });

        let screenshot_image_path = file_path("images/test.png");

        let width = screen.display_info.width as f32;
        let height = screen.display_info.height as f32;

        let x: i32 = (width * 0.165) as i32; 
        let y: i32 = (height * 0.696) as i32; 

        let w: u32 = (width * 0.675) as u32;  
        let h: u32 = (height * 0.188) as u32; 

        let image = screen.capture_area(x, y, w, h).unwrap();
        image.save(&screenshot_image_path).unwrap();
        println!("运行耗时: {:?}", start.elapsed());

        
        // Set image path and load the image to Ocr
        if region1 == true {
            let img_path = screenshot_image_path;
            ocr_input = image_path(img_path, &engine)?;
        } else {
            // Handle other regions or provide a default
            // For now, let's assume we always have region1
            continue;
        }

        // Extract text from image using OcrEngine
        let extracted_text = engine.get_text(&ocr_input)?;

        println!("{}", extracted_text);

        rm_env();
        let mut envfile = EnvFile::new(&Path::new(".env")).unwrap();

        // Use match statement to pass game name to dotfile to be loaded into toml file
        match extracted_text.as_str() {
            "Mario Kart World" => {
                envfile.update("CURRENT_GAME", "Mario Kart World");
                envfile.write().expect("Failed to write to dotenv file");
                execute();
            }
            "Pokemon Pokopia" => {
                envfile.update("CURRENT_GAME", "Pokemon Pokopia");
                envfile.write().expect("Failed to write to dotenv file");
                execute();
            }
            _ => println!("Error: No game detected")
        }
    }
}

fn execute() {
    // Make shell file executable
    let chmod = Command::new("chmod")
        .arg("+x")
        .arg("./src/shell/config.sh")
        .output()
        .expect("failed to make script executable");

    // Pass dotenv file into shell file and execute
    let execute = Command::new("bash")
        .arg("-c")
        .arg("set -a; source /home/$USER/Switch-2-GameDetect-RPC/.env; set +a; ./src/shell/config.sh")
        .output()
        .expect("failed to execute");

    println!("{}", String::from_utf8_lossy(&chmod.stdout));
    println!("{}", String::from_utf8_lossy(&execute.stdout));
}

fn rm_env() {
    let chmod = Command::new("chmod")
        .arg("+x")
        .arg("./src/shell/rm.sh")
        .output()
        .expect("failed to make script executable");

    // Pass dotenv file into shell file and execute
    let execute = Command::new("bash")
        .arg("-c")
        .arg("./src/shell/rm.sh")
        .output()
        .expect("failed to execute");

    println!("{}", String::from_utf8_lossy(&chmod.stdout));
    println!("{}", String::from_utf8_lossy(&execute.stdout));
}

fn image_path(img_path: PathBuf, engine: &OcrEngine) -> Result<ocrs::OcrInput, Box<dyn Error>> {
    let img = image::open(img_path)?.into_rgb8();
    let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
    let ocr_input = engine.prepare_input(img_source)?;
    Ok(ocr_input)
}

