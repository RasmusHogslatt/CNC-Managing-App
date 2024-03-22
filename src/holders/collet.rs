#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Collet {
    pub name: String,
}

impl Default for Collet {
    fn default() -> Self {
        Self {
            name: "Collet".to_string(),
        }
    }
}

impl Collet {
    pub fn holder_edit(&mut self, ui: &mut egui::Ui, add: &mut bool) {
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.separator();
            ui.text_edit_singleline(&mut self.name);
        });
        ui.separator();
        *add = ui.button("Add").clicked()
    }

    pub fn display(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.label(&self.name);
            });
        });
    }
}
