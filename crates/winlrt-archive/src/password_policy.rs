use serde::Serialize;

/// Parola politikası (Faz 4 beta — kurumsal pilot).
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct PasswordPolicy {
    pub id: &'static str,
    pub label: &'static str,
    pub min_length: u32,
    pub require_upper: bool,
    pub require_lower: bool,
    pub require_digit: bool,
    pub require_symbol: bool,
}

impl PasswordPolicy {
    pub fn consumer() -> Self {
        Self {
            id: "consumer",
            label: "Bireysel (beta)",
            min_length: 8,
            require_upper: false,
            require_lower: false,
            require_digit: false,
            require_symbol: false,
        }
    }

    pub fn corporate() -> Self {
        Self {
            id: "corporate",
            label: "Kurumsal (KVKK pilot)",
            min_length: 12,
            require_upper: true,
            require_lower: true,
            require_digit: true,
            require_symbol: true,
        }
    }

    pub fn violations(&self, password: &str) -> Vec<&'static str> {
        let len = password.chars().count() as u32;
        let mut out = Vec::new();
        if len < self.min_length {
            out.push("min_length");
        }
        if self.require_upper && !password.chars().any(|c| c.is_ascii_uppercase()) {
            out.push("upper");
        }
        if self.require_lower && !password.chars().any(|c| c.is_ascii_lowercase()) {
            out.push("lower");
        }
        if self.require_digit && !password.chars().any(|c| c.is_ascii_digit()) {
            out.push("digit");
        }
        if self.require_symbol && !password.chars().any(|c| !c.is_ascii_alphanumeric()) {
            out.push("symbol");
        }
        out
    }

    pub fn passes(&self, password: &str) -> bool {
        self.violations(password).is_empty()
    }
}

/// `WINLRT_PASSWORD_POLICY=corporate` → kurumsal; aksi halde bireysel.
pub fn active_policy() -> PasswordPolicy {
    match std::env::var("WINLRT_PASSWORD_POLICY")
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "corporate" | "kurumsal" | "enterprise" => PasswordPolicy::corporate(),
        _ => PasswordPolicy::consumer(),
    }
}

/// Explorer `--secure` menüsü kurumsal politikayı varsayılan uygular (override: env).
pub fn policy_for_secure_create() -> PasswordPolicy {
    if std::env::var("WINLRT_PASSWORD_POLICY")
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
    {
        return active_policy();
    }
    PasswordPolicy::corporate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corporate_requires_classes() {
        let p = PasswordPolicy::corporate();
        assert!(!p.passes("short"));
        assert!(!p.passes("alllowercase12!"));
        assert!(p.passes("Winlrt-Pilot12"));
    }
}
