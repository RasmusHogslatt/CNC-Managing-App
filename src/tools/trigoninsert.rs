use crate::tool::ToolCategory;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct TrigonInsert {
    pub name: String,
    pub degree: f32,
}
impl Default for TrigonInsert {
    fn default() -> Self {
        Self {
            name: "Trigon insert".to_string(),
            degree: 35.0,
        }
    }
}

impl TrigonInsert {
    pub fn tool_edit(&mut self, ui: &mut egui::Ui, add: &mut bool) {
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.separator();
            ui.text_edit_singleline(&mut self.name);
        });
        ui.add(egui::Slider::new(&mut self.degree, 0.001..=200.0).text("Degree"));
        ui.separator();
        *add = ui.button("Add").clicked()
    }

    pub fn display(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.label(&self.name);
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Degree:");
                ui.label(&self.degree.to_string());
            });
        });
    }

    pub fn get_category(&self) -> ToolCategory {
        ToolCategory::LatheInsert
    }

    pub fn get_degree(&self) -> f32 {
        self.degree
    }
}
