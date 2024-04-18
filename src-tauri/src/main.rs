use core::panic;
use once_cell::sync::Lazy;
use std::fs;
use std::process::Command;
mod template;
use template::generate_template::WallpaperManager;


// we get the images from the user
#[tauri::command]
async fn get_img_address(img_address: String) {
    get_img(img_address).await;
}


// a command to get the user name
fn get_user() -> String {
    let command_user = Command::new("whoami")
        .output()
        .expect("error getting the username");

    if command_user.status.success() {
        let user_name = String::from_utf8_lossy(&command_user.stdout)
            .trim()
            .to_string();
        user_name // Return the user_name directly
    } else {
        panic!("Error getting the user");
    }
}

// we create a static variable where we get the path

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

// we get the path of the images, we replace for the real path and
// generate a template
async fn get_img(img_address: String) {
    let trimmed_address = trim_words(&img_address, &["2F"]);
    let trim_fix = &trimmed_address.replace("%", "/");
    let real_path = &trim_fix.replace("asset://localhost/", " ");
    let mut  wallpaper_manager = WallpaperManager;
    let result = match wallpaper_manager.generate_template(real_path).await {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Error generating template: {}", err);
            Err(err)
        }
    };

    if let Err(err) = result {
        panic!("Panic occurred: {:?}", err);
    }
}



// we send the path of every image to the user
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
