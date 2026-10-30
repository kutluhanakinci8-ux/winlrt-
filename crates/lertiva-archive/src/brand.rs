//! Lertiva ürün markası — kullanıcıya görünen adlar (tek kaynak).

pub const PLATFORM: &str = "Lertiva";
pub const PRODUCT_ARC: &str = "Lertiva Arc";
pub const PRODUCT_ZIP: &str = "Lertiva Zip";
pub const PRODUCT_ZIP_ALIAS: &str = "Lertiva Pack";
pub const MOTOR_CRATE: &str = "lertiva-archive";

pub fn product_zip_id() -> &'static str {
    "Lertiva Zip"
}

pub fn product_arc_id() -> &'static str {
    "Lertiva Arc"
}
