use eframe::egui::{CentralPanel, Context, TextEdit, Vec2};
use eframe::{run_native, App, CreationContext, Frame, NativeOptions};

use taxator_calculator::money::{Currency, Money};

struct TaxHelperApp {
    income: Option<Money>,
    tax_amount: Option<Money>,
}

impl TaxHelperApp {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            income: None,
            tax_amount: None,
        }
    }
}

impl App for TaxHelperApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Tax Helper Application");

                let mut amount_str = match self.income {
                    Some(income) => format!("{:.2}", income.amount()),
                    None => format!("{:.2}", 0.),
                };
                let amount_edit =
                    ui.add(TextEdit::singleline(&mut amount_str).hint_text("Tax amount"));
                if amount_edit.changed() {
                    if let Ok(new_income_amount) = amount_str.parse::<f32>() {
                        let new_income = Money::new(new_income_amount, Currency::UAH);
                        self.income.replace(new_income);
                    }
                }

                let calculate_button = ui.button("Calculate");
                if let Some(income) = self.income {
                    if calculate_button.clicked() {
                        let tax = income.percent(5.0);
                        self.tax_amount.replace(tax);
                    }
                }

                if let Some(tax) = self.tax_amount {
                    ui.label(format!("Tax amount: {:.2}", tax.amount()));
                }
            });
        });
    }
}

fn main() {
    let mut win_options = NativeOptions::default();
    win_options
        .viewport
        .inner_size
        .replace(Vec2::new(360., 240.));

    run_native(
        "Tax Helper",
        win_options,
        Box::new(|cc| Box::new(TaxHelperApp::new(cc))),
    )
    .unwrap();
}
