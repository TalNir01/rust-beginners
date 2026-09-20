use rust_decimal::Decimal;
pub(crate) struct Account {
    balance: Decimal,
}
pub(crate) trait Deposit {
    fn as_decimal(&self) -> Decimal;
}

impl Deposit for u32 {
    fn as_decimal(&self) -> Decimal {
        Decimal::from(*self)
    }
}

impl Account {
    pub(crate) fn new() -> Self {
        Self {
            balance: Decimal::ZERO,
        }
    }

    pub(crate) fn deposit(&mut self, amount: impl Deposit) {
        self.balance += amount.as_decimal()
    }

    pub(crate) fn pay(&mut self, amount: Decimal) -> bool {
        if amount > self.balance {
            false
        } else {
            self.balance -= amount;
            true
        }
    }
}
