use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};

pub struct LineSegmentWidget {
    start: Pos2,
    end: Pos2,
    color: Color32,
    stroke_width: f32,
    hover_text: Option<String>,
}

impl LineSegmentWidget {
    pub fn new(start: Pos2, end: Pos2, color: Color32, stroke_width: f32) -> Self {
        Self {
            start,
            end,
            color,
            stroke_width,
            hover_text: None,
        }
    }

    pub fn hover_text(mut self, text: impl Into<String>) -> Self {
        self.hover_text = Some(text.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let (rect, mut response) = ui.allocate_exact_size(
            Vec2::new((self.end.x - self.start.x).abs(), self.stroke_width),
            Sense::hover(),
        );

        if ui.is_rect_visible(rect) {
            ui.painter().line_segment(
                [self.start, self.end],
                Stroke::new(self.stroke_width, self.color),
            );
        }

        if let Some(hover_text) = self.hover_text {
            response = response.on_hover_text(hover_text);
        }

        response
    }
}
