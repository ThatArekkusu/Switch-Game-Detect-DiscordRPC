use image_compare::{Algorithm};
use image;

use std::process::Command;
use std::error::Error;
use std::thread;
use std::time::Duration;
use std::fs;

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

        // Set image path and load the image to Ocr
        if score > 0.9 {
            if previous_game != "Home Menu" {
                rm_env(); 
                envfile.update("CURRENT_GAME", "Home Menu");
                envfile.write().expect("Failed to write to dotenv file");
                println!("{}", score_string);
                execute();
                thread::sleep(Duration::from_millis(50));
                restart_xorg_rpc(); 
            } 
                thread::sleep(Duration::from_millis(50));
                continue;
        }


        let x_mw: i32 = (width * 0.2703) as i32; 
        let y_mw: i32 = (height * 0.0465) as i32; 
        let w_mw: u32 = (width * 0.4438) as u32;  
        let h_mw: u32 = (height * 0.4556) as u32;

        let _ = fs::remove_file("game-images/mario-kart-world-logo-windowed.png");
        let screenshot_image_path_mario_windowed = file_path("game-images/mario-kart-world-logo-windowed.png");
        let image_mario_kart_world_logo_windowed = screen.capture_area(x_mw, y_mw, w_mw, h_mw).unwrap();
        image_mario_kart_world_logo_windowed.save(&screenshot_image_path_mario_windowed).expect("Err, could not save screenshot");
        let screenshot_mario_windowed = image::open("game-images/mario-kart-world-logo-windowed.png").unwrap().into_luma8();
        let mario_kart_world_reference_windowed = image::open("ui-reference/mario-kart-world-logo-windowed-reference.png").unwrap().into_luma8();


        let x_mf: i32 = (width * 0.2598) as i32; 
        let y_mf: i32 = (height * 0.0305) as i32; 
        let w_mf: u32 = (width * 0.4578) as u32;  
        let h_mf: u32 = (height * 0.4375) as u32;

        let _ = fs::remove_file("game-images/mario-kart-world-logo-fullscreen.png");
        let screenshot_image_path_mario_fullscreen = file_path("game-images/mario-kart-world-logo-fullscreen.png");
        let image_mario_kart_world_logo_fullscreen = screen.capture_area(x_mf, y_mf, w_mf, h_mf).unwrap();
        image_mario_kart_world_logo_fullscreen.save(&screenshot_image_path_mario_fullscreen).expect("Err, could not save screenshot");
        let screenshot_mario_fullscreen = image::open("game-images/mario-kart-world-logo-fullscreen.png").unwrap().into_luma8();
        let mario_kart_world_reference_fullscreen = image::open("ui-reference/mario-kart-world-logo-fullscreen-reference.png").unwrap().into_luma8();

        let mario_kart_similarity_windowed = image_compare::gray_similarity_structure(&Algorithm::MSSIMSimple, &mario_kart_world_reference_windowed, &screenshot_mario_windowed).unwrap();
        let mario_kart_similarity_fullscreen = image_compare::gray_similarity_structure(&Algorithm::MSSIMSimple, &mario_kart_world_reference_fullscreen, &screenshot_mario_fullscreen).unwrap();

        let mario_score: f64 = f64::max(mario_kart_similarity_windowed.score, mario_kart_similarity_fullscreen.score);
        let mario_score_string = mario_score.to_string();
        println!("{}", mario_score_string);

        let pokemon_score = 0.5;
        println!("{}", mario_score_string);
                if mario_score > 0.45 {
                    if previous_game != "Mario Kart World" {
                        rm_env();
                        envfile.update("CURRENT_GAME", "Mario Kart World");
                        envfile.write().expect("Failed to write to dotenv file");
                        println!("Mario");
                        execute();
                        thread::sleep(Duration::from_millis(50));
                        restart_xorg_rpc();
                    }
                } else if pokemon_score > 0.9 { 
                    if previous_game != "Pokemon Pokopia" {
                        rm_env();
                        envfile.update("CURRENT_GAME", "Pokemon Pokopia");
                        envfile.write().expect("Failed to write to dotenv file");
                        execute();
                        thread::sleep(Duration::from_millis(500));
                        restart_xorg_rpc();
                    }
                } else {
                    println!("Error: No game detected");
            }
            //thread::sleep(Duration::from_millis(250));
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

fn restart_xorg_rpc() {
    let chmod = Command::new("chmod")
        .arg("+x")
        .arg("./src/shell/restart-xorg-rpc.sh")
        .output()
        .expect("failed to make script executable");

    // Pass dotenv file into shell file and execute
    let execute = Command::new("bash")
        .arg("-c")
        .arg("./src/shell/restart-xorg-rpc.sh")
        .output()
        .expect("failed to execute");

    println!("{}", String::from_utf8_lossy(&chmod.stdout));
    println!("{}", String::from_utf8_lossy(&execute.stdout));
}