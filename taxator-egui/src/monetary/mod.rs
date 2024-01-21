//! Components to display and manipulate monetary amounts.

use eframe::egui::{Id, Response, TextEdit, Ui, Widget};

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

/// A specialised text editor for editing numbers.
#[derive(Debug)]
pub struct AmountEdit<'a> {
    number: &'a mut f32,
    id: Id,
    text: String,
}

impl<'a> AmountEdit<'a> {
    pub fn new(id: &str, num: &'a mut f32) -> Self {
        Self {
            number: num,
            id: Id::new(id),
            text: String::default(),
        }
    }

    fn load_text(&self, ctx: &Ui) -> String {
        ctx.data(|m| m.get_temp(self.id))
            .unwrap_or_else(|| format!("{:.2}", self.number))
    }

    fn store_text(&self, ctx: &Ui) {
        ctx.data_mut(|m| m.insert_temp(self.id, self.text.clone()));
    }
}

impl<'a> Widget for AmountEdit<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let mut amount_str = self.load_text(ui);
        let text_edit = TextEdit::singleline(&mut amount_str)
            .hint_text("Enter your number")
            .char_limit(10);

        let response = ui.add(text_edit);

        if response.changed() {
            let num = amount_str.parse().unwrap_or(*self.number);
            let num = f32::min(1000000000., num);
            let num = f32::max(0.01, num);
            *self.number = num;
            self.text = amount_str;
            self.store_text(ui);
        }

        response
    }
}
