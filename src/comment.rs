#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Comment {
    pub comment: String,
}

impl Comment {
    pub fn default() -> Self {
        Self {
            comment: "...".to_string(),
        }
    }
    pub fn display(&mut self, ui: &mut egui::Ui) {
        ui.add(egui::widgets::Label::new(&self.comment).truncate(true))
            .on_hover_text(&self.comment);
    }
}
