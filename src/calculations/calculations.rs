use crate::calculations::imperialmetricconversion::*;
use crate::calculations::radiandegreeconversion::*;
use crate::{reset_states, ManagingApp};
use strum::{Display, EnumIter, EnumString};

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
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UniversalCalculations {
    pub calculation_type: CalculationType,
    pub metric: bool,
    pub imperial_metric_conversion: ImperialMetricConversionFields,
    pub radian_degree_conversion: RadianDegreeConversionFields,
}

impl Default for UniversalCalculations {
    fn default() -> Self {
        Self {
            metric: true,
            calculation_type: CalculationType::ImperialMetricConversion,
            imperial_metric_conversion: ImperialMetricConversionFields::default(),
            radian_degree_conversion: RadianDegreeConversionFields::default(),
        }
    }
}

pub fn calculations(app: &mut ManagingApp, ctx: &egui::Context) {
    let mut is_window_open = true;

    egui::Window::new("Utility Calculations")
        .open(&mut is_window_open)
        .show(ctx, |ui| {
            ui.label("This is the calculations page");
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
        .resizable(true)
        .show_inside(ui, |_ui| {
            // ui.horizontal(|ui| {
            //     ui.radio_value(
            //         &mut app.gui_singletons.universal_calculations.metric,
            //         true,
            //         "Metric",
            //     );
            //     ui.radio_value(
            //         &mut app.gui_singletons.universal_calculations.metric,
            //         false,
            //         "Imperial",
            //     );
            // });
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
    }
}
