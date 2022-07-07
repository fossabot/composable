
//! Autogenerated weights for `assets_registry`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=runtime/picasso/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `assets_registry`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> assets_registry::WeightInfo for WeightInfo<T> {
	// Storage: AssetsRegistry ForeignToLocal (r:1 w:1)
	// Storage: unknown [0x3a7472616e73616374696f6e5f6c6576656c3a] (r:1 w:1)
	// Storage: CurrencyFactory AssetIdRanges (r:1 w:1)
	// Storage: AssetsRegistry AssetRatio (r:1 w:1)
	// Storage: CurrencyFactory AssetEd (r:0 w:1)
	// Storage: AssetsRegistry LocalToForeign (r:0 w:1)
	fn register_asset() -> Weight {
		(49_716_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: AssetsRegistry AssetRatio (r:1 w:1)
	// Storage: AssetsRegistry LocalToForeign (r:0 w:1)
	// Storage: AssetsRegistry ForeignToLocal (r:0 w:1)
	fn update_asset() -> Weight {
		(32_407_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: AssetsRegistry MinFeeAmounts (r:1 w:1)
	fn set_min_fee() -> Weight {
		(27_065_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
