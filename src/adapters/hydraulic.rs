#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Hydraulic {
    pub name: String,
}

impl Default for Hydraulic {
    fn default() -> Self {
        Self {
            name: "Hydraulic".to_string(),
        }
    }
}

impl Hydraulic {
    pub fn hydraulic_edit(&mut self, ui: &mut egui::Ui, add: &mut bool) {
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.separator();
            ui.text_edit_singleline(&mut self.name);
        });
        ui.separator();
        *add = ui.button("Add").clicked()
    }

    pub fn display(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.label(&self.name);
            });
        });
    }
}
