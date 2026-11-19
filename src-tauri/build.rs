fn main() {
    //the name windows shows in its screen saver list (string 1), beside tauri's icon and version
    #[cfg(windows)]
    embed_resource::compile("screensaver.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
    tauri_build::build()
}
