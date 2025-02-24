pub mod constants;
pub mod enums;
pub mod escrow;
pub mod events;
pub mod kvstore;
pub mod macros;
pub mod staking;
pub mod types;

pub mod graphql {
	pub mod escrow_query;
	pub mod escrows_query;
	pub mod staking_leader_query;
	pub mod staking_leaders_query;
	pub mod staking_rewards_query;
}

#[tokio::main]
async fn main() -> web3::Result<()> {
	Ok(())
}
