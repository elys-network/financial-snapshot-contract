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
    // USDC
    USDC,
}

impl ElysDenom {
    pub fn as_str(&self) -> &'static str {
        match self {
            ElysDenom::Elys => "uelys",
            ElysDenom::Eden => "ueden",
            ElysDenom::EdenBoost => "uedenb",
            ElysDenom::Usdc => "uusdc",
            ElysDenom::USDC => "USDC",
        }
    }
}

#[cw_serde]
pub enum EarnType {
    AllProgram = 0,
	UsdcProgram = 1,
	ElysProgram = 2,
	EdenProgram = 3,
	EdenBProgram = 4,
}