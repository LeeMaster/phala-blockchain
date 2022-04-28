use super::Balance;
use frame_support::{
    dispatch::DispatchResult,
    traits::{self, BalanceStatus},
};

use call_trace::{trace_with, CallContext, Trace};
use sp_runtime::DispatchError;

const ENOUGH: Balance = Balance::MAX / 2;

struct Null;

impl Trace for Null {
    fn on_pre(&mut self, _ctx: &CallContext) {
        // println!("Enter {:?}", ctx);
    }

    fn on_post(&mut self, _ctx: &CallContext) {
        // println!("Exit {:?}", ctx);
    }
}

#[derive(Default)]
pub struct NoImbalance;

impl traits::TryDrop for NoImbalance {
    fn try_drop(self) -> Result<(), Self> {
        Ok(())
    }
}

impl traits::Imbalance<Balance> for NoImbalance {
    type Opposite = NoImbalance;

    #[trace_with(Null)]
    fn zero() -> Self {
        Self
    }

    #[trace_with(Null)]
    fn drop_zero(self) -> Result<(), Self> {
        Ok(())
    }

    #[trace_with(Null)]
    fn split(self, _amount: Balance) -> (Self, Self) {
        Default::default()
    }

    #[trace_with(Null)]
    fn merge(self, _other: Self) -> Self {
        self
    }

    #[trace_with(Null)]
    fn subsume(&mut self, _other: Self) {}

    #[trace_with(Null)]
    fn offset(self, _other: Self::Opposite) -> traits::SameOrOther<Self, Self::Opposite> {
        traits::SameOrOther::Same(self)
    }

    #[trace_with(Null)]
    fn peek(&self) -> Balance {
        0
    }
}

pub struct NoCurrency;

impl<AccountId> traits::Currency<AccountId> for NoCurrency {
    type Balance = Balance;

    type PositiveImbalance = NoImbalance;
    type NegativeImbalance = NoImbalance;

    #[trace_with(Null)]
    fn total_balance(_who: &AccountId) -> Self::Balance {
        ENOUGH
    }

    #[trace_with(Null)]
    fn can_slash(_who: &AccountId, _value: Self::Balance) -> bool {
        false
    }

    #[trace_with(Null)]
    fn total_issuance() -> Self::Balance {
        ENOUGH
    }

    #[trace_with(Null)]
    fn minimum_balance() -> Self::Balance {
        0
    }

    #[trace_with(Null)]
    fn burn(_amount: Self::Balance) -> Self::PositiveImbalance {
        traits::Imbalance::zero()
    }

    #[trace_with(Null)]
    fn issue(_amount: Self::Balance) -> Self::NegativeImbalance {
        traits::Imbalance::zero()
    }

    #[trace_with(Null)]
    fn free_balance(_who: &AccountId) -> Self::Balance {
        ENOUGH
    }

    #[trace_with(Null)]
    fn ensure_can_withdraw(
        _who: &AccountId,
        _amount: Self::Balance,
        _reasons: frame_support::traits::WithdrawReasons,
        _new_balance: Self::Balance,
    ) -> frame_support::dispatch::DispatchResult {
        Ok(())
    }

    #[trace_with(Null)]
    fn transfer(
        _source: &AccountId,
        _dest: &AccountId,
        _value: Self::Balance,
        _existence_requirement: frame_support::traits::ExistenceRequirement,
    ) -> frame_support::dispatch::DispatchResult {
        Ok(())
    }

    #[trace_with(Null)]
    fn slash(_who: &AccountId, _value: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
        (traits::Imbalance::zero(), 0)
    }

    #[trace_with(Null)]
    fn deposit_into_existing(
        _who: &AccountId,
        _value: Self::Balance,
    ) -> Result<Self::PositiveImbalance, sp_runtime::DispatchError> {
        Ok(traits::Imbalance::zero())
    }

    #[trace_with(Null)]
    fn deposit_creating(_who: &AccountId, _value: Self::Balance) -> Self::PositiveImbalance {
        traits::Imbalance::zero()
    }

    #[trace_with(Null)]
    fn withdraw(
        _who: &AccountId,
        _value: Self::Balance,
        _reasons: frame_support::traits::WithdrawReasons,
        _liveness: frame_support::traits::ExistenceRequirement,
    ) -> Result<Self::NegativeImbalance, sp_runtime::DispatchError> {
        Ok(traits::Imbalance::zero())
    }

    #[trace_with(Null)]
    fn make_free_balance_be(
        _who: &AccountId,
        _balance: Self::Balance,
    ) -> frame_support::traits::SignedImbalance<Self::Balance, Self::PositiveImbalance> {
        frame_support::traits::SignedImbalance::zero()
    }
}

impl<AccountId> traits::ReservableCurrency<AccountId> for NoCurrency {
    fn can_reserve(_: &AccountId, _: Self::Balance) -> bool {
        true
    }
    fn slash_reserved(_: &AccountId, _: Self::Balance) -> (Self::NegativeImbalance, Self::Balance) {
        (Default::default(), 0)
    }
    fn reserved_balance(_: &AccountId) -> Self::Balance {
        0
    }
    fn reserve(_: &AccountId, _: Self::Balance) -> DispatchResult {
        Ok(())
    }
    fn unreserve(_: &AccountId, _: Self::Balance) -> Self::Balance {
        0
    }
    fn repatriate_reserved(
        _: &AccountId,
        _: &AccountId,
        _: Self::Balance,
        _: BalanceStatus,
    ) -> Result<Self::Balance, DispatchError> {
        Ok(0)
    }
}

pub struct NoAccountStore;

impl<AccountId> traits::StoredMap<AccountId, pallet_balances::AccountData<Balance>>
    for NoAccountStore
{
    fn get(_: &AccountId) -> pallet_balances::AccountData<Balance> {
        pallet_balances::AccountData {
            free: ENOUGH,
            reserved: ENOUGH,
            misc_frozen: ENOUGH,
            fee_frozen: ENOUGH,
        }
    }

    fn try_mutate_exists<R, E: From<sp_runtime::DispatchError>>(
        _k: &AccountId,
        f: impl FnOnce(&mut Option<pallet_balances::AccountData<Balance>>) -> Result<R, E>,
    ) -> Result<R, E> {
        f(&mut None)
    }
}
