use crate as pallet_chainlink;
use codec::MaxEncodedLen;
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64},
};
use frame_system as system;
use sp_core::{bounded::BoundedVec, ConstU32, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		Chainlink: pallet_chainlink,
		TestPallet: pallet
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
}

parameter_types! {
	pub const MaxResultLen : u32 = 16;
	pub const MaxRequestLen : u32 = 16;
	pub const ValidityPeriod: u32 = 10;
}

impl pallet_chainlink::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type Callback = pallet::Call<Test>;
	type ValidityPeriod = ValidityPeriod;
	type MaxRequestLen = MaxRequestLen;
	type MaxRequests = ConstU32<1>;
	type MaxResultLen = MaxResultLen;
	type MaxSpecIndexLen = ();
}

impl pallet::Config for Test {
	//type RuntimeEvent = RuntimeEvent;
	type Callback = pallet::Call<Test>;
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_core::bounded::BoundedVec;

	#[pallet::config]
	pub trait Config: frame_system::Config + crate::Config {
		type Callback: From<Call<Self>> + Into<<Self as crate::Config>::Callback>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(crate) type Result<T> = StorageValue<_, u128, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		DecodingFailed,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn callback(
			_origin: OriginFor<T>,
			result: BoundedVec<u8, T::MaxResultLen>,
		) -> DispatchResult {
			let r: u128 = u128::decode(&mut &result[..]).map_err(|_| Error::<T>::DecodingFailed)?;
			<Result<T>>::put(r);
			Ok(())
		}
	}
}

impl<T: pallet::Config> MaxEncodedLen for pallet::Call<T> {
	fn max_encoded_len() -> usize {
		todo!()
	}
}

impl<T: pallet::Config> pallet_chainlink::CallbackWithParameter<T::MaxResultLen>
	for pallet::Call<T>
{
	fn with_result(&self, result: BoundedVec<u8, T::MaxResultLen>) -> Option<Self>
	where
		Self: Sized,
	{
		match *self {
			pallet::Call::callback { .. } => Some(pallet::Call::callback { result }),
			_ => None,
		}
	}
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		// Total issuance will be 200 with treasury account initialized at ED.
		balances: vec![(0, 100000), (1, 100000), (2, 100000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}
