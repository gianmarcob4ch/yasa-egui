# Yasa
A multi platform screenshot utility written in Rust. 

The program has been ran and tested in
- Fedora Workstation 39, Gnome 45
- macOS Sonoma 14.3.1
- Windows 11 (on ARM e x64)

## Features
- Multiscreen capture (partial or full-screen)
- Delay timer: delays the capture for the desired time in seconds
- Easily accessible User Interface - egui 0.22.0 (egui-extras, egui-toast, egui-modal)
- Take notes on screenshot
- Crop: it is possible to crop the capture afterwards
- Multi-format save to drive (PNG, JPEG, GIF)
- Clipboard support
- Hotkeys support (not global)



## Testing locally Linux and macOS

``` bash
git clone https://github.com/gianmarcob4ch/yasa-egui.git
cd Yasa-egui
cargo run
```

