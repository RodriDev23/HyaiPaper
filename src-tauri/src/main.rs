use once_cell::sync::Lazy;
use std::fs;
use std::process::Command;

mod template;
use template::generate_template::WallpaperManager;

static WALLPAPERS_PATH: Lazy<Vec<String>> = Lazy::new(|| {
    let mut wallpaper_paths = Vec::new();

    let directory_path = format!("/home/{}/Pictures", get_user());
    if let Ok(entries) = fs::read_dir(&directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let directory_path_wallpaper =
                    format!("{}/{}", directory_path, file_name.to_string_lossy());
                wallpaper_paths.push(directory_path_wallpaper);
            }
        }
    } else {
        panic!("Failed to read directory");
    }

    wallpaper_paths
});

fn trim_words(input: &str, words: &[&str]) -> String {
    let mut result = input.to_string();
    for word in words {
        result = result.replace(word, "");
    }

    result
}

async fn get_img(img_address: String) {
    println!("img selected : {}", img_address);
    let trimmed_address = trim_words(&img_address, &["2F"]);
    let trim_fix = &trimmed_address.replace("%", "/");
    let real_path = &trim_fix.replace("asset://localhost/", " ");
    println!("Trimmed img address: {}", real_path);

    let wallpaper_manager = WallpaperManager;
    let result = match wallpaper_manager.generate_template(real_path).await {
        Ok(_) => {
            println!("Template generated successfully");
            Ok(())
        }
        Err(err) => {
            eprintln!("Error generating template: {}", err);
            Err(err)
        }
    };

    if let Err(err) = result {
        eprintln!("Panic occurred: {:?}", err);
    }
}


#[tauri::command]
async fn get_img_address(img_address: String) {
    get_img(img_address).await;

}

fn get_user() -> String {
    let mut user_name = String::new();
    let command_user = Command::new("whoami")
        .output()
        .expect("error getting the username");
    if command_user.status.success() {
        user_name = String::from_utf8_lossy(&command_user.stdout)
            .trim()
            .to_string();
        println!("user name: {}", user_name);
    } else {
        panic!("Error getting the user");
    }
    user_name
}

#[tauri::command]
fn send_wallpapers_user() -> Vec<String> {
    WALLPAPERS_PATH.clone()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            send_wallpapers_user,
            get_img_address
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

