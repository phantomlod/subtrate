#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::inherent::Vec;


#[frame_support::pallet]
pub mod pallet {
	pub use super::*;

	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T:Config> {
		dna: Vec<u8>,
		price:u32,
		gender: Gender,
		owner: T::AccountId,
	}
	pub type Id = u32;

	#[derive(TypeInfo, Encode ,Decode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender{
		fn default()-> Self{
			Gender::Male
		}
	}
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn kitties_id)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type KittiesId<T> = StorageValue<_, Id,ValueQuery>;


	// key : id
	//value : kitties
	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, Id, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn owner_kitty_list)]
	pub(super) type OwnerKitties<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittiesStored(Vec<u8>,T::AccountId),
		NumberKitties(u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		// TooYoung,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		NotOwner,
		Error,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	//extrinsic
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_student(origin: OriginFor<T>,dna: Vec<u8>, price: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;
			// ensure!(price>20, Error::<T>::TooYoung);
			let mut gender = Gender::Male;
			if dna.len() % 2 == 0 {
				gender = Gender::Female;
			}
			let kitties = Kitty {
				dna: dna.clone(),
				price: price,
				gender: gender,
				owner: who.clone(),
			};
			// let current_id = Self::kitties_id();
			// let current_id = KittiesId::<T>::get();
			let mut current_id = <KittiesId<T>>::get();

			// Kitties::<T>::insert(current_id, kitties);
			<Kitties<T>>::insert(current_id, kitties);
			current_id +=1;
			KittiesId::<T>::put(current_id);
			// Emit an event.
			Self::deposit_event(Event::KittiesStored(dna,who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		//swap owner
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn swap_owner(origin: OriginFor<T>,id : Id, new_owner : T::AccountId) -> DispatchResult {
			
			let _who = ensure_signed(origin)?;

			let mut kitty  = <Kitties<T>>::get(id.clone()) ;
			match kitty {
				None => Err(Error::<T>::NotOwner.into()),
				Some(mut kitty_x) => {
					kitty_x.owner = new_owner;
					//update storage
					<Kitties<T>>::insert(id,kitty_x);
					Ok(())
				}
			}
			
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn count_kitty(origin: OriginFor<T>) -> DispatchResult {

			let mut count = 0 ;
			let mut current_id = <KittiesId<T>>::get();
			let mut kitty = <Kitties<T>>::get(current_id.clone());
			
			match kitty {				
				None => Err(Error::<T>::Error.into()),
				Some(kitty) => {
					let _who = ensure_signed(origin)?;
					let mut count = 0 ;
					let mut current_id = <KittiesId<T>>::get();
					if kitty.owner == _who {
						count +=1 ;
					}
					current_id +=1 ;
					KittiesId::<T>::put(current_id);
					//emit event
					Self::deposit_event(Event::NumberKitties(count));
					Ok(())
				}
			}

			
		}


	}
}


// helper function

// impl<T> Pallet<T> {
// 	fn gen_gender(name: Vec<u8>) -> Result<Gender,Error<T>>{
// 		let mut res = Gender::Male;
// 		if name.len() % 2 ==0 {
// 			res = Gender::Female;
// 		}
// 		Ok(res)
// 	}
// }
