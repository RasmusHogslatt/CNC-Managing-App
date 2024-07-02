use egui::{Color32, Response, Ui};

pub struct LabeledDragValueWidget<'a> {
    label: String,
    value: &'a mut f32,
    color: Color32,
    hover_text: String,
}

impl<'a> LabeledDragValueWidget<'a> {
    pub fn new(label: impl Into<String>, value: &'a mut f32) -> Self {
        Self {
            label: label.into(),
            value,
            color: Color32::WHITE,
            hover_text: String::new(),
        }
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn hover_text(mut self, text: impl Into<String>) -> Self {
        self.hover_text = text.into();
        self
    }

    pub fn show(self, ui: &mut Ui) -> (Response, f32) {
        let label = ui.colored_label(self.color, &self.label);
        if !self.hover_text.is_empty() {
            label.on_hover_text(&self.hover_text);
        }
        let response = ui.add(egui::DragValue::new(self.value));
        (response, *self.value)
    }
}
