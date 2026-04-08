use std::process::Command;
use envfile::EnvFile;
use std::path::Path;


fn main() {
    let mut envfile = EnvFile::new(&Path::new(".env")).unwrap();
    
    let game = "Mario Kart World";

    match game {
        "Mario Kart World" => {
            envfile.update("CURRENT_GAME", "Mario Kart World");
        }
        _ => println!("Error: No game detected")
    }

    envfile.write().expect("Failed to write to dotenv file");

    let chmod = Command::new("chmod")
        .arg("+x")
        .arg("./src/shell/config.sh")
        .output()
        .expect("failed to make script executable");

    let execute = Command::new("bash")
        .arg("-c")
        .arg("set -a; source /home/$USER/Switch-2-GameDetect-RPC/.env; set +a; ./src/shell/config.sh")
        .output()
        .expect("failed to execute");

    println!("{}", String::from_utf8_lossy(&chmod.stdout));
    println!("{}", String::from_utf8_lossy(&execute.stdout));
}


