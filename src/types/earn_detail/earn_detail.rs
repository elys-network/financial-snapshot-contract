use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub struct AprUsdc {
    pub uusdc: u64,
    pub ueden: u64,
}

#[cw_serde]
pub struct AprElys {
    pub uusdc: u64,
    pub ueden: u64,
    pub uedenb: u64,
}

#[cw_serde]
pub struct BalanceBorrowed {
    pub usd_amount: Decimal,
    pub percentage: Decimal,
}

#[cw_serde]
pub struct BalanceAvailable {
    pub amount: u64,
    pub usd_amount: Decimal,
}

#[cw_serde]
pub struct BalanceReward {
    pub asset: String,
    pub amount: u64,
    pub usd_amount: Option<Decimal>,
}

#[cw_serde]
pub struct VestingDetail {
    pub total_vest: BalanceAvailable,
    pub balance_vested: BalanceAvailable,
    pub remaining_vest: BalanceAvailable,
    pub remaining_time: u64,
}

#[cw_serde]
pub struct ValidatorDetail {
    // The validator ID.
    pub id: String,
    // The validator name.
    pub name: String,
    // Voting power percentage for this validator.
    pub voting_power: u64,
    // Comission percentage for the validator.
    pub comission: Decimal,
    // The url of the validator profile picture
    pub profile_picture_src: Option<String>,
}

#[cw_serde]
pub struct StakedPosition {
    // The position ID.
    pub id: String,
    // The validator that's being unstaked from.
    pub validator: ValidatorDetail,
    // The amount that's being staked.
    pub staked: BalanceAvailable,
}

#[cw_serde]
pub struct UnstakedPosition {
    // The position ID.
    pub id: String,
    // The validator that's being unstaked from.
    pub validator: ValidatorDetail,
    pub remaining_time: u64, // Remaining time to unstake in days.
    // The amount that's being staked.
    pub unstaked: BalanceAvailable,
}