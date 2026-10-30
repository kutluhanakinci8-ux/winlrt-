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
            Self::Fast => 1,
            Self::Balanced => 6,
            Self::Maximum => 19,
            Self::EmailAttachment => 9,
        }
    }
}
