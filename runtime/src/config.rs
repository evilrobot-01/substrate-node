use self::additional_config::AssetsToBlockAuthor;
use super::*;
use sp_runtime::traits::ConvertInto;

mod dotsama {
	// Reference Kusama/Polkadot runtimes as dev-dependencies, for easy navigation to configuration
	#[cfg(test)]
	type Kusama = kusama_runtime::Runtime;
	#[cfg(test)]
	type Polkadot = polkadot_runtime::Runtime;
	#[cfg(test)]
	type DealWithFees = polkadot_runtime_common::impls::DealWithFees<Polkadot>;
	#[cfg(test)]
	type WeightToFee = polkadot_runtime_constants::WeightToFee;
	#[cfg(test)]
	type ConstantMultiplier =
		sp_weights::ConstantMultiplier<super::Balances, polkadot_runtime::TransactionByteFee>;
	#[cfg(test)]
	type SlowAdjustingFeeUpdate = polkadot_runtime_common::SlowAdjustingFeeUpdate<Polkadot>;
}

impl pallet_transaction_payment::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Handler for withdrawing, refunding and depositing the transaction fee.
	// LS: Handler for withdrawal (pre-dispatch), correct and deposit (post-dispatch) of tx fees
	// LS: No handler for OnUnbalanced so fees burned, DOT/KSM use DealWithFees<Runtime> 80/20 split
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	/// A fee multiplier for `Operational` extrinsics to compute "virtual tip" to boost `priority`
	// LS: multiplier to prioritise operational extrinsics
	// LS: virtual_tip = final_fee * operational_fee_multiplier
	type OperationalFeeMultiplier = ConstU8<5>;
	/// Convert a weight value into a deductible fee based on the currency type.
	// LS: IdentifyFee -> one unit of weight = one unit of fee, DOT/KSM use WeightToFee
	type WeightToFee = IdentityFee<Balance>;
	/// Convert a length value into a deductible fee based on the currency type.
	// LS: tx length, DOT/KSM use ConstantMultiplier<Balance, TransactionByteFee>
	type LengthToFee = IdentityFee<Balance>;
	/// Update the multiplier of the next block, based on the previous block's weight.
	// LS: Block fee multiplier, DOT/KSM use SlowAdjustingFeeUpdate<Self>
	type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
}

// LS: implemented within Statemint/e
impl pallet_asset_tx_payment::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// The fungibles instance used to pay for transactions in assets.
	// LS: Assets pallet via fungibles::Balanced<AccountId> trait
	type Fungibles = Assets;
	/// The actual transaction charging logic that charges the fees.
	// LS: Fungibles adapter for balance to asset conversion and credit handler
	type OnChargeAssetTransaction = pallet_asset_tx_payment::FungiblesAdapter<
		// LS: convert native balance to asset balance:
		// LS: NB: ratio of native min balance (existential deposit) vs asset min balance
		pallet_assets::BalanceToAssetBalance<Balances, Runtime, ConvertInto>,
		// LS: transaction fees to block author
		AssetsToBlockAuthor<Runtime>,
	>;
}

#[cfg(test)]
mod code_walkthrough {
	use super::{BalancesCall, Runtime, RuntimeCall, RuntimeOrigin};
	use codec::Encode;
	use frame_support::traits::Hooks;
	#[allow(unused_imports)]
	use pallet_transaction_payment_rpc_runtime_api::{
		TransactionPaymentApi, TransactionPaymentCallApi,
	};
	use sp_core::crypto::AccountId32;
	use sp_runtime::{
		testing::TestXt,
		traits::{Dispatchable, SignedExtension},
		MultiAddress,
	};

	type XT = TestXt<RuntimeCall, ()>;

	const ASSET: u32 = 0;
	const BLOCK: u32 = 0;
	const BENEFICIARY: AccountId32 = AccountId32::new([0u8; 32]);
	const TRANSFER: <Runtime as frame_system::Config>::RuntimeCall =
		RuntimeCall::Balances(BalancesCall::transfer {
			dest: MultiAddress::Id(BENEFICIARY),
			value: TRANSFER_AMOUNT,
		});
	const TRANSFER_AMOUNT: u128 = 1_000_000_000_000;
	const TRANSACTOR: AccountId32 = AccountId32::new([0u8; 32]);
	const TIP: u128 = 100_000_000_000;

