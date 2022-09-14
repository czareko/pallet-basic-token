#![cfg_attr(rustfmt, rustfmt_skip)]
use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;


pub trait WeightInfo {
    fn init() -> Weight;
    fn transfer() -> Weight;
}
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn init() -> Weight {
        Weight::from_ref_time(27_167_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    fn transfer() -> Weight {
        Weight::from_ref_time(41_860_000 as u64)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
}

impl WeightInfo for () {
    fn init() -> Weight {
        Weight::from_ref_time(27_167_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    fn transfer() -> Weight {
        Weight::from_ref_time(41_860_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
}