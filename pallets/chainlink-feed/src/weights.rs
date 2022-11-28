//! Autogenerated weights for pallet_chainlink_feed
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-02, STEPS: `[20, ]`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE:
//! `[]` EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE:
//! 128

// Executed Command:
// ./target/debug/node-template
// benchmark
// --execution
// wasm
// --wasm-execution
// compiled
// -p
// pallet_chainlink_feed
// -e
// *
// -s
// 20
// -r
// 10
// --raw
// --output
// ../pallet-chainlink-feed/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use sp_std::marker::PhantomData;
use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for SubstrateWeight<T> {
	fn create_feed(o: u32) -> Weight {
		Weight::from_ref_time(73_153_000)
			// Standard Error: 233_000
			.saturating_add(Weight::from_ref_time(25_403_000).saturating_mul(o as u64))
			.saturating_add(DbWeight::get().reads(2))
			.saturating_add(DbWeight::get().reads((2u64).saturating_mul(o as u64)))
			.saturating_add(DbWeight::get().writes(3))
			.saturating_add(DbWeight::get().writes((2u64).saturating_mul(o as u64)))
	}
	fn transfer_ownership() -> Weight {
		Weight::from_ref_time(35_000_000)
			.saturating_add(DbWeight::get().reads(1 as u64))
			.saturating_add(DbWeight::get().writes(1 as u64))
	}
	fn cancel_ownership_transfer() -> Weight {
		Weight::from_ref_time(35_000_000)
			.saturating_add(DbWeight::get().reads(1 as u64))
			.saturating_add(DbWeight::get().writes(1 as u64))
	}
	fn accept_ownership() -> Weight {
		Weight::from_ref_time(35_000_000)
			.saturating_add(DbWeight::get().reads(1))
			.saturating_add(DbWeight::get().writes(1))
	}
	fn set_pruning_window(o: u32) -> Weight {
		Weight::from_ref_time(6_280_000)
			// Standard Error: 22_000
			.saturating_add(Weight::from_ref_time(5_429_000).saturating_mul(o as u64))
			.saturating_add(DbWeight::get().reads(1))
			.saturating_add(DbWeight::get().writes((2u64).saturating_mul(o as u64)))
	}
	fn submit_opening_round_answers() -> Weight {
		Weight::from_ref_time(149_000_000)
			.saturating_add(DbWeight::get().reads(6))
			.saturating_add(DbWeight::get().writes(6))
	}
	fn submit_closing_answer(_o: u32) -> Weight {
		Weight::from_ref_time(123_300_000)
			.saturating_add(DbWeight::get().reads(7))
			.saturating_add(DbWeight::get().writes(6))
	}
	fn change_oracles(d: u32, n: u32) -> Weight {
		Weight::from_ref_time(0)
			// Standard Error: 272_000
			.saturating_add(Weight::from_ref_time(23_471_000).saturating_mul(d as u64))
			// Standard Error: 272_000
			.saturating_add(Weight::from_ref_time(28_220_000).saturating_mul(n as u64))
			.saturating_add(DbWeight::get().reads(1))
			.saturating_add(DbWeight::get().reads((1u64).saturating_mul(d as u64)))
			.saturating_add(DbWeight::get().reads((2u64).saturating_mul(n as u64)))
			.saturating_add(DbWeight::get().writes(1))
			.saturating_add(DbWeight::get().writes((1u64).saturating_mul(d as u64)))
			.saturating_add(DbWeight::get().writes((2u64).saturating_mul(n as u64)))
	}
	fn update_future_rounds() -> Weight {
		Weight::from_ref_time(34_000_000)
			.saturating_add(DbWeight::get().reads(1))
			.saturating_add(DbWeight::get().writes(1))
	}
	fn set_requester() -> Weight {
		Weight::from_ref_time(38_000_000)
			.saturating_add(DbWeight::get().reads(2))
			.saturating_add(DbWeight::get().writes(1))
	}
	fn remove_requester() -> Weight {
		Weight::from_ref_time(37_000_000)
			.saturating_add(DbWeight::get().reads(2))
			.saturating_add(DbWeight::get().writes(1))
	}
	fn request_new_round() -> Weight {
		Weight::from_ref_time(76_000_000)
			.saturating_add(DbWeight::get().reads(4))
			.saturating_add(DbWeight::get().writes(4))
	}
	fn withdraw_payment() -> Weight {
		Weight::from_ref_time(111_000_000)
			.saturating_add(DbWeight::get().reads(3))
			.saturating_add(DbWeight::get().writes(3))
	}
	fn transfer_admin() -> Weight {
		Weight::from_ref_time(30_000_000)
			.saturating_add(DbWeight::get().reads(1))
			.saturating_add(DbWeight::get().writes(1))
	}
	fn cancel_admin_transfer() -> Weight {
		Weight::from_ref_time(30_000_000)
			.saturating_add(DbWeight::get().reads(1))
			.saturating_add(DbWeight::get().writes(1))
	}
	fn accept_admin() -> Weight {
		Weight::from_ref_time(30_000_000)
			.saturating_add(DbWeight::get().reads(1))
			.saturating_add(DbWeight::get().writes(1))
	}
	fn withdraw_funds() -> Weight {
		Weight::from_ref_time(79_000_000)
			.saturating_add(DbWeight::get().reads(3))
			.saturating_add(DbWeight::get().writes(2))
	}
	fn reduce_debt() -> Weight {
		Weight::from_ref_time(50_000_000)
			.saturating_add(DbWeight::get().reads(2))
			.saturating_add(DbWeight::get().writes(2))
	}
	fn transfer_pallet_admin() -> Weight {
		Weight::from_ref_time(30_000_000)
			.saturating_add(DbWeight::get().reads(2))
			.saturating_add(DbWeight::get().writes(1))
	}
	fn cancel_pallet_admin_transfer() -> Weight {
		Weight::from_ref_time(29_000_000)
			.saturating_add(DbWeight::get().reads(2))
			.saturating_add(DbWeight::get().writes(1))
	}
	fn accept_pallet_admin() -> Weight {
		Weight::from_ref_time(29_000_000)
			.saturating_add(DbWeight::get().reads(1))
			.saturating_add(DbWeight::get().writes(2))
	}
	fn set_feed_creator() -> Weight {
		Weight::from_ref_time(27_000_000)
			.saturating_add(DbWeight::get().reads(1))
			.saturating_add(DbWeight::get().writes(1))
	}
	fn remove_feed_creator() -> Weight {
		Weight::from_ref_time(27_000_000)
			.saturating_add(DbWeight::get().reads(1))
			.saturating_add(DbWeight::get().writes(1))
	}
}
