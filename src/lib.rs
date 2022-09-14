#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use crate::weights::WeightInfo;


    #[pallet::pallet]
    #[pallet::without_storage_info]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        Initialized(T::AccountId),
        Transfer(T::AccountId, T::AccountId, u64),
    }

    #[pallet::storage]
    #[pallet::getter(fn get_balance)]
    pub(super) type Balances<T: Config> =
    StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

    #[pallet::type_value]
    pub(super) fn TotalSupplyDefaultValue<T: Config>() -> u64 {
        30000000
    }

    #[pallet::storage]
    pub(super) type TotalSupply<T: Config> =
    StorageValue<_, u64, ValueQuery, TotalSupplyDefaultValue<T>>;

    #[pallet::storage]
    pub(super) type Init<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::error]
    pub enum Error<T> {
        AlreadyInitialized,
        InsufficientFunds,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::init())]
        pub fn init(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            ensure!(!<Init<T>>::get(),<Error<T>>::AlreadyInitialized);

            <Balances<T>>::insert(sender.clone(),<TotalSupply<T>>::get());

            Init::<T>::put(true);
            Self::deposit_event(Event::Initialized(sender));
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::transfer())]
        pub fn transfer(origin: OriginFor<T>,to: T::AccountId, value: u64)
        -> DispatchResultWithPostInfo{
            let sender = ensure_signed(origin)?;
            let sender_balance = <Balances<T>>::get(&sender);
            let receiver_balance = <Balances<T>>::get(&to);

            //Calculate new balances
            let updated_from_balance = sender_balance
                .checked_sub(value)
                .ok_or(Error::<T>::InsufficientFunds)?;
            let updated_to_balance = receiver_balance
                .checked_add(value)
                .expect("Entire supply fits in u64");

            <Balances<T>>::insert(&sender,updated_from_balance);
            <Balances<T>>::insert(&to,updated_to_balance);

            Self::deposit_event(Event::Transfer(sender,to,value));
            Ok(().into())
        }
    }
}