	#[cfg(test)]
	fn overview() {
		let len: usize = TRANSFER.encode().len();

		// LS: Transaction Payment pallet: no extrinsics, just RPC API, SignedExtension, hook
		type TxPayment = pallet_transaction_payment::Pallet<Runtime>;
		type TxPaymentEvent = pallet_transaction_payment::pallet::Event<Runtime>;

		// LS: RPC API
		let extrinsic = XT::new(TRANSFER, None);
		// LS: query *predicted* weight, class, inclusion fee (based on extrinsic weight attribute)
		let dispatch_info = TxPayment::query_info(extrinsic.clone(), len as u32).into();
		let _fee_details = TxPayment::query_fee_details(extrinsic, len as u32);
		// LS: *call* variants always include all fees, above only if extrinsic signed
		let _ = TxPayment::query_call_info(TRANSFER, len as u32);
		let _ = TxPayment::query_call_fee_details(TRANSFER, len as u32);

		// LS: Signed Extension - ChargeTransactionPayment
		type ChargeTxPayment = pallet_transaction_payment::ChargeTransactionPayment<Runtime>;
		let se = ChargeTxPayment::from(TIP);
		se.validate(&TRANSACTOR, &TRANSFER, &dispatch_info, len).unwrap(); // LS: used by transaction queue to *quickly* validate
		let pre = se.pre_dispatch(&TRANSACTOR, &TRANSFER, &dispatch_info, len).ok(); // LS: withdraw fees
		let post_info = TRANSFER.dispatch(RuntimeOrigin::signed(TRANSACTOR)).unwrap(); // LS: dispatch call to determine actual fees
		ChargeTxPayment::post_dispatch(pre, &dispatch_info, &post_info, len, &Ok(())).unwrap();

		// LS: Hooks
		TxPayment::on_finalize(BLOCK); // LS: updates nextFeeMultiplier storage item for next block

		// LS: runtime config of signed extensions
		type SignedExtensions = super::SignedExtra;

		// LS: Asset Transaction Payment: no extrinsics, just SignedExtension
		type AssetTxPayment = pallet_asset_tx_payment::Pallet<Runtime>;
		type AssetTxPaymentEvent = pallet_asset_tx_payment::pallet::Event<Runtime>;

		// LS: Signed Extension - ChargeAssetTxPayment
		type ChargeAssetTxPayment = pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>;
		let se = ChargeAssetTxPayment::from(TIP, Some(ASSET)); // LS: AssetID included
		se.validate(&TRANSACTOR, &TRANSFER, &dispatch_info, len).unwrap();
		let pre = se.pre_dispatch(&TRANSACTOR, &TRANSFER, &dispatch_info, len).ok();
		let post_info = TRANSFER.dispatch(RuntimeOrigin::signed(TRANSACTOR)).unwrap();
		ChargeAssetTxPayment::post_dispatch(pre, &dispatch_info, &post_info, len, &Ok(())).unwrap();
	}
}

mod additional_config {
	use super::{AccountId, Balance, Balances, Runtime, RuntimeEvent, EXISTENTIAL_DEPOSIT};
	use frame_support::{
		log::{log, Level},
		parameter_types,
		traits::fungibles::{Balanced, CreditOf},
	};
	use frame_system::EnsureRoot;
	use pallet_asset_tx_payment::HandleCredit;
	use sp_core::crypto::AccountId32;
	use sp_std::marker::PhantomData;

	pub const UNITS: Balance = 1_000_000_000_000;
	pub const CENTS: Balance = UNITS / 100;
	pub const MILLICENTS: Balance = CENTS / 1_000;

	parameter_types! {
		pub const AssetDeposit: Balance = UNITS / 10; // 1 / 10 WND deposit to create asset
		pub const AssetAccountDeposit: Balance = deposit(1, 16);
		pub const ApprovalDeposit: Balance = EXISTENTIAL_DEPOSIT;
		pub const AssetsStringLimit: u32 = 50;
		pub const MetadataDepositBase: Balance = deposit(1, 68);
		pub const MetadataDepositPerByte: Balance = deposit(0, 1);
	}

	impl pallet_assets::Config for Runtime {
		type RuntimeEvent = RuntimeEvent;
		type Balance = Balance;
		type AssetId = u32;
		type Currency = Balances;
		type ForceOrigin = EnsureRoot<AccountId>;
		type AssetDeposit = AssetDeposit;
		type AssetAccountDeposit = AssetAccountDeposit;
		type MetadataDepositBase = MetadataDepositBase;
		type MetadataDepositPerByte = MetadataDepositPerByte;
		type ApprovalDeposit = ApprovalDeposit;
		type StringLimit = AssetsStringLimit;
		type Freezer = ();
		type Extra = ();
		type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
	}

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 100 * CENTS + (bytes as Balance) * 5 * MILLICENTS
	}

	pub type AccountIdOf<R> = <R as frame_system::Config>::AccountId;

	/// A `HandleCredit` implementation that naively transfers the fees to the block author.
	/// Will drop and burn the assets in case the transfer fails.
	pub struct AssetsToBlockAuthor<R>(PhantomData<R>);
	impl<R> HandleCredit<AccountIdOf<R>, pallet_assets::Pallet<R>> for AssetsToBlockAuthor<R>
	where
		R: pallet_assets::Config,
		AccountIdOf<R>: From<AccountId> + Into<AccountId>,
	{
		fn handle_credit(credit: CreditOf<AccountIdOf<R>, pallet_assets::Pallet<R>>) {
			log!(Level::Info, "Assets to block author called");

			// In case of error: Will drop the result triggering the `OnDrop` of the imbalance.
			let ferdie = [
				28, 189, 45, 67, 83, 10, 68, 112, 90, 208, 136, 175, 49, 62, 24, 248, 11, 83, 239,
				22, 179, 97, 119, 205, 75, 119, 184, 70, 242, 165, 240, 124,
			];
			let author = AccountIdOf::<R>::from(AccountId32::try_from(ferdie.as_slice()).unwrap());
			let _ = pallet_assets::Pallet::<R>::resolve(&author, credit);
		}
	}
}
