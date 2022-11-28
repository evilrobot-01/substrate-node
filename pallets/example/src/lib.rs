#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use codec::MaxEncodedLen;
	use frame_support::{
		pallet_prelude::*, sp_runtime::traits::UniqueSaturatedFrom, traits::Currency,
	};
	use frame_system::pallet_prelude::*;

	type BalanceOf<T> = <<T as pallet_chainlink::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_chainlink::Config {
		type Callback: From<Call<Self>> + Into<<Self as pallet_chainlink::Config>::Callback>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(crate) type Result<T> = StorageValue<_, i128, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		EncodingFailed,
		DecodingFailed,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn send_request(
			origin: OriginFor<T>,
			operator: T::AccountId,
			spec_id: BoundedVec<u8, T::MaxSpecIndexLen>,
		) -> DispatchResult {
			let parameters = (
				"get",
				"https://min-api.cryptocompare.com/data/pricemultifull?fsyms=ETH&tsyms=USD",
				"path",
				"RAW.ETH.USD.PRICE",
				"times",
				"100000000",
			);
			let call: <T as Config>::Callback =
				Call::callback { result: BoundedVec::<u8, T::MaxResultLen>::default() }.into();

			let fee = BalanceOf::<T>::unique_saturated_from(100u32);
			<pallet_chainlink::Pallet<T>>::initiate_request(
				origin,
				operator,
				spec_id,
				0,
				parameters.encode().try_into().map_err(|_| Error::<T>::EncodingFailed)?,
				fee,
				call.into(),
			)?;

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn callback(
			_origin: OriginFor<T>,
			result: BoundedVec<u8, T::MaxResultLen>,
		) -> DispatchResult {
			// The result is expected to be a SCALE encoded `i128`
			let r: i128 = i128::decode(&mut &result[..]).map_err(|_| Error::<T>::DecodingFailed)?;
			<Result<T>>::put(r);
			Ok(())
		}
	}

	impl<T: Config> MaxEncodedLen for Call<T> {
		fn max_encoded_len() -> usize {
			todo!()
		}
	}

	impl<T: Config> pallet_chainlink::CallbackWithParameter<T::MaxResultLen> for Call<T> {
		fn with_result(&self, result: BoundedVec<u8, T::MaxResultLen>) -> Option<Self>
		where
			Self: Sized,
		{
			match *self {
				Call::callback { .. } => Some(Call::callback { result }),
				_ => None,
			}
		}
	}
}
