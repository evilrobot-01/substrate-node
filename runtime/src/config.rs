use super::*;

pub const MILLICENTS: Balance = 1_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS; // assume this is worth about a cent.
pub const DOLLARS: Balance = 100 * CENTS;

parameter_types! {
    pub const BasicDeposit: Balance = 10 * DOLLARS;                        // 258 bytes on-chain
    pub const FieldDeposit: Balance = 250 * CENTS;                         // 66 bytes on-chain
    pub const SubAccountDeposit: Balance = 2 * DOLLARS;                    // 53 bytes on-chain
    pub const MaxSubAccounts: u32 = 100;
    pub const MaxAdditionalFields: u32 = 100;
    pub const MaxRegistrars: u32 = 20;
}

// Reference Kusama and Polkadot runtimes as dev-dependencies, for easy navigation of associated configuration
#[cfg(test)] type Kusama = kusama_runtime::Runtime;
#[cfg(test)] type Polkadot = polkadot_runtime::Runtime;

// Based on kitchensink-runtime at https://github.com/paritytech/substrate/blob/0ee03277c33b6334ddba7434e014fa637dcb6107/bin/node/runtime/src/lib.rs#L1311-L1324
impl pallet_identity::Config for Runtime {
    /// The overarching event type.
    type RuntimeEvent = RuntimeEvent;
    /// The currency trait.
    type Currency = Balances; // LS: Balances pallet dependency via ReservableCurrency trait
    /// The amount held on deposit for a registered identity
    type BasicDeposit = BasicDeposit;
    /// The amount held on deposit per additional field for a registered identity.
    type FieldDeposit = FieldDeposit;
    /// The amount held on deposit for a registered sub-account. This should account for the fact
    /// that one storage item's value will increase by the size of an account ID, and there will
    /// be another trie item whose value is the size of an account ID plus 32 bytes.
    type SubAccountDeposit = SubAccountDeposit;
    /// The maximum number of sub-accounts allowed per identified account.
    type MaxSubAccounts = MaxSubAccounts;
    /// Maximum number of additional fields that may be stored in an ID. Needed to bound the I/O
    /// required to access an identity, but can be pretty high.
    type MaxAdditionalFields = MaxAdditionalFields;
    /// Maximum number of registrars allowed in the system. Needed to bound the complexity of, e.g., updating judgements.
    type MaxRegistrars = MaxRegistrars;
    /// What to do with slashed funds.
    type Slashed = (); // LS: kitchensink-runtime, Kusama, Polkadot all use Treasury here
    /// The origin which may forcibly set or remove a name. Root can always do this.
    type ForceOrigin = EnsureRoot<AccountId>; // LS: kitchensink-runtime, Kusama, Polkadot all use EnsureRootOrHalfCouncil (pallet-collective) here
    /// The origin which may add or remove registrars. Root can always do this.
    type RegistrarOrigin = EnsureRoot<AccountId>; // LS: kitchensink-runtime, Kusama, Polkadot all use EnsureRootOrHalfCouncil (pallet-collective) here
    /// Weight information for extrinsics in this pallet.
    type WeightInfo = pallet_identity::weights::SubstrateWeight<Runtime>;
}

// Based on https://docs.substrate.io/tutorials/work-with-pallets/add-a-pallet/#implement-the-configuration-for-nicks
impl pallet_nicks::Config for Runtime {
    /// The overarching event type.
    type RuntimeEvent = RuntimeEvent;
    /// The currency trait.
    // The Balances pallet implements the ReservableCurrency trait. `Balances` is defined in `construct_runtime!` macro.
    type Currency = Balances; // LS: Balances pallet dependency via ReservableCurrency trait
    /// Reservation fee.
    type ReservationFee = ConstU128<100>;
    /// What to do with slashed funds.
    type Slashed = (); // No action is taken when deposits are forfeited.
    /// The origin which may forcibly set or remove a name. Root can always do this.
    type ForceOrigin = EnsureRoot<AccountId>; // Configure the FRAME System Root origin as the Nick pallet admin: https://paritytech.github.io/substrate/master/frame_system/enum.RawOrigin.html#variant.Root
    // LS: Bound name length (8-32) - on-chain storage expensive
    /// The minimum length a name may be.
    type MinLength = ConstU32<8>;
    /// The maximum length a name may be.
    type MaxLength = ConstU32<32>;
}