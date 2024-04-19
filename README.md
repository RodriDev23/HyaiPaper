# HyaiPaper


HyaiPaper is a wallpaper selector designed to provide a faster experience for users in Hyprland.
Features

![Alt Text](video2.gif)


    Quickly browse and select wallpapers.
    High-performance wallpaper rendering.
    Easy-to-use interface.


## Before Installation

Before installing HyaiPaper, it's recommended to make a backup copy of your existing `hyprpaper.conf` configuration file in case of any errors during installation or using it. You can do this by copying the file to a safe location.

```bash
cp ~/.config/hypr/hyprpaper.conf ~/Documents/hyprpaper_backup.conf
```

Installation

To install HyaiPaper, follow these steps:

    Ensure your system has the following packages installed:
        Hyprland
        Hyprpaper
        Hyprctl

You can install it doing:
  
```bash
 sudo pacman -S hyprland hyprpaper hyprctl
```


Usage

You can download the latest release of HyaiPaper from [here](https://github.com/RodriDev23/HyaiPaper/releases).

After downloading, navigate to the downloaded directory and make the app executable:

```bash 
chmod a+x Hyai-paper.appImage
```

Finally, move the app to the bin folder to easily launch it with your favorite launcher like rofi:

```bash
sudo mv Hyai-paper /bin
```

After installing and starting HyaiPaper, follow these steps:

    Ensure your wallpapers are stored in the "Pictures" folder.
    Launch your favorite launcher app and select the HyaiPaper application.
    Start customizing your screen by browsing and selecting wallpapers.

Contributing

Contributions are welcome! Here's how you can contribute to HyaiPaper:

    Fork the repository.
    Create a new branch: git checkout -b feature-name
    Make your changes.
    Commit your changes: git commit -m 'Add new feature'
    Push to the branch: git push origin feature-name
    Submit a pull request.

License

This project is licensed under the MIT License - see the LICENSE file for details.
Acknowledgements

    Thanks to the developers of Tauri for providing the framework.
    Special thanks to the Hyprland community for their support and feedback.
