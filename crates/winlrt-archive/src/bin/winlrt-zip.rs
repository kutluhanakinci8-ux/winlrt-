fn main() {
    if let Err(e) = winlrt_archive::zip_product_cli::run() {
        eprintln!("hata: {e}");
        std::process::exit(1);
    }
}
