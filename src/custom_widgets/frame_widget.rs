
use egui::{Color32, FontId, Pos2, Rect, Sense, Stroke, Vec2};


pub struct Line {
    start: Pos2,
    end: Pos2,
    color: Color32,
    hover_text: String,
    width: f32,
    text_offset: f32,
}

pub struct Text {
    content: String,
    position: Pos2,
    color: Color32,
}

pub struct Arrow {
    start: Pos2,
    end: Pos2,
    color: Color32,
    hover_text: String,
    width: f32,
    text_offset: f32,
}

pub struct DrawingFrame {
    pub size: Vec2,
    lines: Vec<Line>,
    texts: Vec<Text>,
    arrows: Vec<Arrow>,
}

impl DrawingFrame {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            size: Vec2::new(width, height),
            lines: Vec::new(),
            texts: Vec::new(),
            arrows: Vec::new(),
        }
    }

    pub fn add_line(
        &mut self,
        start: Pos2,
        end: Pos2,
        color: Color32,
        hover_text: String,
        width: f32,
        text_offset: f32,
    ) {
        self.lines.push(Line {
            start,
            end,
            color,
            hover_text,
            width,
            text_offset,
        });
    }

    pub fn add_text(&mut self, content: String, position: Pos2, color: Color32) {
        self.texts.push(Text {
            content,
            position,
            color,
        });
    }

    pub fn add_arrow(
        &mut self,
        start: Pos2,
        end: Pos2,
        color: Color32,
        hover_text: String,
        width: f32,
        text_offset: f32,
    ) {
        self.arrows.push(Arrow {
            start,
            end,
            color,
            hover_text,
            width,
            text_offset,
        });
    }
}

impl egui::Widget for DrawingFrame {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());
        let hover_scale = 1.1;
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Draw frame
            painter.rect_stroke(rect, 0.0, Stroke::new(1.0, Color32::WHITE));

            // Create a clipped painter for line segments
            let clipped_painter = painter.with_clip_rect(rect.expand(1.0));

            // Draw all lines and check hover
            for line in &self.lines {
                let start = rect.min + line.start.to_vec2();
                let end = rect.min + line.end.to_vec2();

                // Draw line with clipping
                clipped_painter.line_segment([start, end], Stroke::new(line.width, line.color));

                // Check hovering
                let line_rect = Rect::from_two_pos(
                    Pos2::new(start.x, start.y + line.width * 0.5),
                    Pos2::new(end.x, end.y - line.width * 0.5),
                )
                .expand(2.0);

                if ui.rect_contains_pointer(line_rect) {
                    clipped_painter.line_segment(
                        [start, end],
                        Stroke::new(line.width * hover_scale, line.color),
                    );
                    let text_pos = line_rect.center_top() - Vec2::new(0.0, line.text_offset);
                    let galley = painter.layout_no_wrap(
                        line.hover_text.clone(),
                        FontId::default(),
                        line.color,
                    );
                    let text_rect = Rect::from_min_size(
                        text_pos - Vec2::new(galley.rect.width() / 2.0, galley.rect.height()),
                        galley.rect.size(),
                    );
                    // Draw hover text without clipping
                    painter.rect_filled(text_rect.expand(4.0), 4.0, Color32::LIGHT_GRAY);
                    painter.galley(
                        text_pos - Vec2::new(galley.rect.width() * 0.5, galley.rect.height()),
                        galley,
                        Color32::WHITE,
                    );
                }
            }

            // Draw all texts without clipping
            for text in &self.texts {
                let position = rect.min + text.position.to_vec2();
                let galley =
                    painter.layout_no_wrap(text.content.clone(), FontId::default(), text.color);
                painter.galley(position, galley, text.color);
            }

            // Create a clipped painter for arrows
            let clipped_painter = painter.with_clip_rect(rect.expand(1.0));

            // Draw all arrows and check hover
            for arrow in &self.arrows {
                let start = rect.min + arrow.start.to_vec2();
                let end = rect.min + arrow.end.to_vec2();

                // Draw arrow with clipping
                clipped_painter.arrow(start, end - start, Stroke::new(arrow.width, arrow.color));

                // Check hovering
                let arrow_rect = Rect::from_two_pos(start, end).expand(4.0);
                if ui.rect_contains_pointer(arrow_rect) {
                    clipped_painter.arrow(
                        start,
                        end - start,
                        Stroke::new(arrow.width * hover_scale, arrow.color),
                    );
                    let text_pos = arrow_rect.center_top() - Vec2::new(0.0, arrow.text_offset);
                    painter.text(
                        text_pos,
                        egui::Align2::CENTER_BOTTOM,
                        &arrow.hover_text,
                        egui::FontId::default(),
                        arrow.color,
                    );
                }
            }
        }
        response
    }
}
