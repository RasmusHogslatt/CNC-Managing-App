

use crate::custom_widgets::*;
use crate::ManagingApp;
use egui::{Color32, Pos2};



use frame_widget::DrawingFrame;
use labeled_drag_value_widget::LabeledDragValueWidget;

pub const FRAME_COLOR: Color32 = Color32::TRANSPARENT;
pub const CLAW_COLOR: Color32 = Color32::BROWN;
pub const ARROW_COLOR: Color32 = Color32::BLACK;
pub const BAR_COLOR: Color32 = Color32::DARK_GRAY;
pub const FACING_STOCK_COLOR: Color32 = Color32::from_rgb(255, 195, 0);
pub const WORKPIECE_COLOR: Color32 = Color32::BLUE;
pub const SAFETY_MARGIN_COLOR: Color32 = Color32::YELLOW;
pub const CUTTING_TOOL_COLOR: Color32 = Color32::RED;
pub const Z_ZERO_COLOR: Color32 = Color32::BLACK;
pub const FRAME_PADDING: f32 = 10.0;
pub const MAX_BAR_LENGTH: f32 = 1000.0;
pub const FRAME_HEIGHT: f32 = 200.0;
pub const LINE_WIDTH: f32 = 25.0;
pub const TEXT_OFFSET: f32 = 20.0;
pub const ARROW_WIDTH: f32 = 3.0;
pub const ARROW_TOP_OFFSET: f32 = 40.0;
use super::calculations::{
    CuttingToolFields, MaterialFields, WorkpieceFields, FRAME_WIDTH, MIN_COLUMN_WIDTH,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct ThreeClawPullingFields {
    pub z_zero: f32,                                        // Front face of material
    pub desired_safety_margin_past_claw_overextension: f32, // Desired safety margin past claw overextension
    pub gripping_point: f32,     // Point where the claw grips the material
    pub claw_overextension: f32, // Distance the claw extends past gripping point towards chuck
    pub scaling_factor: f32,     // Visualization scaling factor
}

pub fn handle_three_claw_pulling(app: &mut ManagingApp, ui: &mut egui::Ui) {
    let three_claw_fields = &mut app.gui_singletons.universal_calculations.three_claw_pulling;
    let material_fields = &mut app.gui_singletons.universal_calculations.material_fields;
    let cutting_tool_fields = &mut app
        .gui_singletons
        .universal_calculations
        .cutting_tool_fields;
    let workpiece_fields = &mut app.gui_singletons.universal_calculations.workpiece_fields;
    egui::Grid::new("three_claw_pulling_grid")
        .num_columns(2)
        .min_col_width(MIN_COLUMN_WIDTH)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Three Claw Pulling").heading());
            ui.end_row();
            let (_response, _) =
                LabeledDragValueWidget::new("Z Zero: ", &mut three_claw_fields.z_zero)
                    .color(Z_ZERO_COLOR)
                    .hover_text("Front face of material")
                    .show(ui);
            ui.end_row();
            let (_response, _) = LabeledDragValueWidget::new(
                "Claw overextension: ",
                &mut three_claw_fields.claw_overextension,
            )
            .color(CLAW_COLOR)
            .hover_text("Distance the claw extends past gripping point towards chuck")
            .show(ui);
            ui.end_row();
            let (_response, _) = LabeledDragValueWidget::new(
                "Desired safety margin past claw overextension: ",
                &mut three_claw_fields.desired_safety_margin_past_claw_overextension,
            )
            .color(SAFETY_MARGIN_COLOR)
            .hover_text("Desired safety margin past claw overextension")
            .show(ui);
            ui.end_row();
            let (response, _) = LabeledDragValueWidget::new(
                "Gripping point: ",
                &mut three_claw_fields.gripping_point,
            )
            .color(CLAW_COLOR)
            .hover_text("Point where the claw grips the material")
            .show(ui);

            if response.changed() {
                calculate_three_claw_pulling(
                    three_claw_fields,
                    material_fields,
                    cutting_tool_fields,
                    workpiece_fields,
                );
            }
        });
    ui.separator();
    egui::Grid::new("material_fields_grid")
        .num_columns(2)
        .min_col_width(MIN_COLUMN_WIDTH)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Material").heading());
            ui.end_row();
            let (_response, _) =
                LabeledDragValueWidget::new("Diameter: ", &mut material_fields.diameter)
                    .color(BAR_COLOR)
                    .hover_text("Diameter of the material")
                    .show(ui);
            ui.end_row();
            let (response, _) =
                LabeledDragValueWidget::new("Length: ", &mut material_fields.length)
                    .color(BAR_COLOR)
                    .hover_text("Length of the bar")
                    .show(ui);
            ui.end_row();
            if response.changed() {
                calculate_three_claw_pulling(
                    three_claw_fields,
                    material_fields,
                    cutting_tool_fields,
                    workpiece_fields,
                );
            }
        });
    ui.separator();
    egui::Grid::new("workpiece_fields_grid")
        .num_columns(2)
        .min_col_width(MIN_COLUMN_WIDTH)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Workpiece").heading());
            ui.end_row();
            let (_response, _) =
                LabeledDragValueWidget::new("Length: ", &mut workpiece_fields.length)
                    .color(WORKPIECE_COLOR)
                    .hover_text("Length of the workpiece")
                    .show(ui);
            ui.end_row();
            let (_response, _) = LabeledDragValueWidget::new(
                "Facing stock right: ",
                &mut workpiece_fields.facing_stock_right,
            )
            .color(FACING_STOCK_COLOR)
            .hover_text("Facing stock on the right side of the workpiece")
            .show(ui);
            ui.end_row();
            let (response, _) = LabeledDragValueWidget::new(
                "Facing stock left: ",
                &mut workpiece_fields.facing_stock_left,
            )
            .color(FACING_STOCK_COLOR)
            .hover_text("Facing stock on the left side of the workpiece")
            .show(ui);
            ui.end_row();
            if response.changed() {
                calculate_three_claw_pulling(
                    three_claw_fields,
                    material_fields,
                    cutting_tool_fields,
                    workpiece_fields,
                );
            }
        });
    ui.separator();
    egui::Grid::new("cutting_tool_fields_grid")
        .num_columns(2)
        .min_col_width(MIN_COLUMN_WIDTH)
        .show(ui, |ui| {
            ui.heading("Cutting Tool");
            ui.end_row();
            let (response, _) =
                LabeledDragValueWidget::new("Width: ", &mut cutting_tool_fields.width)
                    .color(CUTTING_TOOL_COLOR)
                    .hover_text("Width of the cutting tool")
                    .show(ui);
            ui.end_row();
            if response.changed() {
                calculate_three_claw_pulling(
                    three_claw_fields,
                    material_fields,
                    cutting_tool_fields,
                    workpiece_fields,
                );
            }
        });

    ui.separator();
    egui::Grid::new("visualization_grid")
        .num_columns(2)
        .min_col_width(MIN_COLUMN_WIDTH)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Visualization").heading());
            ui.end_row();
            ui.add(
                egui::Slider::new(&mut three_claw_fields.scaling_factor, 0.5..=5.0)
                    .text("Scale")
                    .clamp_to_range(true),
            );
            ui.end_row();
        });

    let drawing_frame = DrawingFrame::new(FRAME_WIDTH, FRAME_HEIGHT);
    let offset_from_right: f32 = 10.0;
    let offset_from_left: f32 = 10.0;
    let bar_y = drawing_frame.size.y * 0.5;

    // Calculate the maximum length that can be displayed
    let max_display_length = (drawing_frame.size.x - offset_from_left - offset_from_right)
        / three_claw_fields.scaling_factor;

    // Calculate positions

    let bar_start = Pos2::new(drawing_frame.size.x - offset_from_right, bar_y);
    let bar_end = Pos2::new(
        bar_start.x - material_fields.length * three_claw_fields.scaling_factor,
        bar_y,
    );

    // Now add all the lines to the drawing frame
    let mut drawing_frame: DrawingFrame = drawing_frame; // Make drawing_frame mutable here

    // Draw the main bar (material length)
    drawing_frame.add_line(
        bar_start,
        bar_end,
        BAR_COLOR,
        format!("Bar Length: {:.2}", material_fields.length),
        material_fields.diameter, // No scaling for y-axis
        TEXT_OFFSET,
    );

    if material_fields.length > max_display_length {
        drawing_frame.add_text(
            "Bar extends past view".to_string(),
            Pos2::new(offset_from_left, drawing_frame.size.y - TEXT_OFFSET),
            Color32::YELLOW,
        );
    }

    // Height values for reuse
    let y_below = bar_y + (material_fields.diameter + LINE_WIDTH) * 0.5;
    let y_above = bar_y - (material_fields.diameter + LINE_WIDTH) * 0.5;

    // Right facing stock value
    let right_facing_stock_end =
        bar_start.x - workpiece_fields.facing_stock_right * three_claw_fields.scaling_factor;
    drawing_frame.add_line(
        Pos2::new(bar_start.x, y_below),
        Pos2::new(right_facing_stock_end, y_below),
        FACING_STOCK_COLOR,
        "Facing stock".to_string(),
        LINE_WIDTH,
        TEXT_OFFSET,
    );

    // Workpiece length
    let workpiece_end =
        right_facing_stock_end - workpiece_fields.length * three_claw_fields.scaling_factor;
    drawing_frame.add_line(
        Pos2::new(right_facing_stock_end, y_below),
        Pos2::new(workpiece_end.max(offset_from_left), y_below),
        WORKPIECE_COLOR,
        "Workpiece".to_string(),
        LINE_WIDTH,
        TEXT_OFFSET,
    );

    // Left facing stock value
    let left_facing_stock_end =
        workpiece_end - workpiece_fields.facing_stock_left * three_claw_fields.scaling_factor;
    drawing_frame.add_line(
        Pos2::new(workpiece_end, y_below),
        Pos2::new(left_facing_stock_end, y_below),
        FACING_STOCK_COLOR,
        "Facing stock".to_string(),
        LINE_WIDTH,
        TEXT_OFFSET,
    );

    let cutting_tool_end =
        left_facing_stock_end - cutting_tool_fields.width * three_claw_fields.scaling_factor;
    drawing_frame.add_line(
        Pos2::new(left_facing_stock_end, y_below),
        Pos2::new(cutting_tool_end, y_below),
        CUTTING_TOOL_COLOR,
        "Cutting tool".to_string(),
        LINE_WIDTH,
        TEXT_OFFSET,
    );

    let claw_gripping_point_start =
        bar_start.x - (three_claw_fields.gripping_point * three_claw_fields.scaling_factor);
    let claw_overextension_end = claw_gripping_point_start
        - (three_claw_fields.claw_overextension) * three_claw_fields.scaling_factor;

    drawing_frame.add_line(
        Pos2::new(claw_gripping_point_start, y_above),
        Pos2::new(claw_overextension_end, y_above),
        CLAW_COLOR,
        "Claw".to_string(),
        LINE_WIDTH,
        TEXT_OFFSET,
    );

    let desired_safe_margin_end = claw_overextension_end
        - (three_claw_fields.desired_safety_margin_past_claw_overextension
            * three_claw_fields.scaling_factor);
    drawing_frame.add_line(
        Pos2::new(claw_overextension_end, y_above),
        Pos2::new(desired_safe_margin_end, y_above),
        SAFETY_MARGIN_COLOR,
        "Safety margin".to_string(),
        LINE_WIDTH,
        TEXT_OFFSET,
    );

    drawing_frame.add_arrow(
        Pos2::new(claw_gripping_point_start, ARROW_TOP_OFFSET),
        Pos2::new(claw_gripping_point_start, bar_y),
        ARROW_COLOR,
        "Gripping point".to_string(),
        ARROW_WIDTH,
        TEXT_OFFSET,
    );
    drawing_frame.add_text(
        format!("Gripping: {:.2}", three_claw_fields.gripping_point),
        Pos2::new(claw_gripping_point_start, TEXT_OFFSET),
        ARROW_COLOR,
    );

    let zero_point = bar_start.x - (three_claw_fields.z_zero * three_claw_fields.scaling_factor);
    drawing_frame.add_arrow(
        Pos2::new(zero_point, ARROW_TOP_OFFSET),
        Pos2::new(zero_point, bar_y),
        Z_ZERO_COLOR,
        "Z-Zero".to_string(),
        ARROW_WIDTH,
        TEXT_OFFSET,
    );
    drawing_frame.add_text(
        format!("Z-Zero: {:.2}", three_claw_fields.z_zero),
        Pos2::new(zero_point, TEXT_OFFSET),
        Z_ZERO_COLOR,
    );

    ui.add(drawing_frame);
}

pub fn calculate_three_claw_pulling(
    _three_claw_fields: &mut ThreeClawPullingFields,
    _material_fields: &mut MaterialFields,
    _cutting_tool_fields: &mut CuttingToolFields,
    _workpiece_fields: &mut WorkpieceFields,
) {
}
