use cosmwasm_schema::cw_serde;

#[cw_serde]
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
    pub fn as_str(&self) -> &'static str {
        match self {
            ElysDenom::Elys => "uelys",
            ElysDenom::Eden => "ueden",
            ElysDenom::EdenBoost => "uedenb",
            ElysDenom::Usdc => "uusdc",
        }
    }
}