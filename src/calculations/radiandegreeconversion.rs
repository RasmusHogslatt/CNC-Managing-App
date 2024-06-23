use crate::ManagingApp;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct RadianDegreeConversionFields {
    pub radian: f32,
    pub degree: f32,
}

pub fn handle_radian_degree_conversion(app: &mut ManagingApp, ui: &mut egui::Ui) {
    ui.label("Radian/Degree Conversion");
    ui.horizontal(|ui| {
        ui.label("Degree:");
        if ui
            .add(egui::Slider::new(
                &mut app
                    .gui_singletons
                    .universal_calculations
                    .radian_degree_conversion
                    .degree,
                -360.0..=360.0,
            ))
            .changed()
        {
            app.gui_singletons
                .universal_calculations
                .radian_degree_conversion
                .radian = app
                .gui_singletons
                .universal_calculations
                .radian_degree_conversion
                .degree
                * (std::f32::consts::PI / 180.0);
        }
    });
    ui.horizontal(|ui| {
        ui.label("Radian:");
        if ui
            .add(egui::Slider::new(
                &mut app
                    .gui_singletons
                    .universal_calculations
                    .radian_degree_conversion
                    .radian,
                -2.0..=2.0,
            ))
            .changed()
        {
            app.gui_singletons
                .universal_calculations
                .radian_degree_conversion
                .degree = app
                .gui_singletons
                .universal_calculations
                .radian_degree_conversion
                .radian
                * (180.0 / std::f32::consts::PI);
        }
    });
    ui.horizontal(|ui| {
        ui.label(format!(
            "{:.5} degrees",
            app.gui_singletons
                .universal_calculations
                .radian_degree_conversion
                .degree
        ));
        ui.label(" = ");
        ui.label(format!(
            "{:.5} radians",
            app.gui_singletons
                .universal_calculations
                .radian_degree_conversion
                .radian
        ));
    });
}
