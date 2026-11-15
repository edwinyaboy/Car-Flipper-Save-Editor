fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
    res.set("ProductName", "Car Flipper Save Editor");
    res.set(
        "FileDescription",
        "Save editor and mod tool for Car Flipper",
    );
    res.set("CompanyName", "Shrine");
    res.set("LegalCopyright", "Copyright 2026 Shrine");
    res.set("ProductVersion", "1.0.0.0");
    res.set("FileVersion", "1.0.0.0");
    res.set_language(0x0409);
    if let Err(e) = res.compile() {
        eprintln!("winres compile failed: {}", e);
    }
}
