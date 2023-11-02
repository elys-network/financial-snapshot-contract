pub enum ElysDenom {
    // Elys
    Elys,
    // Eden
    Eden,
    // Eden Boost
    EdenBoost,
    // Usdc
    Usdc,
}

impl ElysDenom {
    fn as_str(&self) -> &'static str {
        match self {
            ElysDenom::Elys => "uelys",
            ElysDenom::Eden => "ueden",
            ElysDenom::EdenBoost => "uedenb",
            ElysDenom::Usdc => "uusdc",
        }
    }
}