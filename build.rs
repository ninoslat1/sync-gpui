fn main() {
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("public/icon.ico");
        res.compile().expect("Failed to embed icon");
    }
}