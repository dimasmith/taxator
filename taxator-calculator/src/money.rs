//! Simple representation of monetary amounts.

use thiserror::Error;

/// A monetary amount in a given currency.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Money {
    amount: f32,
    currency: Currency,
}

/// Currency representation.
/// As I only operate a few currency this enum is by no means complete.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum Currency {
    #[default]
    UAH,
    USD,
    EUR,
}

#[derive(Debug, Copy, Clone, PartialEq, Error)]
pub enum MoneyArithmeticsError {
    #[error("operation is not allowed for different currencies")]
    CurrencyMismatch(Currency, Currency),
}

impl Money {
    pub fn new(amount: f32, currency: Currency) -> Self {
        Self { amount, currency }
    }

    /// Creates a new amount which is a provided percent of a current amount.
    pub fn percent(&self, percent: f32) -> Self {
        let scaled_amount = self.amount / 100. * percent;
        Money::new(scaled_amount, self.currency)
    }

    /// Adds two monetary amounts of the same currency.
    /// Fails if currencies are different.
    pub fn add(&self, rhs: &Money) -> Result<Self, MoneyArithmeticsError> {
        self.assert_currency_match(rhs)?;
        let new_amount = self.amount + rhs.amount;
        Ok(Money::new(new_amount, self.currency))
    }

    /// Subtracts monetary amount from current amount.
    /// Amounts must have the same currencies
    /// Fails if currencies are different.
    pub fn sub(&self, rhs: &Money) -> Result<Self, MoneyArithmeticsError> {
        self.assert_currency_match(rhs)?;
        let new_amount = self.amount - rhs.amount;
        Ok(Money::new(new_amount, self.currency))
    }

    pub fn amount(&self) -> f32 {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Checks if money currencies match.
    fn assert_currency_match(&self, rhs: &Money) -> Result<(), MoneyArithmeticsError> {
        if self.currency != rhs.currency {
            return Err(MoneyArithmeticsError::CurrencyMismatch(
                self.currency,
                rhs.currency,
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_percent_of_amount() {
        let amount = Money::new(200.0, Currency::UAH);
        let percent = 12.5;

        let percent_amount = amount.percent(percent);

        assert_eq!(percent_amount, Money::new(25.0, Currency::UAH));
    }

    #[test]
    fn add_amounts() {
        let base_amount = Money::new(100., Currency::UAH);
        let added = Money::new(200., Currency::UAH);

        let sum = base_amount.add(&added).unwrap();

        assert_eq!(sum, Money::new(300., Currency::UAH));
    }

    #[test]
    fn cannot_add_amounts_of_different_currencies() {
        let base_amount = Money::new(100., Currency::USD);
        let added = Money::new(200., Currency::UAH);

        let sum = base_amount.add(&added);

        assert!(matches!(
            sum,
            Err(MoneyArithmeticsError::CurrencyMismatch(
                Currency::USD,
                Currency::UAH
            ))
        ));
    }

    #[test]
    fn sub_amounts() {
        let base_amount = Money::new(300., Currency::UAH);
        let subtracted = Money::new(200., Currency::UAH);

        let sum = base_amount.sub(&subtracted).unwrap();

        assert_eq!(sum, Money::new(100., Currency::UAH));
    }

    #[test]
    fn cannot_sub_amounts_of_different_currencies() {
        let base_amount = Money::new(100., Currency::EUR);
        let subtracted = Money::new(50., Currency::UAH);

        let sum = base_amount.add(&subtracted);

        assert!(matches!(
            sum,
            Err(MoneyArithmeticsError::CurrencyMismatch(
                Currency::EUR,
                Currency::UAH
            ))
        ));
    }
}
