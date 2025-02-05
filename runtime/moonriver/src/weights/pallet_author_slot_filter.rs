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

//! Autogenerated weights for `pallet_author_slot_filter`
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
// --runtime=./target/production/wbuild/moonriver-runtime/moonriver_runtime.wasm
// --genesis-builder=runtime
// --genesis-builder-preset=development
// --steps=50
// --repeat=20
// --pallet=pallet_author_slot_filter
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./benchmarking/frame-weight-template.hbs
// --output=./runtime/moonriver/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_author_slot_filter`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_author_slot_filter::WeightInfo for WeightInfo<T> {
	/// Storage: `AuthorFilter::EligibleCount` (r:0 w:1)
	/// Proof: `AuthorFilter::EligibleCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_eligible() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_092_000 picoseconds.
		Weight::from_parts(5_324_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
