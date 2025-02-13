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

//! Autogenerated weights for `pallet_author_mapping`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-11-11, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! WASM-EXECUTION: Compiled, CHAIN: Some("moonriver-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/moonbeam
// benchmark
// pallet
// --chain=moonriver-dev
// --steps=50
// --repeat=20
// --pallet=pallet_author_mapping
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

/// Weights for `pallet_author_mapping`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_author_mapping::WeightInfo for WeightInfo<T> {
	/// Storage: `AuthorMapping::MappingWithDeposit` (r:1 w:1)
	/// Proof: `AuthorMapping::MappingWithDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `AuthorMapping::NimbusLookup` (r:0 w:1)
	/// Proof: `AuthorMapping::NimbusLookup` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `3808`
		// Minimum execution time: 28_155_000 picoseconds.
		Weight::from_parts(28_848_000, 3808)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `AuthorMapping::MappingWithDeposit` (r:2 w:2)
	/// Proof: `AuthorMapping::MappingWithDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorMapping::NimbusLookup` (r:0 w:1)
	/// Proof: `AuthorMapping::NimbusLookup` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `292`
		//  Estimated: `6232`
		// Minimum execution time: 16_943_000 picoseconds.
		Weight::from_parts(17_803_000, 6232)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `AuthorMapping::MappingWithDeposit` (r:1 w:1)
	/// Proof: `AuthorMapping::MappingWithDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `AuthorMapping::NimbusLookup` (r:0 w:1)
	/// Proof: `AuthorMapping::NimbusLookup` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn clear_association() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `420`
		//  Estimated: `3885`
		// Minimum execution time: 30_731_000 picoseconds.
		Weight::from_parts(31_557_000, 3885)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `AuthorMapping::NimbusLookup` (r:1 w:1)
	/// Proof: `AuthorMapping::NimbusLookup` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorMapping::MappingWithDeposit` (r:1 w:1)
	/// Proof: `AuthorMapping::MappingWithDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn remove_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `514`
		//  Estimated: `3979`
		// Minimum execution time: 34_285_000 picoseconds.
		Weight::from_parts(35_331_000, 3979)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `AuthorMapping::NimbusLookup` (r:1 w:1)
	/// Proof: `AuthorMapping::NimbusLookup` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AuthorMapping::MappingWithDeposit` (r:1 w:1)
	/// Proof: `AuthorMapping::MappingWithDeposit` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn set_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `399`
		//  Estimated: `3864`
		// Minimum execution time: 33_865_000 picoseconds.
		Weight::from_parts(34_558_000, 3864)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}
