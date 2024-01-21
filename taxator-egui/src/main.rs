use eframe::{App, CreationContext, Frame, NativeOptions, run_native};
use eframe::egui::{Button, CentralPanel, Context, ScrollArea, TextEdit, TopBottomPanel, Vec2};
use eframe::egui::panel::TopBottomSide;

use taxator_calculator::money::{Currency, Money};
use taxator_calculator::tax::TaxCalculator;
use taxator_egui::ledger::LedgerView;
use taxator_egui::monetary::AmountEdit;

struct TaxHelperApp {
    incomes: Vec<Money>,
    tax_amount: Option<Money>,
    edited_amount: Money,
    calculator: TaxCalculator,
}

impl TaxHelperApp {
    #[allow(dead_code)]
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            incomes: vec![],
            tax_amount: None,
            edited_amount: Money::new(0., Currency::UAH),
            calculator: TaxCalculator::default(),
        }
    }

    pub fn test(_cc: &CreationContext) -> Self {
        Self {
            incomes: vec![
                Money::new(100., Currency::UAH),
                Money::new(200., Currency::UAH),
                Money::new(300., Currency::UAH),
            ],
            tax_amount: None,
            edited_amount: Money::new(0., Currency::UAH),
            calculator: TaxCalculator::default(),
        }
    }

    fn remove_income(&mut self, idx: usize) {
        self.incomes.remove(idx);
    }
}

impl App for TaxHelperApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::new(TopBottomSide::Top, "experiment").show(ctx, |ui| {

            let mut amount = self.edited_amount.amount();
            let amount_edit = AmountEdit::new("amount_editor", &mut amount);
            ui.add(amount_edit);
            self.edited_amount = Money::new(amount, Currency::UAH);
            self.edited_amount = Money::new(amount, Currency::UAH);

        });
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Tax Helper Application");

                    let mut deleted_idx: Option<usize> = None;
                    LedgerView::new(self.incomes.as_slice())
                        .with_delete(&mut deleted_idx)
                        .with_totals()
                        .show_ui(ui);
                    if let Some(idx) = deleted_idx {
                        self.remove_income(idx);
                    }

                    ui.horizontal(|ui| {
                        ui.label("Income");
                        let mut amount_str = format!("{:.2}", self.edited_amount.amount());
                        let edit = TextEdit::singleline(&mut amount_str)
                            .hint_text("Tax amount")
                            .cursor_at_end(false);
                        let amount_edit = ui.add(edit);
                        if amount_edit.changed() {
                            if let Ok(new_income_amount) = amount_str.parse::<f32>() {
                                let new_income = Money::new(new_income_amount, Currency::UAH);
                                self.edited_amount = new_income;
                            }
                        }

                        let record_amount_button = Button::new("Record amount");
                        let record_amount_button =
                            ui.add_enabled(self.edited_amount.is_positive(), record_amount_button);
                        if record_amount_button.clicked() {
                            self.incomes.push(self.edited_amount);
                            self.edited_amount = Money::default();
                        }
                        ui.end_row();
                    });

                    if !self.incomes.is_empty() {
                        let calculate_button = ui.button("Calculate");
                        if calculate_button.clicked() {
                            let tax = self.calculator.for_incomes(&self.incomes).unwrap();
                            self.tax_amount = Some(tax);
                        }
                    }

                    if let Some(tax) = self.tax_amount {
                        ui.label(format!("Tax amount: {:.2}", tax.amount()));
                    }
                })
            });
        });
    }
}

fn main() {
    let mut win_options = NativeOptions::default();
    win_options
        .viewport
        .inner_size
        .replace(Vec2::new(360., 540.));

    run_native(
        "Tax Helper",
        win_options,
        Box::new(|cc| Box::new(TaxHelperApp::test(cc))),
    )
    .unwrap();
}
