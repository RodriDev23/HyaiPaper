use std::io::{self, Write};
use std::process::Command;
use std::str;

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
        // Create the template text
        let monitor_name = self.detect_monitor().await.unwrap_or_default();
        let template_text = format!(
            "preload = {}\nwallpaper = {},{}\nsplash = false\n",
            location_wallpaper, monitor_name, location_wallpaper
        );

        // Create the file and write the template text
        let mut file_hyprland_wallpaper = std::fs::File::create("hyprpaper.conf")?;
        file_hyprland_wallpaper.write_all(template_text.as_bytes())?;
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

    async fn set_wallpaper(user: &str) -> Result<(), std::io::Error> {
        let arg_path = format!("/home/{}/.config/hypr/hyprpaper.conf", user);
        let mv_command = Command::new("mv")
            .arg("hyprpaper.conf")
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

    pub async fn generate_template(&self, path: &str) -> io::Result<()> {
        if std::path::Path::new("hyprpaper.conf").exists() {
            self.delete_template().await?;
        } // we create an instance of the function
        self.create_file(path).await?;
        WallpaperManager::set_wallpaper(&self.get_username().await).await?;
        Ok(())
    }
}
