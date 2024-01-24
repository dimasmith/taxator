//! Registry of incomes with income dates
use chrono::NaiveDate;
use std::cmp::Ordering;

use crate::money::Money;

/// Records containing income amounts and income dates.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LedgerRecord {
    amount: Money,
    date: NaiveDate,
}

/// A log of incomes by dates.
/// Ledger always keeps entries in the chronological order from oldest to newest.
/// However, the order of entries arriving in the same day are not guaranteed.
/// As the sorting happens only on modification, that's not a big issue.
#[derive(Debug, Default)]
pub struct Ledger {
    entries: Vec<LedgerRecord>,
}

impl LedgerRecord {
    pub fn new(amount: Money, date: NaiveDate) -> Self {
        Self { amount, date }
    }

    pub fn amount(&self) -> Money {
        self.amount
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    fn compare(&self, other: &LedgerRecord) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl Ledger {
    pub fn new(entries: &[LedgerRecord]) -> Self {
        let mut content = entries.to_vec();
        content.sort_by(LedgerRecord::compare);
        Self { entries: content }
    }

    /// Calculate a total of all records in the Ledger
    pub fn total(&self) -> Money {
        let amounts: Vec<Money> = self.entries.iter().map(LedgerRecord::amount).collect();
        Money::total(&amounts).unwrap()
    }

    /// Add a new record to the ledger
    pub fn add(&mut self, entry: LedgerRecord) {
        self.entries.push(entry);
        self.entries.sort_by(LedgerRecord::compare);
    }

    pub fn remove(&mut self, idx: usize) {
        self.entries.remove(idx);
    }

    /// A slice representing all records
    pub fn iter(&self) -> &[LedgerRecord] {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl PartialOrd for LedgerRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

#[cfg(test)]
mod tests {
    use crate::ledger::{Ledger, LedgerRecord};
    use crate::money::{Currency, Money};
    use chrono::NaiveDate;

    #[test]
    fn items_are_sorted_on_creation() {
        let day = LedgerRecord::new(
            Money::new(10., Currency::UAH),
            NaiveDate::from_ymd_opt(2024, 01, 23).unwrap(),
        );
        let dawn = LedgerRecord::new(
            Money::new(1., Currency::UAH),
            NaiveDate::from_ymd_opt(2024, 01, 24).unwrap(),
        );
        let dusk = LedgerRecord::new(
            Money::new(100., Currency::UAH),
            NaiveDate::from_ymd_opt(2024, 01, 22).unwrap(),
        );
        let ledger = Ledger::new(&[dusk, dawn, day]);

        let entries = ledger.iter();

        assert_eq!(entries, &[dusk, day, dawn]);
    }

    #[test]
    fn items_are_sorted_on_add() {
        let day = LedgerRecord::new(
            Money::new(10., Currency::UAH),
            NaiveDate::from_ymd_opt(2024, 01, 23).unwrap(),
        );
        let dawn = LedgerRecord::new(
            Money::new(1., Currency::UAH),
            NaiveDate::from_ymd_opt(2024, 01, 24).unwrap(),
        );
        let dusk = LedgerRecord::new(
            Money::new(100., Currency::UAH),
            NaiveDate::from_ymd_opt(2024, 01, 22).unwrap(),
        );
        let mut ledger = Ledger::new(&[dusk, dawn]);

        ledger.add(day);

        let entries = ledger.iter();
        assert_eq!(entries, &[dusk, day, dawn]);
    }
}
