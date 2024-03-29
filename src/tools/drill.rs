use crate::tool::ToolCategory;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Drill {
    pub name: String,
    pub diameter: f32,
}
impl Default for Drill {
    fn default() -> Self {
        Self {
            name: "Drill".to_string(),
            diameter: 10.0,
        }
    }
}

impl Drill {
    pub fn tool_edit(&mut self, ui: &mut egui::Ui, add: &mut bool) {
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.separator();
            ui.text_edit_singleline(&mut self.name);
        });
        ui.add(egui::Slider::new(&mut self.diameter, 0.001..=200.0).text("Diameter"));
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
                ui.label("Diameter:");
                ui.label(&self.diameter.to_string());
            });
        });
    }

    pub fn get_category(&self) -> ToolCategory {
        ToolCategory::Rotating
    }

    pub fn get_diameter(&self) -> f32 {
        self.diameter
    }
}
