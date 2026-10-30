/// E-posta eki ZIP üst boyut hedefi (I6 / winlrtZIP W7).
pub const EMAIL_ATTACHMENT_MAX_ZIP_BYTES: u64 = 10 * 1024 * 1024;

/// Karışık ~10 MiB ham fixture için e-posta profili oluşturma üst süre (ms, CI).
pub const EMAIL_ATTACHMENT_MIXED_MAX_CREATE_MS: u128 = 120_000;

/// Sıkıştırma profilleri — rekabet KPI’larına göre (Faz 1+).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionProfile {
    /// KPI: hız (P0)
    Fast,
    #[default]
    Balanced,
    /// KPI: sıkıştırma oranı (P0)
    Maximum,
    /// E-posta eki boyut limiti
    EmailAttachment,
}

impl CompressionProfile {
    pub fn zstd_level(self) -> i32 {
        match self {
            Self::Fast => 0,
            Self::Balanced => 6,
            Self::Maximum => 19,
            Self::EmailAttachment => 9,
        }
    }

    pub fn email_attachment_size_ok(zip_bytes: u64) -> bool {
        zip_bytes <= EMAIL_ATTACHMENT_MAX_ZIP_BYTES
    }
}
