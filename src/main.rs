use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
use image_compare::{Algorithm};
use image;

use std::process::Command;
use std::error::Error;
use std::thread;
use std::time::Duration;
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

        let screenshot_image_path = file_path("images/nintendo-switch-ui-screenshot.png");

        let width = screen.display_info.width as f32;
        let height = screen.display_info.height as f32;

        let x: i32 = (width * 0.165) as i32; 
        let y: i32 = (height * 0.696) as i32; 

        let w: u32 = (width * 0.675) as u32;  
        let h: u32 = (height * 0.188) as u32; 

        let image = screen.capture_area(x, y, w, h).unwrap();
        image.save(&screenshot_image_path).unwrap();
        println!("运行耗时: {:?}", start.elapsed());

        let mut envfile = EnvFile::new(&file_path(".env")).unwrap();
        let previous_game = envfile.get("CURRENT_GAME").unwrap_or("").to_string();

        let switch_ui_reference_fullscreen = image::open("ui-reference/nintendo-switch-ui-fullscreen.png").unwrap().into_luma8();
        let switch_ui_reference_windowed = image::open("ui-reference/nintendo-switch-ui-windowed.png").unwrap().into_luma8();
        let switch_ui_screenshot = image::open("images/nintendo-switch-ui-screenshot.png").unwrap().into_luma8();

        let ui_similarity_fullscreen = image_compare::gray_similarity_structure(&Algorithm::MSSIMSimple, &switch_ui_reference_fullscreen, &switch_ui_screenshot).unwrap();
        let ui_similarity_windowed = image_compare::gray_similarity_structure(&Algorithm::MSSIMSimple, &switch_ui_reference_windowed, &switch_ui_screenshot).unwrap();

        let score: f64 = f64::max(ui_similarity_fullscreen.score, ui_similarity_windowed.score);
        let score_string = score.to_string();

        let ocr_input;
        // Set image path and load the image to Ocr
        if score > 0.9 {
            if previous_game != "Home Menu" {
                rm_env(); 
                envfile.update("CURRENT_GAME", "Home Menu");
                envfile.write().expect("Failed to write to dotenv file");
                println!("{}", score_string);
                execute();  
            }
                thread::sleep(Duration::from_millis(500));
                continue;
        } else {
            let captured_image = screen.capture().unwrap();
            captured_image.save(&screenshot_image_path).unwrap();
            ocr_input = image_path(screenshot_image_path, &engine)?;
        }

        // Extract text from image using OcrEngine
        let extracted_text = engine.get_text(&ocr_input)?;
        let detected_game = extracted_text.trim();

        println!("{}", extracted_text);

        // Use match statement to pass game name to dotfile to be loaded into toml file
        match extracted_text.as_str().trim() {
                "Mario Kart World" if previous_game != detected_game => {
                rm_env();
                envfile.update("CURRENT_GAME", "Mario Kart World");
                envfile.write().expect("Failed to write to dotenv file");
                execute();
                }
            
            "Pokemon Pokopia" if previous_game != detected_game => {
                rm_env();
                envfile.update("CURRENT_GAME", "Pokemon Pokopia");
                envfile.write().expect("Failed to write to dotenv file");
                execute();
            }
            _ => println!("Error: No game detected")
        }
        thread::sleep(Duration::from_millis(500));
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

