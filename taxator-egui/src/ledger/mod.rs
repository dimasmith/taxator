//! Components to render income ledger
use eframe::egui::{Grid, InnerResponse, Response, Ui, Widget};

use taxator_calculator::money::Money;

pub struct LedgerView<'a> {
    ledger: &'a [Money],
    can_delete: bool,
    deleted_index: Option<&'a mut Option<usize>>,
    calculate_totals: bool,
}

impl<'a> LedgerView<'a> {
    pub fn new(ledger: &'a [Money]) -> Self {
        Self {
            ledger,
            can_delete: false,
            deleted_index: None,
            calculate_totals: false,
        }
    }

    pub fn with_delete(mut self, deleted_index: &'a mut Option<usize>) -> Self {
        self.can_delete = true;
        self.deleted_index.replace(deleted_index);
        self
    }

    pub fn with_totals(mut self) -> Self {
        self.calculate_totals = true;
        self
    }

    pub fn show_ui(self, ui: &mut Ui) -> InnerResponse<Option<usize>> {
        let num_columns = if self.can_delete { 3 } else { 2 };
        let mut deleted_index = None;
        let component = Grid::new("ledger")
            .num_columns(num_columns)
            .striped(true)
            .show(ui, |ui| {
                for (idx, money) in self.ledger.iter().enumerate() {
                    ui.label(format!("{}", idx));
                    ui.label(format!("{}", money));
                    if self.can_delete && ui.button("X").clicked() {
                        deleted_index.replace(idx);
                    }
                    ui.end_row();
                }

                if self.calculate_totals {
                    let total = Money::total(self.ledger).unwrap();
                    ui.label("Total: ");
                    ui.label(format!("{}", total));
                    if self.can_delete {
                        ui.label("");
                    }

                    ui.end_row();
                }
            });

        if self.can_delete {
            if let Some(idx) = deleted_index {
                let ei = self.deleted_index.unwrap();
                ei.replace(idx);
            }
        }

        InnerResponse::new(None, component.response)
    }
}

impl<'a> Widget for LedgerView<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let num_columns = if self.can_delete { 3 } else { 2 };
        let mut deleted_idx = None;
        let component = Grid::new("ledger")
            .num_columns(num_columns)
            .striped(true)
            .show(ui, |ui| {
                for (idx, money) in self.ledger.iter().enumerate() {
                    ui.label(format!("{}", idx));
                    ui.label(format!("{}", money));
                    if self.can_delete && ui.button("X").clicked() {
                        deleted_idx = Some(idx);
                    }
                    ui.end_row();
                }
            });

        component.response
    }
}
