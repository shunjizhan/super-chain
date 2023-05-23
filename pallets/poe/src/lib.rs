#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;		// when running in test env, use mock.rs module

#[cfg(test)]
mod tests;	// when running in test env, use test.rs module

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
	}

	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,	// key type
		(T::AccountId, T::BlockNumber),			// value type	
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
		ClaimTransferred(T::AccountId, T::AccountId, BoundedVec<u8, T::MaxClaimLength>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// create a claim
		#[pallet::weight(0)]
		pub fn create_claim(
			origin: OriginFor<T>,
			claim: BoundedVec<u8, T::MaxClaimLength>
		) -> DispatchResultWithPostInfo {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let sender = ensure_signed(origin)?;

			// Update storage.
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(
				&bounded_claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);

			// Emit an event.
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		/// revoke a claim
		#[pallet::weight(0)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			claim: BoundedVec<u8, T::MaxClaimLength>
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			// Update storage.
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&bounded_claim);

			// Emit an event.
			Self::deposit_event(Event::ClaimRevoked(sender, claim));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		/// transfer a claim
		#[pallet::weight(0)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			to: T::AccountId,
			claim: BoundedVec<u8, T::MaxClaimLength>
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			// Update storage.
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;
			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&bounded_claim);
			Proofs::<T>::insert(
				&bounded_claim,
				(to.clone(), frame_system::Pallet::<T>::block_number()),
			);

			// Emit an event.
			Self::deposit_event(Event::ClaimTransferred(sender, to, claim));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}
	}
}
