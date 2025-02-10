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

//! Autogenerated weights for `pallet_relay_storage_roots`
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
// --pallet=pallet_relay_storage_roots
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

/// Weights for `pallet_relay_storage_roots`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_relay_storage_roots::WeightInfo for WeightInfo<T> {
	/// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	/// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `RelayStorageRoots::RelayStorageRoot` (r:1 w:2)
	/// Proof: `RelayStorageRoots::RelayStorageRoot` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `RelayStorageRoots::RelayStorageRootKeys` (r:1 w:1)
	/// Proof: `RelayStorageRoots::RelayStorageRootKeys` (`max_values`: Some(1), `max_size`: Some(121), added: 616, mode: `MaxEncodedLen`)
	fn set_relay_storage_root() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `972`
		//  Estimated: `3509`
		// Minimum execution time: 22_352_000 picoseconds.
		Weight::from_parts(23_127_000, 3509)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}
