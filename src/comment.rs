#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Comment {
    pub comment: String,
}

impl Comment {
    pub fn default() -> Self {
        Self {
            comment: "__".to_string(),
        }
    }
    pub fn display(&self, ui: &mut egui::Ui) {
        ui.add(egui::widgets::Label::new(&self.comment).truncate(true));
    }
}
