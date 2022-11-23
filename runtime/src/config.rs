use super::*;
use crate::config::additional_config::AssetsToBlockAuthor;
use sp_runtime::traits::ConvertInto;

impl pallet_transaction_payment::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Handler for withdrawing, refunding and depositing the transaction fee.
	// LS: transaction fees withdrawn before execution, weight adjusted after transaction execution
	// LS: transaction fees then deposited/refunded as necessary
	// LS: balances pallet for fee payment using native token, no handler for 'OnUnbalanced
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	/// A fee multiplier for `Operational` extrinsics to compute "virtual tip" to boost `priority`
	///
	/// This value is multiplied by the `final_fee` to obtain a "virtual tip" that is later
	/// added to a tip component in regular `priority` calculations.
	/// It means that a `Normal` transaction can front-run a similarly-sized `Operational`
	/// extrinsic (with no tip), by including a tip value greater than the virtual tip.
	///
	/// ```rust,ignore
	/// // For `Normal`
	/// let priority = priority_calc(tip);
	///
	/// // For `Operational`
	/// let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;
	/// let priority = priority_calc(tip + virtual_tip);
	/// ```
	///
	/// Note that since we use `final_fee` the multiplier applies also to the regular `tip`
	/// sent with the transaction. So, not only does the transaction get a priority bump based
	/// on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`
	type OperationalFeeMultiplier = ConstU8<5>;
	/// Convert a weight value into a deductible fee based on the currency type.
	type WeightToFee = IdentityFee<Balance>; // LS: one unit of weight = one unit of fee
	/// Convert a length value into a deductible fee based on the currency type.
	type LengthToFee = IdentityFee<Balance>;
	/// Update the multiplier of the next block, based on the previous block's weight.
	type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
}

impl pallet_asset_tx_payment::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// The fungibles instance used to pay for transactions in assets.
	type Fungibles = Assets; // LS: Assets pallet via fungibles::Balanced<AccountId> trait
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
	use super::{Balance, Block, Runtime, RuntimeCall};
	use frame_support::traits::Hooks;

	#[cfg(test)]
	fn overview() {
		// LS: Pallets
		type TxPayment = pallet_transaction_payment::Pallet<Runtime>;
		type AssetTxPayment = pallet_asset_tx_payment::Pallet<Runtime>;

		// LS: No extrinsics, just SignedExtensions and on_finalize hook
		type ChargeTransactionPayment =
			pallet_transaction_payment::ChargeTransactionPayment<Runtime>;
		type ChargeAssetTxPayment = pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>;
		type SignedExtensions = super::SignedExtra; // LS: runtime config
		TxPayment::on_finalize(); // LS: updates nextFeeMultiplier storage item for next block

		// Runtime APIs
		type TransactionPaymentApi =
			dyn pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>;
		type TransactionPaymentCallApi =
			dyn pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<
				Block,
				Balance,
				RuntimeCall,
			>;
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
