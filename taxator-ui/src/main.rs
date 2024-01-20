use eframe::{App, CreationContext, Frame, NativeOptions, run_native};
use eframe::egui::{CentralPanel, Context, Grid, TextEdit, Vec2};

use taxator_calculator::money::{Currency, Money};

struct TaxHelperApp {
    incomes: Vec<Money>,
    tax_amount: Option<Money>,
    edited_amount: Money,
}

impl TaxHelperApp {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            incomes: vec![],
            tax_amount: None,
            edited_amount: Money::new(0., Currency::UAH),
        }
    }

    fn calculate_total(&mut self) -> Money {
        let total = self.incomes.iter()
            .fold(Money::new(0., Currency::UAH), |x, y| x.add(y).unwrap());
        total
    }
}

impl App for TaxHelperApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Tax Helper Application");

                Grid::new("incomes_grid").show(ui, |ui| {
                    for (idx, income) in self.incomes.iter().enumerate() {
                        ui.label(format!("{}", idx + 1));
                        ui.label(format!("{:.2}", income.amount()));
                        ui.end_row();
                    }
                    ui.label("Total");
                    let total = self.calculate_total();
                    ui.label(format!("{:.2}", total.amount()));
                    ui.end_row();

                });

                let mut amount_str = format!("{:.2}", self.edited_amount.amount());
                let amount_edit =
                    ui.add(TextEdit::singleline(&mut amount_str).hint_text("Tax amount"));
                if amount_edit.changed() {
                    if let Ok(new_income_amount) = amount_str.parse::<f32>() {
                        let new_income = Money::new(new_income_amount, Currency::UAH);
                        self.edited_amount = new_income;
                    }
                }

                if self.edited_amount.amount() > 0. {
                    let record_amount_button = ui.button("Record amount");
                    if record_amount_button.clicked() {
                        self.incomes.push(self.edited_amount);
                        self.edited_amount = Money::new(0., Currency::UAH);
                    }
                }

                if !self.incomes.is_empty() {
                    let calculate_button = ui.button("Calculate");
                    if calculate_button.clicked() {
                        let total = self.calculate_total();
                        let tax = total.percent(5.0);
                        self.tax_amount = Some(tax);
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
