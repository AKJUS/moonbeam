// Copyright 2024 Moonbeam foundation
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-02-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --runtime=./target/production/wbuild/moonbeam-runtime/moonbeam_runtime.wasm
// --genesis-builder=runtime
// --genesis-builder-preset=development
// --steps=50
// --repeat=20
// --pallet=pallet_assets
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./benchmarking/frame-weight-template.hbs
// --output=./runtime/moonbeam/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_assets`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for WeightInfo<T> {
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 0_000 picoseconds.
		Weight::from_parts(0, 0)
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::NextAssetId` (r:1 w:0)
	/// Proof: `Assets::NextAssetId` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3639`
		// Minimum execution time: 10_269_000 picoseconds.
		Weight::from_parts(10_582_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn start_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241`
		//  Estimated: `3639`
		// Minimum execution time: 11_568_000 picoseconds.
		Weight::from_parts(11_921_000, 3639)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:657 w:656)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:656 w:656)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 656]`.
	fn destroy_accounts(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `892 + c * (183 ±0)`
		//  Estimated: `3639 + c * (2597 ±0)`
		// Minimum execution time: 15_033_000 picoseconds.
		Weight::from_parts(15_134_000, 3639)
			// Standard Error: 6_079
			.saturating_add(Weight::from_parts(13_419_314, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2597).saturating_mul(c.into()))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Approvals` (r:657 w:656)
	/// Proof: `Assets::Approvals` (`max_values`: None, `max_size`: Some(136), added: 2611, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 656]`.
	fn destroy_approvals(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `313 + a * (74 ±0)`
		//  Estimated: `3639 + a * (2611 ±0)`
		// Minimum execution time: 15_423_000 picoseconds.
		Weight::from_parts(15_744_000, 3639)
			// Standard Error: 3_826
			.saturating_add(Weight::from_parts(5_539_306, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 2611).saturating_mul(a.into()))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:0)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(152), added: 2627, mode: `MaxEncodedLen`)
	fn finish_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `3639`
		// Minimum execution time: 12_819_000 picoseconds.
		Weight::from_parts(13_091_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `3639`
		// Minimum execution time: 21_579_000 picoseconds.
		Weight::from_parts(22_238_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `3639`
		// Minimum execution time: 29_084_000 picoseconds.
		Weight::from_parts(29_731_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `420`
		//  Estimated: `6184`
		// Minimum execution time: 45_611_000 picoseconds.
		Weight::from_parts(46_726_000, 6184)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `420`
		//  Estimated: `6184`
		// Minimum execution time: 41_327_000 picoseconds.
		Weight::from_parts(42_610_000, 6184)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `420`
		//  Estimated: `6184`
		// Minimum execution time: 45_506_000 picoseconds.
		Weight::from_parts(46_378_000, 6184)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `3639`
		// Minimum execution time: 15_707_000 picoseconds.
		Weight::from_parts(16_469_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `3639`
		// Minimum execution time: 15_768_000 picoseconds.
		Weight::from_parts(16_612_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn freeze_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241`
		//  Estimated: `3639`
		// Minimum execution time: 11_237_000 picoseconds.
		Weight::from_parts(11_713_000, 3639)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn thaw_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241`
		//  Estimated: `3639`
		// Minimum execution time: 11_446_000 picoseconds.
		Weight::from_parts(11_868_000, 3639)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:0)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(152), added: 2627, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `3639`
		// Minimum execution time: 13_111_000 picoseconds.
		Weight::from_parts(13_464_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `3639`
		// Minimum execution time: 11_229_000 picoseconds.
		Weight::from_parts(11_635_000, 3639)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(152), added: 2627, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `3639`
		// Minimum execution time: 27_379_000 picoseconds.
		Weight::from_parts(28_282_335, 3639)
			// Standard Error: 584
			.saturating_add(Weight::from_parts(1_624, 0).saturating_mul(n.into()))
			// Standard Error: 584
			.saturating_add(Weight::from_parts(1_730, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(152), added: 2627, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383`
		//  Estimated: `3639`
		// Minimum execution time: 30_864_000 picoseconds.
		Weight::from_parts(31_396_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(152), added: 2627, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `94`
		//  Estimated: `3639`
		// Minimum execution time: 12_018_000 picoseconds.
		Weight::from_parts(12_554_524, 3639)
			// Standard Error: 311
			.saturating_add(Weight::from_parts(1_077, 0).saturating_mul(n.into()))
			// Standard Error: 311
			.saturating_add(Weight::from_parts(1_758, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Metadata` (r:1 w:1)
	/// Proof: `Assets::Metadata` (`max_values`: None, `max_size`: Some(152), added: 2627, mode: `MaxEncodedLen`)
	fn force_clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `383`
		//  Estimated: `3639`
		// Minimum execution time: 30_774_000 picoseconds.
		Weight::from_parts(31_204_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn force_asset_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `3639`
		// Minimum execution time: 10_780_000 picoseconds.
		Weight::from_parts(11_153_000, 3639)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Approvals` (r:1 w:1)
	/// Proof: `Assets::Approvals` (`max_values`: None, `max_size`: Some(136), added: 2611, mode: `MaxEncodedLen`)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `241`
		//  Estimated: `3639`
		// Minimum execution time: 17_198_000 picoseconds.
		Weight::from_parts(17_871_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Approvals` (r:1 w:1)
	/// Proof: `Assets::Approvals` (`max_values`: None, `max_size`: Some(136), added: 2611, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn transfer_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `513`
		//  Estimated: `6184`
		// Minimum execution time: 54_291_000 picoseconds.
		Weight::from_parts(55_341_000, 6184)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Approvals` (r:1 w:1)
	/// Proof: `Assets::Approvals` (`max_values`: None, `max_size`: Some(136), added: 2611, mode: `MaxEncodedLen`)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `399`
		//  Estimated: `3639`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(20_708_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Approvals` (r:1 w:1)
	/// Proof: `Assets::Approvals` (`max_values`: None, `max_size`: Some(136), added: 2611, mode: `MaxEncodedLen`)
	fn force_cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `399`
		//  Estimated: `3639`
		// Minimum execution time: 20_105_000 picoseconds.
		Weight::from_parts(20_829_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn set_min_balance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `3639`
		// Minimum execution time: 11_965_000 picoseconds.
		Weight::from_parts(12_257_000, 3639)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn touch() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `435`
		//  Estimated: `3639`
		// Minimum execution time: 33_338_000 picoseconds.
		Weight::from_parts(34_546_000, 3639)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn touch_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `3639`
		// Minimum execution time: 30_237_000 picoseconds.
		Weight::from_parts(30_905_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `561`
		//  Estimated: `3639`
		// Minimum execution time: 35_445_000 picoseconds.
		Weight::from_parts(36_412_000, 3639)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	fn refund_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `353`
		//  Estimated: `3639`
		// Minimum execution time: 32_634_000 picoseconds.
		Weight::from_parts(33_594_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	fn block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `3639`
		// Minimum execution time: 15_764_000 picoseconds.
		Weight::from_parts(16_419_000, 3639)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(174), added: 2649, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(122), added: 2597, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `420`
		//  Estimated: `6184`
		// Minimum execution time: 54_229_000 picoseconds.
		Weight::from_parts(55_496_000, 6184)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}
