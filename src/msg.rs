use cosmwasm_std::{Addr, Binary};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(cw_orch::QueryFns)] // Function generation
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(cw2::ContractVersion)]
    ContractVersion {},
    #[returns(AggregateResult)]
    Aggregate {
        queries: Vec<Call>,
    },
    #[returns(AggregateResult)]
    TryAggregate {
        require_success: Option<bool>,
        include_cause: Option<bool>,
        queries: Vec<Call>,
    },
    #[returns(AggregateResult)]
    TryAggregateOptional {
        include_cause: Option<bool>,
        queries: Vec<CallOptional>,
    },
    #[returns(BlockAggregateResult)]
    BlockAggregate {
        queries: Vec<Call>,
    },
    #[returns(BlockAggregateResult)]
    BlockTryAggregate {
        require_success: Option<bool>,
        include_cause: Option<bool>,
        queries: Vec<Call>,
    },
    #[returns(BlockAggregateResult)]
    BlockTryAggregateOptional {
        include_cause: Option<bool>,
        queries: Vec<CallOptional>,
    },
}

#[cw_serde]
pub struct Call {
    pub address: Addr,
    pub data: Binary,
}

#[cw_serde]
pub struct CallOptional {
    pub require_success: bool,
    pub address: Addr,
    pub data: Binary,
}


#[derive(Default)]
#[cw_serde]
pub struct CallResult {
    pub success: bool,
    pub data: Binary,
}

#[cw_serde]
pub struct AggregateResult {
    pub return_data: Vec<CallResult>,
}

#[cw_serde]
pub struct BlockAggregateResult {
    pub block: u64,
    pub return_data: Vec<CallResult>,
}

impl AggregateResult {
    pub fn from_return_data(return_data: Vec<CallResult>) -> AggregateResult {
        AggregateResult { return_data }
    }
}

impl BlockAggregateResult {
    pub fn from_return_data(block: u64, return_data: Vec<CallResult>) -> BlockAggregateResult {
        BlockAggregateResult { block, return_data }
    }
}
