use egui::Color32;

use crate::holder::HolderCategory;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Collet {
    pub name: String,
    pub color: Color32,
}

impl Default for Collet {
    fn default() -> Self {
        Self {
            name: "Collet".to_string(),
            color: Color32::LIGHT_BLUE,
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

    pub fn get_category(&self) -> HolderCategory {
        HolderCategory::MillingHolder
    }

    pub fn get_type(&self) -> String {
        "Collet".to_string()
    }
}
