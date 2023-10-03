use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::GetPortfolioResp, types::*};

pub fn get_portfolio(deps: Deps<ElysQuery>, address: String) -> Result<GetPortfolioResp, ContractError> {
    let portfolio : Portfolio = PORTFOLIO.load(deps.storage, &address)?;
    let resp = GetPortfolioResp {
        portfolio: portfolio,
    };

    Ok(resp)
}
