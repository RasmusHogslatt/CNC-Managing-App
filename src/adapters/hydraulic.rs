use egui::Color32;

use crate::adapter::AdapterCategory;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Hydraulic {
    pub name: String,
    pub color: Color32,
}

impl Default for Hydraulic {
    fn default() -> Self {
        Self {
            name: "Hydraulic".to_string(),
            color: Color32::GREEN,
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
                ui.colored_label(self.color, &self.name);
            });
        });
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_color(&mut self, color: Color32) {
        self.color = color;
    }

    pub fn get_color(&self) -> Color32 {
        self.color
    }

    pub fn get_category(&self) -> AdapterCategory {
        AdapterCategory::Standard
    }

    pub fn get_type(&self) -> String {
        "Hydraulic".to_string()
    }
}
