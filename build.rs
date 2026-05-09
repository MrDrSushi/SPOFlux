fn main() {

    // MacOS
    #[cfg(target_os = "macos")]
    {
        // I will need to learn how to implement this thing later
    }

    // Linux
    #[cfg(target_os = "linux")]
    {
        // I need to research about EFF - something completely new to me :)
    }

    // Windows
    #[cfg(target_os = "windows")]
    {
        winresource::WindowsResource::new().compile().unwrap();
    }

}