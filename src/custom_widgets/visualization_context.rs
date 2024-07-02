use egui::{Color32, Pos2, Rect, Ui};

use crate::{arrow_widget::ArrowWidget, line_segment_widget::LineSegmentWidget};

pub struct VisualizationContext<'a> {
    pub ui: &'a mut Ui,
    pub frame_rect: Rect,
    pub frame_padding: f32,
    pub width_scale: f32,
    pub height_scale: f32,
}

impl<'a> VisualizationContext<'a> {
    pub fn new(
        ui: &'a mut Ui,
        frame_rect: Rect,
        frame_padding: f32,
        width_scale: f32,
        height_scale: f32,
    ) -> Self {
        Self {
            ui,
            frame_rect,
            frame_padding,
            width_scale,
            height_scale,
        }
    }

    pub fn draw_horizontal_line(
        &mut self,
        x_start: f32,
        x_end: f32,
        y: f32,
        color: Color32,
        width: f32,
        hover_text: impl Into<String>,
    ) {
        let start = Pos2::new(
            self.frame_rect.min.x + self.frame_padding + x_start * self.width_scale,
            y,
        );
        let end = Pos2::new(
            self.frame_rect.min.x + self.frame_padding + x_end * self.width_scale,
            y,
        );

        LineSegmentWidget::new(start, end, color, width)
            .hover_text(hover_text)
            .show(self.ui);
    }

    pub fn draw_vertical_line(
        &mut self,
        x: f32,
        y_start: f32,
        y_end: f32,
        color: Color32,
        width: f32,
        hover_text: impl Into<String>,
    ) {
        let start = Pos2::new(
            self.frame_rect.min.x + self.frame_padding + x * self.width_scale,
            y_start,
        );
        let end = Pos2::new(
            self.frame_rect.min.x + self.frame_padding + x * self.width_scale,
            y_end,
        );

        LineSegmentWidget::new(start, end, color, width)
            .hover_text(hover_text)
            .show(self.ui);
    }

    pub fn draw_vertical_arrow(
        &mut self,
        x: f32,
        y_start: f32,
        y_end: f32,
        color: Color32,
        width: f32,
        hover_text: impl Into<String>,
    ) {
        let start = Pos2::new(
            self.frame_rect.min.x + self.frame_padding + x * self.width_scale,
            self.frame_rect.min.y + y_start,
        );
        let end = Pos2::new(
            self.frame_rect.min.x + self.frame_padding + x * self.width_scale,
            self.frame_rect.min.y + y_end,
        );

        ArrowWidget::new(start, end, color, width)
            .hover_text(hover_text)
            .show(self.ui);
    }
}

pub fn calculate_bar_end_x(context: &VisualizationContext, material_length: f32) -> f32 {
    context.frame_rect.min.x + context.frame_padding + material_length * context.width_scale
}
