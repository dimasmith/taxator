//! Components to display and manipulate monetary amounts.

use eframe::egui::{Response, Ui, Widget};

use taxator_calculator::money::Money;

#[derive(Default)]
pub struct MoneyLabel {
    money: Money,
}

impl MoneyLabel {
    pub fn new(money: Money) -> Self {
        Self { money }
    }
}

impl Widget for MoneyLabel {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(format!("{}", self.money))
    }
}

