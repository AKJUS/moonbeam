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

//! Autogenerated weights for `pallet_referenda`
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
// --runtime=./target/production/wbuild/moonbase-runtime/moonbase_runtime.wasm
// --genesis-builder=runtime
// --genesis-builder-preset=development
// --steps=50
// --repeat=20
// --pallet=pallet_referenda
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./benchmarking/frame-weight-template.hbs
// --output=./runtime/moonbase/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: `Referenda::ReferendumCount` (r:1 w:1)
	/// Proof: `Referenda::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:0 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `42428`
		// Minimum execution time: 34_267_000 picoseconds.
		Weight::from_parts(35_088_000, 42428)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `510`
		//  Estimated: `83866`
		// Minimum execution time: 47_970_000 picoseconds.
		Weight::from_parts(49_175_000, 83866)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:0)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3303`
		//  Estimated: `42428`
		// Minimum execution time: 68_531_000 picoseconds.
		Weight::from_parts(69_869_000, 42428)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:0)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3323`
		//  Estimated: `42428`
		// Minimum execution time: 68_740_000 picoseconds.
		Weight::from_parts(69_731_000, 42428)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:1)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `510`
		//  Estimated: `83866`
		// Minimum execution time: 59_684_000 picoseconds.
		Weight::from_parts(61_002_000, 83866)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:1)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `510`
		//  Estimated: `83866`
		// Minimum execution time: 53_998_000 picoseconds.
		Weight::from_parts(55_547_000, 83866)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `415`
		//  Estimated: `4377`
		// Minimum execution time: 29_384_000 picoseconds.
		Weight::from_parts(30_120_000, 4377)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `378`
		//  Estimated: `4377`
		// Minimum execution time: 28_893_000 picoseconds.
		Weight::from_parts(29_347_000, 4377)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `83866`
		// Minimum execution time: 32_118_000 picoseconds.
		Weight::from_parts(32_739_000, 83866)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::MetadataOf` (r:1 w:0)
	/// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `969`
		//  Estimated: `83866`
		// Minimum execution time: 95_762_000 picoseconds.
		Weight::from_parts(98_188_000, 83866)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Referenda::TrackQueue` (r:1 w:0)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:1)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `102`
		//  Estimated: `5477`
		// Minimum execution time: 10_568_000 picoseconds.
		Weight::from_parts(11_181_000, 5477)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3090`
		//  Estimated: `42428`
		// Minimum execution time: 46_961_000 picoseconds.
		Weight::from_parts(48_069_000, 42428)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3090`
		//  Estimated: `42428`
		// Minimum execution time: 48_954_000 picoseconds.
		Weight::from_parts(50_510_000, 42428)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2915`
		//  Estimated: `5477`
		// Minimum execution time: 26_464_000 picoseconds.
		Weight::from_parts(27_806_000, 5477)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2915`
		//  Estimated: `5477`
		// Minimum execution time: 25_980_000 picoseconds.
		Weight::from_parts(27_290_000, 5477)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:0)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2919`
		//  Estimated: `5477`
		// Minimum execution time: 31_666_000 picoseconds.
		Weight::from_parts(32_744_000, 5477)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:0)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:1 w:1)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2939`
		//  Estimated: `5477`
		// Minimum execution time: 31_869_000 picoseconds.
		Weight::from_parts(33_098_000, 5477)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `249`
		//  Estimated: `42428`
		// Minimum execution time: 20_956_000 picoseconds.
		Weight::from_parts(21_682_000, 42428)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `42428`
		// Minimum execution time: 21_169_000 picoseconds.
		Weight::from_parts(21_896_000, 42428)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `194`
		//  Estimated: `4377`
		// Minimum execution time: 13_752_000 picoseconds.
		Weight::from_parts(14_455_000, 4377)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:1)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `42428`
		// Minimum execution time: 27_758_000 picoseconds.
		Weight::from_parts(28_723_000, 42428)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:1 w:1)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `42428`
		// Minimum execution time: 29_338_000 picoseconds.
		Weight::from_parts(30_239_000, 42428)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `338`
		//  Estimated: `42428`
		// Minimum execution time: 25_131_000 picoseconds.
		Weight::from_parts(26_000_000, 42428)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `321`
		//  Estimated: `42428`
		// Minimum execution time: 24_617_000 picoseconds.
		Weight::from_parts(25_164_000, 42428)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `338`
		//  Estimated: `42428`
		// Minimum execution time: 23_782_000 picoseconds.
		Weight::from_parts(24_511_000, 42428)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `342`
		//  Estimated: `42428`
		// Minimum execution time: 23_605_000 picoseconds.
		Weight::from_parts(24_262_000, 42428)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `342`
		//  Estimated: `83866`
		// Minimum execution time: 35_378_000 picoseconds.
		Weight::from_parts(36_503_000, 83866)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `338`
		//  Estimated: `42428`
		// Minimum execution time: 25_299_000 picoseconds.
		Weight::from_parts(26_034_000, 42428)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::MetadataOf` (r:0 w:1)
	/// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `338`
		//  Estimated: `4377`
		// Minimum execution time: 20_962_000 picoseconds.
		Weight::from_parts(21_515_000, 4377)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Referenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(912), added: 3387, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::MetadataOf` (r:1 w:1)
	/// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `271`
		//  Estimated: `4377`
		// Minimum execution time: 16_532_000 picoseconds.
		Weight::from_parts(17_087_000, 4377)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
