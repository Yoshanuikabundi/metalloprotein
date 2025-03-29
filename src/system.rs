//! Abstractions over differences between targets

struct FileHandle {
    #[cfg(target_arch = "wasm32")]
    handle: web_sys::File,
    #[cfg(not(target_arch = "wasm32"))]
    handle: std::path::PathBuf,
}
