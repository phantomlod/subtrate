#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;
use scale_info::TypeInfo;
pub type Id = u32;
use sp_runtime::ArithmeticError;
use frame_support::traits::Currency;
use frame_support::traits::Time;
use frame_support::traits::Randomness;
use frame_support::dispatch::fmt;
use sp_runtime::traits::Hash;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
type TimeOf<T> = <<T as Config>::Time as Time>::Moment;
#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	#[derive(Clone, Encode, Decode, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: T::Hash,
		pub price: BalanceOf<T>,
		pub gender: Gender,
		pub owner: T::AccountId,
		pub created_date : TimeOf<T>,
	}
	impl<T: Config> fmt::Debug for Kitty<T> {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Kitty")
			 .field("dna", &self.dna)
			 .field("price", &self.price)
			 .field("gender", &self.gender)
			 .field("owner", &self.owner)
			 .field("created_date", &self.created_date)
			 .finish()
		}
	}
	#[derive(Clone, Encode, Decode, PartialEq, Copy, TypeInfo, MaxEncodedLen, Debug)]
	pub enum Gender {
		Male,
		Female,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
		type Time: Time ;
		type Randomness: Randomness<Self::Hash,Self::BlockNumber>;
		#[pallet::constant] // put the constant in metadata
		/// Maximum amount added per invocation.
		type MaxAddend: Get<u32>;

		
	}
	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn max_value)]
	pub(super) type MaxValue<T: Config> = StorageValue<_, u32, ValueQuery>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage

	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	pub type KittyId<T> = StorageValue<_, Id, ValueQuery>;


	//key : dna
	//value: kitty
	#[pallet::storage]
	#[pallet::getter(fn get_kitty)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Kitty<T>, OptionQuery>;

	//key:owner
	//value: kitty
	#[pallet::storage]
	#[pallet::getter(fn get_kitty_accountId)]
	pub(super) type KittiesAccountId<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Kitty<T>, OptionQuery>;


	//key : T ::AccountId
	//value: BoundedVec
	#[pallet::storage]
	#[pallet::getter(fn kitty_owned)]
	pub(super) type KittiesOwned<T: Config> = StorageMap<_, Blake2_128Concat,T::AccountId, BoundedVec<T::Hash, T::MaxAddend>, ValueQuery>;

	//key : T ::AccountId
	//value: vec<dna>
	#[pallet::storage]
	#[pallet::getter(fn kitty_owned_vec)]
	pub(super) type KittiesOwnedVec<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		/// A new kitty was successfully created.
		Created { kitty: T::Hash, owner: T::AccountId },
		Transferred { from: T::AccountId, to: T::AccountId, kitty:T::Hash },

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		DuplicateKitty,
		TooManyOwned,
		NoKitty,
		NotOwner,
		TransferToSelf,
		ExceedLimit,
	}

	#[pallet::genesis_config]

	pub struct GenesisConfig<T: Config> {
		pub kitties: Vec<(T::AccountId, Vec<u8>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig { 
				kitties: Default::default()
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for (owner, dna) in self.kitties.iter() {
				let mut dnas = 	Pallet::<T>::gen_dna();
				let gender = Pallet::<T>::gen_gender(&dna).unwrap();
				let mut kitty = Kitty::<T> {
					dna : dnas.clone(),
					price: 0u32.into(),
					gender: gender,
					owner: owner.clone(),
		 			created_date : T::Time::now(),
				};
				KittiesAccountId::<T>::insert(kitty.owner.clone(), kitty);
			}


		}
	}
	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	//extrinsic
	#[pallet::call]
	impl<T:Config> Pallet<T> {

		#[pallet::weight(0) ]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8> ) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let owner = ensure_signed(origin)?;

			// log::info!("total balance:{:?}", T::Currency::total_balance(&owner));

			let gender = Self::gen_gender(&dna)?;

			let dna = Self::gen_dna();

			let kitty = Kitty::<T> { 
				dna: dna.clone(), 
				price: T::Currency::total_balance(&owner), 
				gender, 
				owner: owner.clone(), 
				created_date : T::Time::now(), 
			};

			// Check if the kitty does not already exist in our storage map
			ensure!(!Kitties::<T>::contains_key(&kitty.dna), Error::<T>::DuplicateKitty);

			//log kitty
			log::warn!("Kitty :{:?}", kitty);
			// Performs this operation first as it may fail
			let current_id = KittyId::<T>::get();
			let next_id = current_id.checked_add(1).ok_or(ArithmeticError::Overflow)?;
			let a = (KittiesOwned::<T>::get(&owner)).len() as u32; // lấy độ dài của Vec<dna>
			let b = T::MaxAddend::get() ; // lấy giá trị của biến MaxAddend

			//test MaxAddend
			ensure!(a < b, Error::<T>::ExceedLimit); // nếu dộ dài < b đk thực thi
			// Append kitty to KittiesOwned
			KittiesOwned::<T>::try_append(&owner, kitty.dna.clone()).map_err(|_| Error::<T>::NoKitty)?;

			// Write new kitty to storage
			Kitties::<T>::insert(kitty.dna.clone(), kitty);
			KittyId::<T>::put(next_id);

			// Deposit our "Created" event.
			Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone()});

			Ok(())
		}
		#[pallet::weight(0)]
		pub fn transfer(origin: OriginFor<T>, to: T::AccountId, dna: T::Hash) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let from = ensure_signed(origin)?;
			let mut kitty = Kitties::<T>::get(&dna).ok_or(Error::<T>::NoKitty)?;
			ensure!(kitty.owner == from, Error::<T>::NotOwner);
			ensure!(from != to, Error::<T>::TransferToSelf);

			let mut from_owned = KittiesOwned::<T>::get(&from);

			// Remove kitty from list of owned kitties.
			if let Some(ind) = from_owned.iter().position(|ids| *ids == dna) {
				from_owned.swap_remove(ind);
			} else {
				return Err(Error::<T>::NoKitty.into())
			}

			let mut to_owned = KittiesOwned::<T>::get(&to);
			to_owned.try_push(dna.clone()).map_err(|_| Error::<T>::ExceedLimit);
			kitty.owner = to.clone();

			// Write updates to storage
			Kitties::<T>::insert(&dna, kitty);
			KittiesOwned::<T>::insert(&to, to_owned);
			KittiesOwned::<T>::insert(&from, from_owned);

			Self::deposit_event(Event::Transferred { from, to, kitty: dna });

			Ok(())
		}


	}
}

//helper function
impl<T:Config> Pallet<T> {
	fn gen_gender(dna: &Vec<u8>) -> Result<Gender,Error<T>>{
		let mut res = Gender::Female;
		if dna.len() % 2 ==0 {
			res = Gender::Male;
		}
		Ok(res)
	}
	fn gen_dna() -> T::Hash {
		let (seed,_) = T::Randomness::random_seed();
		let block_number = <frame_system::Pallet<T>>::block_number();
		T::Hashing::hash_of(&(seed, block_number))
	}
	
}