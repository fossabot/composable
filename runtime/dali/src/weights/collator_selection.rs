
//! Autogenerated weights for `collator_selection`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=runtime/dali/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `collator_selection`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> collator_selection::WeightInfo for WeightInfo<T> {
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:0 w:1)
	fn set_invulnerables(b: u32, ) -> Weight {
		(15_026_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((7_804_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	fn set_desired_candidates() -> Weight {
		(20_367_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	fn set_candidacy_bond() -> Weight {
		(20_839_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	fn register_as_candidate(c: u32, ) -> Weight {
		(163_287_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((337_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	fn leave_intent(c: u32, ) -> Weight {
		(150_345_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((372_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	fn note_author() -> Weight {
		(87_052_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection LastAuthoredBlock (r:1000 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	fn new_session(r: u32, c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 3_552_000
			.saturating_add((19_846_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 3_552_000
			.saturating_add((97_177_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
}
