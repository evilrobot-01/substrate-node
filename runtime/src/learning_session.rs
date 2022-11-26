use super::*;
use codec::Encode;
use frame_support::traits::Hooks;
use polkadot_runtime_common::impls::DealWithFees;
use sp_core::crypto::AccountId32;
use sp_runtime::{
	testing::TestXt,
	traits::{ConvertInto, Dispatchable},
	MultiAddress,
};

mod dotsama {
	// Reference Kusama/Polkadot runtimes as dev-dependencies, for easy navigation to configuration
	type Kusama = kusama_runtime::Runtime;
	type Polkadot = polkadot_runtime::Runtime;
	type DealWithFees = polkadot_runtime_common::impls::DealWithFees<Polkadot>;
	type WeightToFee = polkadot_runtime_constants::WeightToFee;
	type ConstantMultiplier =
		sp_weights::ConstantMultiplier<super::Balances, polkadot_runtime::TransactionByteFee>;
	type SlowAdjustingFeeUpdate = polkadot_runtime_common::SlowAdjustingFeeUpdate<Polkadot>;
}

// LS: Traits
use sp_runtime::traits::SignedExtension;
use pallet_transaction_payment::OnChargeTransaction;
use pallet_asset_tx_payment::OnChargeAssetTransaction;

impl pallet_transaction_payment::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Handler for withdrawing, refunding and depositing the transaction fee.
	// LS: OnChargeTransaction handler used by SignedExtension to process transaction fees
	// LS: No handler for LiquidityInfo = fees burned, DOT/KSM use DealWithFees with 80/20 split
	type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees<Runtime>>;
	/// A fee multiplier for `Operational` extrinsics to compute "virtual tip" to boost `priority`
	// LS: prioritise operational extrinsics (virtual_tip = final_fee * operational_fee_multiplier)
	type OperationalFeeMultiplier = ConstU8<5>;
	/// Convert a weight value into a deductible fee based on the currency type.
	// LS: IdentifyFee -> 1 unit of weight:1 unit of fee
	// LS: DOT/KSM use WeightToFee struct, where extrinsic base weight mapped to 1/10 CENT
	type WeightToFee = IdentityFee<Balance>;
	/// Convert a length value into a deductible fee based on the currency type.
	// LS: Extrinsic length
	// LS: DOT/KSM use ConstantMultiplier<Balance, TransactionByteFee> (10 * MILLICENTS)
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
	// LS: Assets pallet via fungibles::Balanced trait
	type Fungibles = Assets;
	/// The actual transaction charging logic that charges the fees.
	// LS: Fungibles adapter for balance to asset conversion and credit handler
	type OnChargeAssetTransaction = pallet_asset_tx_payment::FungiblesAdapter<
		// LS: Converter: native balance to asset balance
		// LS: NB: ratio of native token min balance (existential deposit) vs asset min balance
		pallet_assets::BalanceToAssetBalance<Balances, Runtime, ConvertInto>,
		// LS: Handle credit: transaction fees to block author
		AssetsToBlockAuthor<Runtime>,
	>;
}

#[cfg(test)]
fn overview() {
	let len: usize = TRANSFER.encode().len();

	// LS: Transaction Payment pallet: no extrinsics, just RPC API, SignedExtension, hook
	type TxPayment = pallet_transaction_payment::Pallet<Runtime>;

	// LS: RPC API
	use pallet_transaction_payment_rpc_runtime_api::{
		TransactionPaymentApi, TransactionPaymentCallApi,
	};

	let extrinsic = XT::new(TRANSFER, None);
	// LS: query *predicted* weight, class, inclusion fee (based on extrinsic weight attribute)
	let dispatch_info = TxPayment::query_info(extrinsic.clone(), len as u32).into();
	let _fee_details = TxPayment::query_fee_details(extrinsic, len as u32);
	// LS: *call* variants always include fees, above only if extrinsic signed
	let _ = TxPayment::query_call_info(TRANSFER, len as u32);
	let _ = TxPayment::query_call_fee_details(TRANSFER, len as u32);

	// LS: ChargeTransactionPayment Signed Extension
	type ChargeTxPayment = pallet_transaction_payment::ChargeTransactionPayment<Runtime>;
	let se = ChargeTxPayment::from(TIP);
	se.validate(&TRANSACTOR, &TRANSFER, &dispatch_info, len).unwrap(); // LS: used by transaction queue to *quickly* validate
	let pre = se.pre_dispatch(&TRANSACTOR, &TRANSFER, &dispatch_info, len).ok(); // LS: withdraw fees
	let post_info = TRANSFER.dispatch(RuntimeOrigin::signed(TRANSACTOR)).unwrap(); // LS: dispatch call to determine actual fees
	ChargeTxPayment::post_dispatch(pre, &dispatch_info, &post_info, len, &Ok(())).unwrap();

	// LS: Hooks
	TxPayment::on_finalize(BLOCK); // LS: updates nextFeeMultiplier storage item for next block

	// LS: Asset Transaction Payment: no extrinsics, just SignedExtension
	type AssetTxPayment = pallet_asset_tx_payment::Pallet<Runtime>;

	// LS: runtime config of signed extensions
	type SignedExtensions = SignedExtra;

	// LS: ChargeAssetTxPayment Signed Extension
	type ChargeAssetTxPayment = pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>;
	let se = ChargeAssetTxPayment::from(TIP, Some(ASSET)); // LS: AssetID included
	let pre = se.pre_dispatch(&TRANSACTOR, &TRANSFER, &dispatch_info, len).ok();
	let post_info = TRANSFER.dispatch(RuntimeOrigin::signed(TRANSACTOR)).unwrap();
	ChargeAssetTxPayment::post_dispatch(pre, &dispatch_info, &post_info, len, &Ok(())).unwrap();
}

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
