#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, Blake2_128Concat};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The maxximum length of claim that can be added.
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
		/// ClaimShifted(from, to, calim)
		ClaimShifted(T::AccountId, T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResult {
			let creater = ensure_signed(origin)?;

			// Check range
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			// Check existence
			ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(
				&bounded_claim,
				(creater.clone(), frame_system::Pallet::<T>::block_number()),
			);

			Self::deposit_event(Event::ClaimCreated(creater, claim));
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			let Ok((owner, _)) = Proofs::<T>::try_get(&bounded_claim) else {
					return Err(Error::<T>::ClaimNotExist.into());
            };
			// 如果要避免透露凭证存在（但却是他人拥有）的信息，
			// 最好把上下语句的错误信息统一为 NotHoldClaim 。
			// 否则攻击者可通过错误信息推断哪些凭证是已经存在的，哪怕这些凭证并不属于他。
			ensure!(sender == owner, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&bounded_claim);

			Self::deposit_event(Event::ClaimRevoked(sender, claim));
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn shift_claim(
			origin: OriginFor<T>,
			to: T::AccountId,
			claim: Vec<u8>,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
				.map_err(|_| Error::<T>::ClaimTooLong)?;

			let Ok((owner, _)) = Proofs::<T>::try_get(&bounded_claim) else {
				return Err(Error::<T>::ClaimNotExist.into());
            };
			ensure!(from == owner, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&bounded_claim);

			Proofs::<T>::insert(
				&bounded_claim,
				(to.clone(), frame_system::Pallet::<T>::block_number()),
			);

			Self::deposit_event(Event::ClaimShifted(from, to, claim));
			Ok(())
		}
	}
}
