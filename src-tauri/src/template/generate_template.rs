use std::fs::OpenOptions;
use std::io::{self, BufRead, Seek, Write};
use std::process::Command;
use std::{fs, str};

pub struct WallpaperManager;

impl WallpaperManager {
    async fn detect_monitor(&self) -> Option<String> {
        let output_monitor_name = Command::new("hyprctl")
            .arg("monitors")
            .output()
            .expect("Failed to execute command");

        if output_monitor_name.status.success() {
            let stdout_str = str::from_utf8(&output_monitor_name.stdout).expect("Invalid UTF-8");
            let mut monitor_name = None;
            for line in stdout_str.lines() {
                if let Some(start) = line.find('r') {
                    if let Some(end) = line.find('(') {
                        monitor_name = Some(line[start + 1..end].trim().to_string());
                        break;
                    }
                }
            }
            monitor_name
        } else {
            panic!("Command failed with non-zero exit status");
        }
    }

    async fn get_username(&self) -> String {
        let command_user = Command::new("whoami")
            .output()
            .expect("Failed to execute command");

        if command_user.status.success() {
            let user_name = String::from_utf8_lossy(&command_user.stdout)
                .trim()
                .to_string();
            user_name
        } else {
            panic!("Failed to get the username");
        }
    }

    async fn create_file(&self, location_wallpaper: &str) -> io::Result<()> {
        let user_name = self.get_username().await;
        let monitor_name = self.detect_monitor().await.unwrap_or_default();

        // Create the template text
        let template_text = format!(
            "preload = {}\nwallpaper = {},{}\nsplash = false",
            location_wallpaper, monitor_name, location_wallpaper
        );

        // Create a copy of the user file
        let user_config_path = format!("/home/{}/.config/hypr/hyprpaper.conf", user_name);

        let mut file_hyprland_wallpaper = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(format!(
                "/home/{}/Documents/hyprpaper.conf",
                self.get_username().await
            ))?;

        // Get the user config
        let user_config = fs::read_to_string(&user_config_path)?;

        // Write the user config to the new file
        file_hyprland_wallpaper.write_all(user_config.as_bytes())?;

        // Reset file cursor to the beginning
        file_hyprland_wallpaper.seek(std::io::SeekFrom::Start(0))?;

        // Read the file line by line and write it back with replacements
        let mut contents = String::new();
        for line in io::BufReader::new(&file_hyprland_wallpaper).lines() {
            let line = line?;
            if line.contains("preload") {
                contents.push_str(&template_text);
            } else {
                contents.push_str(&line);
            }
            contents.push('\n');
        }

        // Truncate the file and write the modified contents
        file_hyprland_wallpaper.set_len(0)?;
        file_hyprland_wallpaper.seek(std::io::SeekFrom::Start(0))?;
        file_hyprland_wallpaper.write_all(contents.as_bytes())?;

        Ok(())
    }

    async fn delete_template(&self) -> Result<(), std::io::Error> {
        let delete_command = Command::new("rm")
            .arg("hyprpaper.conf")
            .output()
            .expect("Failed to execute command");

        if delete_command.status.success() {
            Ok(())
        } else {
            panic!("Error deleting template");
        }
    }

    async fn set_wallpaper(&mut self, user: &str) -> Result<(), std::io::Error> {
        let arg_path = format!("/home/{}/.config/hypr/hyprpaper.conf", user);
        let mv_command = Command::new("mv")
            .arg(format!(
                "/home/{}/Documents/hyprpaper.conf",
                self.get_username().await
            ))
            .arg(&arg_path)
            .stderr(std::process::Stdio::piped())
            .output()
            .expect("Failed to execute 'mv' command");

        // we move the new file to the folder
        if !mv_command.status.success() {
            let stderr = String::from_utf8_lossy(&mv_command.stderr);
            panic!("Error executing 'mv' command: {}", stderr);
        }

        // we kill the app in order to update the code

        let kill_hyprpaper = Command::new("killall")
            .arg("hyprpaper")
            .output()
            .expect("Failed to execute 'killall' command");

        if !kill_hyprpaper.status.success() {
            let stderr = String::from_utf8_lossy(&kill_hyprpaper.stderr);
            panic!("Error executing 'killall' command: {}", stderr);
        }

        // and with the ctl tools of hyprland we launch the app again

        let mut hyprctl_command = Command::new("hyprctl")
            .arg("dispatch")
            .arg("exec")
            .arg("hyprpaper")
            .spawn()
            .expect("Failed to execute 'hyprctl' command");

        let hyprctl_status = hyprctl_command
            .wait()
            .expect("Failed to wait on child process");

        let mv_status = mv_command.status.success();
        let kill_status = kill_hyprpaper.status.success();
        let hyper_ctl = hyprctl_status.success();
        // and if everything is okay we make the magic happends
        if mv_status && kill_status && hyper_ctl {
            Ok(())
        } else {
            panic!("Error setting the wallpaper");
        }
    }

    pub async fn generate_template(&mut self, path: &str) -> io::Result<()> {
        if std::path::Path::new("hyprpaper.conf").exists() {
            self.delete_template().await?;
        } // we create an instance of the function
        self.create_file(path).await?;
        self.set_wallpaper(&self.get_username().await).await?;
        Ok(())
    }
}
