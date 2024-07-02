use crate::custom_widgets::*;
use crate::ManagingApp;
use egui::{Color32, Pos2};
use egui::{Frame, Painter, Rect, Stroke, Ui, Vec2};
use labeled_drag_value_widget::LabeledDragValueWidget;
use visualization_context::calculate_bar_end_x;
use visualization_context::VisualizationContext;

pub const FRAME_COLOR: Color32 = Color32::TRANSPARENT;
pub const CLAW_COLOR: Color32 = Color32::BROWN;
pub const CLAW_ARROW_COLOR: Color32 = Color32::BLACK;
pub const BAR_COLOR: Color32 = Color32::DARK_GRAY;
pub const FACING_STOCK_COLOR: Color32 = Color32::YELLOW;
pub const WORKPIECE_COLOR: Color32 = Color32::BLUE;
pub const SAFETY_MARGIN_COLOR: Color32 = Color32::YELLOW;
pub const CUTTING_TOOL_COLOR: Color32 = Color32::RED;
pub const Z_ZERO_COLOR: Color32 = Color32::BLACK;
pub const FRAME_PADDING: f32 = 10.0;
pub const MAX_BAR_LENGTH: f32 = 1000.0;
pub const FRAME_HEIGHT: f32 = 200.0;

use super::calculations::{
    CuttingToolFields, MaterialFields, WorkpieceFields, FRAME_WIDTH, MIN_COLUMN_WIDTH,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct ThreeClawPullingFields {
    pub z_zero: f32,                                        // Front face of material
    pub desired_safety_margin_past_claw_overextension: f32, // Desired safety margin past claw overextension
    pub gripping_point: f32,      // Point where the claw grips the material
    pub claw_overextension: f32,  // Distance the claw extends past gripping point towards chuck
    pub total_safety_margin: f32, // Total safety margin
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
            let (response, _) =
                LabeledDragValueWidget::new("Z Zero: ", &mut three_claw_fields.z_zero)
                    .color(Z_ZERO_COLOR)
                    .hover_text("Front face of material")
                    .show(ui);
            ui.end_row();
            let (response, _) = LabeledDragValueWidget::new(
                "Claw overextension: ",
                &mut three_claw_fields.claw_overextension,
            )
            .color(CLAW_COLOR)
            .hover_text("Distance the claw extends past gripping point towards chuck")
            .show(ui);
            ui.end_row();
            let (response, _) = LabeledDragValueWidget::new(
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
            let (response, _) =
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
            let (response, _) =
                LabeledDragValueWidget::new("Length: ", &mut workpiece_fields.length)
                    .color(WORKPIECE_COLOR)
                    .hover_text("Length of the workpiece")
                    .show(ui);
            ui.end_row();
            let (response, _) = LabeledDragValueWidget::new(
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
    ui.label(egui::RichText::new("Visualization").heading());

    // Visualize the gripping
    visualize_gripping(
        ui,
        Vec2::new(FRAME_WIDTH, FRAME_HEIGHT),
        three_claw_fields,
        material_fields,
        cutting_tool_fields,
        workpiece_fields,
    );
}

pub fn calculate_three_claw_pulling(
    three_claw_fields: &mut ThreeClawPullingFields,
    material_fields: &mut MaterialFields,
    cutting_tool_fields: &mut CuttingToolFields,
    workpiece_fields: &mut WorkpieceFields,
) {
}

pub fn visualize_gripping(
    ui: &mut egui::Ui,
    size: Vec2,
    three_claw_fields: &mut ThreeClawPullingFields,
    material_fields: &mut MaterialFields,
    cutting_tool_fields: &mut CuttingToolFields,
    workpiece_fields: &mut WorkpieceFields,
) {
    let frame = Frame::none()
        .fill(FRAME_COLOR)
        .stroke(Stroke::new(1.0, FRAME_COLOR))
        .inner_margin(10.0)
        .outer_margin(5.0)
        .rounding(1.0);

    frame.show(ui, |ui| {
        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        let frame_rect = response.rect;

        let width_scale = (frame_rect.width() - 2.0 * FRAME_PADDING) / MAX_BAR_LENGTH;
        let height_scale = material_fields.diameter / frame_rect.height() * 10.0;

        let mut context =
            VisualizationContext::new(ui, frame_rect, FRAME_PADDING, width_scale, height_scale);

        draw_z_zero_arrow(&mut context, material_fields, three_claw_fields);
        draw_facing_stock_right(&mut context, material_fields, workpiece_fields);
        draw_workpiece_length(&mut context, material_fields, workpiece_fields);
        draw_facing_stock_left(&mut context, material_fields, workpiece_fields);
        draw_bar_length(&mut context, material_fields);
        draw_cutting_tools(
            &mut context,
            material_fields,
            workpiece_fields,
            cutting_tool_fields,
        );
        let claw_end_x = draw_claw_overextension(
            &mut context,
            material_fields,
            workpiece_fields,
            three_claw_fields,
        );
        draw_safety_margin(&mut context, three_claw_fields, claw_end_x);
    });
}

pub fn draw_bar_length(context: &mut VisualizationContext, material_fields: &MaterialFields) {
    context.draw_horizontal_line(
        0.0,
        material_fields.length,
        context.frame_rect.center().y,
        BAR_COLOR,
        material_fields.diameter * context.height_scale,
        format!("Bar length: {} mm", material_fields.length),
    );
}

pub fn draw_facing_stock_right(
    context: &mut VisualizationContext,
    material_fields: &MaterialFields,
    workpiece_fields: &WorkpieceFields,
) {
    let facing_stock_height = 20.0;
    let bar_end_x = calculate_bar_end_x(context, material_fields.length);

    context.draw_horizontal_line(
        material_fields.length - workpiece_fields.facing_stock_right,
        material_fields.length,
        context.frame_rect.center().y - facing_stock_height,
        FACING_STOCK_COLOR,
        facing_stock_height,
        format!(
            "Right facing stock: {} mm",
            workpiece_fields.facing_stock_right
        ),
    );
}

pub fn draw_workpiece_length(
    context: &mut VisualizationContext,
    material_fields: &MaterialFields,
    workpiece_fields: &WorkpieceFields,
) {
    let workpiece_height = 20.0;
    let bar_end_x = calculate_bar_end_x(context, material_fields.length);
    let workpiece_start_x = bar_end_x - workpiece_fields.facing_stock_right * context.width_scale;

    context.draw_horizontal_line(
        material_fields.length - workpiece_fields.facing_stock_right - workpiece_fields.length,
        material_fields.length - workpiece_fields.facing_stock_right,
        context.frame_rect.center().y + workpiece_height,
        WORKPIECE_COLOR,
        workpiece_height,
        format!("Workpiece length: {} mm", workpiece_fields.length),
    );

    // Draw vertical lines connecting workpiece to the bar
    context.draw_vertical_line(
        material_fields.length - workpiece_fields.facing_stock_right,
        context.frame_rect.center().y,
        context.frame_rect.center().y + workpiece_height,
        WORKPIECE_COLOR,
        1.0,
        "Workpiece start",
    );
    context.draw_vertical_line(
        material_fields.length - workpiece_fields.facing_stock_right - workpiece_fields.length,
        context.frame_rect.center().y,
        context.frame_rect.center().y + workpiece_height,
        WORKPIECE_COLOR,
        1.0,
        "Workpiece end",
    );
}

pub fn draw_facing_stock_left(
    context: &mut VisualizationContext,
    material_fields: &MaterialFields,
    workpiece_fields: &WorkpieceFields,
) {
    let facing_stock_height = 20.0;
    let workpiece_end_x =
        material_fields.length - workpiece_fields.facing_stock_right - workpiece_fields.length;

    context.draw_horizontal_line(
        workpiece_end_x - workpiece_fields.facing_stock_left,
        workpiece_end_x,
        context.frame_rect.center().y - facing_stock_height,
        FACING_STOCK_COLOR,
        facing_stock_height,
        format!(
            "Left facing stock: {} mm",
            workpiece_fields.facing_stock_left
        ),
    );

    // Draw a line connecting left facing stock to the bar
    context.draw_vertical_line(
        workpiece_end_x - workpiece_fields.facing_stock_left,
        context.frame_rect.center().y - facing_stock_height,
        context.frame_rect.center().y,
        FACING_STOCK_COLOR,
        1.0,
        "Left facing stock end",
    );
}

pub fn draw_safety_margin(
    context: &mut VisualizationContext,
    three_claw_fields: &ThreeClawPullingFields,
    claw_end_x: f32,
) {
    let margin_height = 10.0;
    let margin_end_x = claw_end_x
        - three_claw_fields.desired_safety_margin_past_claw_overextension * context.width_scale;

    context.draw_horizontal_line(
        (margin_end_x - context.frame_rect.min.x - context.frame_padding) / context.width_scale,
        (claw_end_x - context.frame_rect.min.x - context.frame_padding) / context.width_scale,
        context.frame_rect.center().y - margin_height / 2.0,
        SAFETY_MARGIN_COLOR,
        margin_height,
        format!(
            "Safety margin: {} mm",
            three_claw_fields.desired_safety_margin_past_claw_overextension
        ),
    );

    // Draw a line connecting margin end to the bar
    context.draw_vertical_line(
        (margin_end_x - context.frame_rect.min.x - context.frame_padding) / context.width_scale,
        context.frame_rect.center().y - margin_height / 2.0,
        context.frame_rect.center().y,
        SAFETY_MARGIN_COLOR,
        1.0,
        "Safety margin end",
    );
}

pub fn draw_claw_overextension(
    context: &mut VisualizationContext,
    material_fields: &MaterialFields,
    workpiece_fields: &WorkpieceFields,
    three_claw_fields: &ThreeClawPullingFields,
) -> f32 {
    let claw_height = 15.0;
    let bar_end_x = calculate_bar_end_x(context, material_fields.length);
    let gripping_point_x = bar_end_x - three_claw_fields.gripping_point * context.width_scale;
    let claw_end_x = gripping_point_x - three_claw_fields.claw_overextension * context.width_scale;

    context.draw_horizontal_line(
        material_fields.length
            - three_claw_fields.gripping_point
            - three_claw_fields.claw_overextension,
        material_fields.length - three_claw_fields.gripping_point,
        context.frame_rect.center().y - claw_height / 2.0,
        CLAW_COLOR,
        claw_height,
        format!(
            "Claw overextension: {} mm",
            three_claw_fields.claw_overextension
        ),
    );

    // Draw vertical arrow at the gripping point
    context.draw_vertical_arrow(
        material_fields.length - three_claw_fields.gripping_point,
        context.frame_padding,
        context.frame_rect.center().y - claw_height / 2.0,
        CLAW_ARROW_COLOR,
        2.0,
        format!(
            "Gripping point: {} mm from end",
            three_claw_fields.gripping_point
        ),
    );

    // Add a label for gripping point
    let text_pos = Pos2::new(
        gripping_point_x,
        context.frame_rect.min.y + context.frame_padding,
    );
    context.ui.painter().text(
        text_pos,
        egui::Align2::CENTER_BOTTOM,
        "Gripping point",
        egui::FontId::proportional(14.0),
        CLAW_ARROW_COLOR,
    );

    claw_end_x
}

pub fn draw_cutting_tools(
    context: &mut VisualizationContext,
    material_fields: &MaterialFields,
    workpiece_fields: &WorkpieceFields,
    cutting_tool_fields: &CuttingToolFields,
) {
    let cutting_tool_height = 25.0;
    let workpiece_end_x =
        material_fields.length - workpiece_fields.facing_stock_right - workpiece_fields.length;
    let cutting_tool_start_x = workpiece_end_x - workpiece_fields.facing_stock_left;

    context.draw_horizontal_line(
        cutting_tool_start_x - cutting_tool_fields.width,
        cutting_tool_start_x,
        context.frame_rect.center().y,
        CUTTING_TOOL_COLOR,
        cutting_tool_height,
        format!("Cutting tool width: {} mm", cutting_tool_fields.width),
    );
}

pub fn calculate_claw_end_x(
    frame_rect: Rect,
    material_fields: &MaterialFields,
    three_claw_fields: &ThreeClawPullingFields,
    frame_padding: f32,
    width_scale: f32,
) -> f32 {
    let bar_end_x = frame_rect.min.x + frame_padding + material_fields.length * width_scale;
    let gripping_point_x = bar_end_x - three_claw_fields.gripping_point * width_scale;
    gripping_point_x - three_claw_fields.claw_overextension * width_scale
}

pub fn draw_z_zero_arrow(
    context: &mut VisualizationContext,
    material_fields: &MaterialFields,
    three_claw_fields: &ThreeClawPullingFields,
) {
    let arrow_height = 30.0;
    let bar_end_x = calculate_bar_end_x(context, material_fields.length);
    let z_zero_x = bar_end_x - three_claw_fields.z_zero * context.width_scale;

    context.draw_vertical_arrow(
        three_claw_fields.z_zero,
        context.frame_padding,
        context.frame_rect.center().y - arrow_height / 2.0,
        Z_ZERO_COLOR,
        2.0,
        format!("Z-zero: {} mm", three_claw_fields.z_zero),
    );

    // Add the label separately
    context.ui.painter().text(
        Pos2::new(z_zero_x, context.frame_rect.min.y + context.frame_padding),
        egui::Align2::CENTER_BOTTOM,
        "Z-zero",
        egui::FontId::proportional(14.0),
        Z_ZERO_COLOR,
    );
}
