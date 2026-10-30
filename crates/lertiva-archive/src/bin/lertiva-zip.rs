fn main() {
    if let Err(e) = lertiva_archive::zip_product_cli::run() {
        eprintln!("hata: {e}");
        std::process::exit(1);
    }
}
