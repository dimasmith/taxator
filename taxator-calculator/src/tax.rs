//! Calculates taxes over money amounts

use crate::money::{Currency, Money};
use thiserror::Error;

const TAX_PERCENT: f32 = 5.;

#[derive(Debug)]
pub struct TaxCalculator {
    percent: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, Error)]
pub enum CalculationError {
    #[error("tax calculation does not support mixed currencies yet")]
    MixedCurrencies,
}

impl TaxCalculator {
    /// Creates calculator for specified tax percent
    pub fn new(percent: f32) -> Self {
        Self { percent }
    }

    pub fn for_income(&self, income: &Money) -> Money {
        income.percent(self.percent)
    }

    pub fn for_incomes(&self, incomes: &[Money]) -> Result<Money, CalculationError> {
        if incomes.is_empty() {
            return Ok(Money::new(0., Currency::default()));
        }

        match Money::total(incomes) {
            Ok(t) => Ok(t.percent(self.percent)),
            Err(_) => Err(CalculationError::MixedCurrencies)
        }
    }
}

impl Default for TaxCalculator {
    fn default() -> Self {
        TaxCalculator::new(TAX_PERCENT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::money::Currency;

    #[test]
    fn tax_is_zero_in_default_currency_for_empty_incomes() {
        let calculator = TaxCalculator::default();
        let incomes = vec![];

        let tax = calculator.for_incomes(&incomes).unwrap();

        assert_eq!(tax, Money::new(0., Currency::default()));
    }

    #[test]
    fn tax_is_percent_of_total_incomes() {
        let calculator = TaxCalculator::default();
        let incomes = vec![
            Money::new(200., Currency::UAH),
            Money::new(400., Currency::UAH),
        ];

        let tax = calculator.for_incomes(&incomes).unwrap();

        assert_eq!(tax, Money::new(30., Currency::default()));
    }

    #[test]
    fn reject_mixed_incomes_currencies() {
        let calculator = TaxCalculator::default();
        let incomes = vec![
            Money::new(200., Currency::UAH),
            Money::new(400., Currency::USD),
        ];

        let result = calculator.for_incomes(&incomes);

        assert!(matches!(result, Err(CalculationError::MixedCurrencies)));
    }
}
