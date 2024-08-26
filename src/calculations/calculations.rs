use std::default;

use crate::calculations::imperialmetricconversion::*;
use crate::calculations::radiandegreeconversion::*;
use crate::three_claw_pulling::*;
use crate::{reset_states, ManagingApp};
use strum::{Display, EnumIter, EnumString};

pub const MIN_COLUMN_WIDTH: f32 = 100.0;
pub const FRAME_WIDTH: f32 = 600.0;

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    EnumIter,
    EnumString,
    Display,
)]
pub enum CalculationType {
    #[default]
    ImperialMetricConversion,
    DegreeRadianConversion,
    GripClawPulling,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UniversalCalculations {
    pub calculation_type: CalculationType,
    pub metric: bool,
    pub imperial_metric_conversion: ImperialMetricConversionFields,
    pub radian_degree_conversion: RadianDegreeConversionFields,
    pub three_claw_pulling: ThreeClawPullingFields,
    pub material_fields: MaterialFields,
    pub cutting_tool_fields: CuttingToolFields,
    pub workpiece_fields: WorkpieceFields,
}

impl Default for UniversalCalculations {
    fn default() -> Self {
        Self {
            metric: true,
            calculation_type: CalculationType::ImperialMetricConversion,
            imperial_metric_conversion: ImperialMetricConversionFields::default(),
            radian_degree_conversion: RadianDegreeConversionFields::default(),
            three_claw_pulling: ThreeClawPullingFields {
                scaling_factor: 1.0,
                ..Default::default()
            },
            material_fields: MaterialFields::default(),
            cutting_tool_fields: CuttingToolFields::default(),
            workpiece_fields: WorkpieceFields::default(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct MaterialFields {
    pub diameter: f32,
    pub length: f32,
}

impl Default for MaterialFields {
    fn default() -> Self {
        Self {
            diameter: 10.0,
            length: 50.0,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct WorkpieceFields {
    pub length: f32,
    pub facing_stock_right: f32,
    pub facing_stock_left: f32,
}

impl Default for WorkpieceFields {
    fn default() -> Self {
        Self {
            length: 10.0,
            facing_stock_right: 0.5,
            facing_stock_left: 0.5,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CuttingToolFields {
    pub width: f32,
}

impl Default for CuttingToolFields {
    fn default() -> Self {
        Self { width: 3.0 }
    }
}

pub fn calculations(app: &mut ManagingApp, ctx: &egui::Context) {
    let mut is_window_open = true;

    egui::Window::new("Utility Calculations")
        .open(&mut is_window_open)
        .show(ctx, |ui| {
            top_calculation_panel(app, ui);
            left_calculation_panel(app, ui);
            central_calculation_panel(app, ui);
        });

    if !is_window_open {
        reset_states(app);
    }
}

pub fn top_calculation_panel(_app: &mut ManagingApp, ui: &mut egui::Ui) {
    egui::TopBottomPanel::top("top_calculation_panel")
        .default_height(20.0)
        .resizable(false)
        .show_inside(ui, |ui| {
                ui.label("All calculations are done in in Millimeter [mm] and Degrees [°] unless otherwise specified.");
        });
}

pub fn left_calculation_panel(app: &mut ManagingApp, ui: &mut egui::Ui) {
    egui::SidePanel::left("left_calculation_panel")
        .default_width(600.0)
        .resizable(false)
        .show_inside(ui, |ui| {
            if ui.button("Imperial/Metric").clicked() {
                app.gui_singletons.universal_calculations.calculation_type =
                    CalculationType::ImperialMetricConversion;
            }
            if ui.button("Degree/Radian").clicked() {
                app.gui_singletons.universal_calculations.calculation_type =
                    CalculationType::DegreeRadianConversion;
            }
            if ui.button("Three Claw Pulling").clicked() {
                app.gui_singletons.universal_calculations.calculation_type =
                    CalculationType::GripClawPulling;
            }
        });
}

pub fn central_calculation_panel(app: &mut ManagingApp, ui: &mut egui::Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        handle_calculation_state_transitions(app, ui);
    });
}

pub fn handle_calculation_state_transitions(app: &mut ManagingApp, ui: &mut egui::Ui) {
    println!(
        "{:?}",
        app.gui_singletons.universal_calculations.calculation_type
    );
    match app.gui_singletons.universal_calculations.calculation_type {
        CalculationType::ImperialMetricConversion => {
            handle_imperial_metric_conversion(app, ui);
        }
        CalculationType::DegreeRadianConversion => {
            handle_radian_degree_conversion(app, ui);
        }
        CalculationType::GripClawPulling => {
            handle_three_claw_pulling(app, ui);
        }
    }
}
