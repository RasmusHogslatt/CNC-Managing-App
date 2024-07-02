use core::fmt;

use strum::{Display, EnumIter, EnumString};

use crate::{utility_calculations::*, ManagingApp};

use super::calculations::MIN_COLUMN_WIDTH;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ImperialMetricConversionFields {
    // Length
    pub metric_unit_length: MetricLength,
    pub imperial_unit_length: ImperialLength,

    pub metric_length_mm: f32,
    pub metric_length_cm: f32,
    pub metric_length_m: f32,
    pub imperial_length_inch: f32,
    pub imperial_length_feet: f32,
    pub imperial_length_yard: f32,

    // Area
    pub metric_unit_area: MetricArea,
    pub imperial_unit_area: ImperialArea,

    pub metric_area_mm2: f32,
    pub metric_area_cm2: f32,
    pub metric_area_m2: f32,
    pub imperial_area_inch2: f32,
    pub imperial_area_feet2: f32,
    pub imperial_area_yard2: f32,

    // Weight
    pub metric_unit_weight: MetricWeight,
    pub imperial_unit_weight: ImperialWeight,

    pub metric_weight_g: f32,
    pub metric_weight_kg: f32,
    pub metric_weight_tonne: f32,
    pub imperial_weight_ounce: f32,
    pub imperial_weight_pound: f32,

    // Temperature (all displayed at once)
    pub metric_temperature_celsius: f32,
    pub metric_temperature_kelvin: f32,
    pub imperial_temperature_fahrenheit: f32,
}
impl Default for ImperialMetricConversionFields {
    fn default() -> Self {
        Self {
            metric_unit_length: Default::default(),
            imperial_unit_length: Default::default(),
            metric_length_mm: Default::default(),
            metric_length_cm: Default::default(),
            metric_length_m: Default::default(),
            imperial_length_inch: Default::default(),
            imperial_length_feet: Default::default(),
            imperial_length_yard: Default::default(),
            metric_unit_area: Default::default(),
            imperial_unit_area: Default::default(),
            metric_area_mm2: Default::default(),
            metric_area_cm2: Default::default(),
            metric_area_m2: Default::default(),
            imperial_area_inch2: Default::default(),
            imperial_area_feet2: Default::default(),
            imperial_area_yard2: Default::default(),
            metric_unit_weight: Default::default(),
            imperial_unit_weight: Default::default(),
            metric_weight_g: Default::default(),
            metric_weight_kg: Default::default(),
            metric_weight_tonne: Default::default(),
            imperial_weight_ounce: Default::default(),
            imperial_weight_pound: Default::default(),
            metric_temperature_celsius: 0.0,
            metric_temperature_kelvin: 273.15,
            imperial_temperature_fahrenheit: 32.0,
        }
    }
}

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
pub enum MetricLength {
    #[default]
    Millimeter,
    Centimeter,
    Meter,
}

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
pub enum ImperialLength {
    #[default]
    Inch,
    Feet,
    Yard,
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq, EnumIter, EnumString,
)]
pub enum MetricArea {
    #[default]
    Millimeter,
    Centimeter,
    Meter,
}

impl fmt::Display for MetricArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MetricArea::Millimeter => write!(f, "mm²"),
            MetricArea::Centimeter => write!(f, "cm²"),
            MetricArea::Meter => write!(f, "m²"),
        }
    }
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq, EnumIter, EnumString,
)]
pub enum ImperialArea {
    #[default]
    Inch,
    Feet,
    Yard,
}

impl fmt::Display for ImperialArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImperialArea::Inch => write!(f, "in²"),
            ImperialArea::Feet => write!(f, "ft²"),
            ImperialArea::Yard => write!(f, "yd²"),
        }
    }
}

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
pub enum MetricWeight {
    #[default]
    Gram,
    Kilogram,
    Tonne,
}

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
pub enum ImperialWeight {
    #[default]
    Ounce,
    Pound,
}

pub fn handle_imperial_metric_conversion(app: &mut ManagingApp, ui: &mut egui::Ui) {
    handle_length_conversion(
        ui,
        &mut app
            .gui_singletons
            .universal_calculations
            .imperial_metric_conversion,
    );
    ui.separator();
    handle_area_conversion(
        &mut app
            .gui_singletons
            .universal_calculations
            .imperial_metric_conversion,
        ui,
    );
    ui.separator();
    handle_weight_conversion(
        &mut app
            .gui_singletons
            .universal_calculations
            .imperial_metric_conversion,
        ui,
    );
    ui.separator();
    handle_temperature_conversion(
        &mut app
            .gui_singletons
            .universal_calculations
            .imperial_metric_conversion,
        ui,
    );
}

pub fn handle_length_conversion(
    ui: &mut egui::Ui,
    conversion: &mut ImperialMetricConversionFields,
) {
    ui.heading("Length Conversion");
    ui.horizontal(|ui| {
        egui::ComboBox::from_label("Length Metric Unit")
            .selected_text(&conversion.metric_unit_length.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut conversion.metric_unit_length,
                    MetricLength::Millimeter,
                    "Millimeter",
                );
                ui.selectable_value(
                    &mut conversion.metric_unit_length,
                    MetricLength::Centimeter,
                    "Centimeter",
                );
                ui.selectable_value(
                    &mut conversion.metric_unit_length,
                    MetricLength::Meter,
                    "Meter",
                );
            });
        egui::ComboBox::from_label("Length Imperial Unit")
            .selected_text(&conversion.imperial_unit_length.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut conversion.imperial_unit_length,
                    ImperialLength::Inch,
                    "Inch",
                );
                ui.selectable_value(
                    &mut conversion.imperial_unit_length,
                    ImperialLength::Feet,
                    "Feet",
                );
                ui.selectable_value(
                    &mut conversion.imperial_unit_length,
                    ImperialLength::Yard,
                    "Yard",
                );
            });
    });

    ui.horizontal(|ui| match conversion.metric_unit_length {
        MetricLength::Millimeter => {
            egui::Grid::new("metric_length_conversion_grid")
                .num_columns(2)
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("Millimeters:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.metric_length_mm))
                        .changed()
                    {
                        conversion.imperial_length_inch =
                            millimeters_to_inches(conversion.metric_length_mm);
                        conversion.imperial_length_feet =
                            millimeters_to_feet(conversion.metric_length_mm);
                        conversion.imperial_length_yard =
                            millimeters_to_yards(conversion.metric_length_mm);
                    }
                });
        }
        MetricLength::Centimeter => {
            egui::Grid::new("metric_length_conversion_grid")
                .num_columns(2)
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("Centimeters:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.metric_length_cm))
                        .changed()
                    {
                        conversion.imperial_length_inch =
                            centimeters_to_inches(conversion.metric_length_cm);
                        conversion.imperial_length_feet =
                            centimeters_to_feet(conversion.metric_length_cm);
                        conversion.imperial_length_yard =
                            centimeters_to_yards(conversion.metric_length_cm);
                    }
                });
        }
        MetricLength::Meter => {
            egui::Grid::new("metric_length_conversion_grid")
                .num_columns(2)
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("Meters:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.metric_length_m))
                        .changed()
                    {
                        conversion.imperial_length_inch =
                            meters_to_inches(conversion.metric_length_m);
                        conversion.imperial_length_feet =
                            meters_to_feet(conversion.metric_length_m);
                        conversion.imperial_length_yard =
                            meters_to_yards(conversion.metric_length_m);
                    }
                });
        }
    });

    ui.horizontal(|ui| match conversion.imperial_unit_length {
        ImperialLength::Inch => {
            egui::Grid::new("imperial_length_conversion_grid")
                .num_columns(2)
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("Inches:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.imperial_length_inch))
                        .changed()
                    {
                        conversion.metric_length_mm =
                            inches_to_millimeters(conversion.imperial_length_inch);
                        conversion.metric_length_cm =
                            inches_to_centimeters(conversion.imperial_length_inch);
                        conversion.metric_length_m =
                            inches_to_meters(conversion.imperial_length_inch);
                    }
                });
        }
        ImperialLength::Feet => {
            egui::Grid::new("imperial_length_conversion_grid")
                .num_columns(2)
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("Feet:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.imperial_length_feet))
                        .changed()
                    {
                        conversion.metric_length_mm =
                            feet_to_millimeters(conversion.imperial_length_feet);
                        conversion.metric_length_cm =
                            feet_to_centimeters(conversion.imperial_length_feet);
                        conversion.metric_length_m =
                            feet_to_meters(conversion.imperial_length_feet);
                    }
                });
        }
        ImperialLength::Yard => {
            egui::Grid::new("imperial_length_conversion_grid")
                .num_columns(2)
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("Yards:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.imperial_length_yard))
                        .changed()
                    {
                        conversion.metric_length_mm =
                            yards_to_millimeters(conversion.imperial_length_yard);
                        conversion.metric_length_cm =
                            yards_to_centimeters(conversion.imperial_length_yard);
                        conversion.metric_length_m =
                            yards_to_meters(conversion.imperial_length_yard);
                    }
                });
        }
    });

    // Display converted values
    ui.horizontal(|ui| {
        match conversion.metric_unit_length {
            MetricLength::Millimeter => {
                ui.label(format!("{} mm", conversion.metric_length_mm));
            }
            MetricLength::Centimeter => {
                ui.label(format!("{} cm", conversion.metric_length_cm));
            }
            MetricLength::Meter => {
                ui.label(format!("{} m", conversion.metric_length_m));
            }
        }
        ui.label(" = ");
        match conversion.imperial_unit_length {
            ImperialLength::Inch => {
                ui.label(format!("{} in", conversion.imperial_length_inch));
            }
            ImperialLength::Feet => {
                ui.label(format!("{} ft", conversion.imperial_length_feet));
            }
            ImperialLength::Yard => {
                ui.label(format!("{} yd", conversion.imperial_length_yard));
            }
        }
    });
}

pub fn handle_area_conversion(conversion: &mut ImperialMetricConversionFields, ui: &mut egui::Ui) {
    ui.heading("Area Conversion");

    ui.horizontal(|ui| {
        egui::ComboBox::from_label("Area Metric Unit")
            .selected_text(conversion.metric_unit_area.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut conversion.metric_unit_area,
                    MetricArea::Millimeter,
                    "mm²",
                );
                ui.selectable_value(
                    &mut conversion.metric_unit_area,
                    MetricArea::Centimeter,
                    "cm²",
                );
                ui.selectable_value(&mut conversion.metric_unit_area, MetricArea::Meter, "m²");
            });
        egui::ComboBox::from_label("Area mperial Unit")
            .selected_text(conversion.imperial_unit_area.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut conversion.imperial_unit_area,
                    ImperialArea::Inch,
                    "in²",
                );
                ui.selectable_value(
                    &mut conversion.imperial_unit_area,
                    ImperialArea::Feet,
                    "ft²",
                );
                ui.selectable_value(
                    &mut conversion.imperial_unit_area,
                    ImperialArea::Yard,
                    "yd²",
                );
            });
    });

    ui.horizontal(|ui| match conversion.metric_unit_area {
        MetricArea::Millimeter => {
            egui::Grid::new("metric_area_conversion_grid")
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("mm²:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.metric_area_mm2))
                        .changed()
                    {
                        conversion.imperial_area_inch2 =
                            square_millimeters_to_square_inches(conversion.metric_area_mm2);
                        conversion.imperial_area_feet2 =
                            square_millimeters_to_square_feet(conversion.metric_area_mm2);
                        conversion.imperial_area_yard2 =
                            square_millimeters_to_square_yards(conversion.metric_area_mm2);
                    }
                });
        }
        MetricArea::Centimeter => {
            egui::Grid::new("metric_area_conversion_grid")
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("cm²:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.metric_area_cm2))
                        .changed()
                    {
                        conversion.imperial_area_inch2 =
                            square_centimeters_to_square_inches(conversion.metric_area_cm2);
                        conversion.imperial_area_feet2 =
                            square_centimeters_to_square_feet(conversion.metric_area_cm2);
                        conversion.imperial_area_yard2 =
                            square_centimeters_to_square_yards(conversion.metric_area_cm2);
                    }
                });
        }
        MetricArea::Meter => {
            egui::Grid::new("metric_area_conversion_grid")
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("m²:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.metric_area_m2))
                        .changed()
                    {
                        conversion.imperial_area_inch2 =
                            square_meters_to_square_inches(conversion.metric_area_m2);
                        conversion.imperial_area_feet2 =
                            square_meters_to_square_feet(conversion.metric_area_m2);
                        conversion.imperial_area_yard2 =
                            square_meters_to_square_yards(conversion.metric_area_m2);
                    }
                });
        }
    });

    ui.horizontal(|ui| match conversion.imperial_unit_area {
        ImperialArea::Inch => {
            egui::Grid::new("imperial_area_conversion_grid")
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("in²:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.imperial_area_inch2))
                        .changed()
                    {
                        conversion.metric_area_m2 =
                            square_inches_to_square_meters(conversion.imperial_area_inch2);
                        conversion.metric_area_cm2 =
                            square_inches_to_square_centimeters(conversion.imperial_area_inch2);
                        conversion.metric_area_mm2 =
                            square_inches_to_square_millimeters(conversion.imperial_area_inch2);
                    }
                });
        }
        ImperialArea::Feet => {
            egui::Grid::new("imperial_area_conversion_grid")
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("ft²:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.imperial_area_feet2))
                        .changed()
                    {
                        conversion.metric_area_m2 =
                            square_feet_to_square_meters(conversion.imperial_area_feet2);
                        conversion.metric_area_cm2 =
                            square_feet_to_square_centimeters(conversion.imperial_area_feet2);
                        conversion.metric_area_mm2 =
                            square_feet_to_square_millimeters(conversion.imperial_area_feet2);
                    }
                });
        }
        ImperialArea::Yard => {
            egui::Grid::new("imperial_area_conversion_grid")
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("yd²:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.imperial_area_yard2))
                        .changed()
                    {
                        conversion.metric_area_m2 =
                            square_yards_to_square_meters(conversion.imperial_area_yard2);
                        conversion.metric_area_cm2 =
                            square_yards_to_square_centimeters(conversion.imperial_area_yard2);
                        conversion.metric_area_mm2 =
                            square_yards_to_square_millimeters(conversion.imperial_area_yard2);
                    }
                });
        }
    });

    ui.horizontal(|ui| {
        match conversion.metric_unit_area {
            MetricArea::Millimeter => {
                ui.label(format!("{} mm²", conversion.metric_area_mm2));
            }
            MetricArea::Centimeter => {
                ui.label(format!("{} cm²", conversion.metric_area_cm2));
            }
            MetricArea::Meter => {
                ui.label(format!("{} m²", conversion.metric_area_m2));
            }
        }
        ui.label(" = ");
        match conversion.imperial_unit_area {
            ImperialArea::Inch => {
                ui.label(format!("{} in²", conversion.imperial_area_inch2));
            }
            ImperialArea::Feet => {
                ui.label(format!("{} ft²", conversion.imperial_area_feet2));
            }
            ImperialArea::Yard => {
                ui.label(format!("{} yd²", conversion.imperial_area_yard2));
            }
        }
    });
}

pub fn handle_weight_conversion(
    conversion: &mut ImperialMetricConversionFields,
    ui: &mut egui::Ui,
) {
    ui.heading("Weight Conversion");

    ui.horizontal(|ui| {
        egui::ComboBox::from_label("Weight Metric Unit")
            .selected_text(conversion.metric_unit_weight.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut conversion.metric_unit_weight, MetricWeight::Gram, "g");
                ui.selectable_value(
                    &mut conversion.metric_unit_weight,
                    MetricWeight::Kilogram,
                    "kg",
                );
                ui.selectable_value(
                    &mut conversion.metric_unit_weight,
                    MetricWeight::Tonne,
                    "tonne",
                );
            });
        egui::ComboBox::from_label("Weight Imperial Unit")
            .selected_text(conversion.imperial_unit_weight.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut conversion.imperial_unit_weight,
                    ImperialWeight::Ounce,
                    "oz",
                );
                ui.selectable_value(
                    &mut conversion.imperial_unit_weight,
                    ImperialWeight::Pound,
                    "lb",
                );
            });
    });

    ui.horizontal(|ui| match conversion.metric_unit_weight {
        MetricWeight::Gram => {
            egui::Grid::new("metric_weight_conversion_grid")
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("g:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.metric_weight_g))
                        .changed()
                    {
                        conversion.imperial_weight_ounce =
                            grams_to_ounces(conversion.metric_weight_g);
                        conversion.imperial_weight_pound =
                            grams_to_pounds(conversion.metric_weight_g);
                    }
                });
        }
        MetricWeight::Kilogram => {
            egui::Grid::new("metric_weight_conversion_grid")
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("kg:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.metric_weight_kg))
                        .changed()
                    {
                        conversion.imperial_weight_ounce =
                            kilograms_to_ounces(conversion.metric_weight_kg);
                        conversion.imperial_weight_pound =
                            kilograms_to_pounds(conversion.metric_weight_kg);
                    }
                });
        }
        MetricWeight::Tonne => {
            egui::Grid::new("metric_weight_conversion_grid")
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("tonne:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.metric_weight_tonne))
                        .changed()
                    {
                        conversion.imperial_weight_ounce =
                            tonnes_to_ounces(conversion.metric_weight_tonne);
                        conversion.imperial_weight_pound =
                            tonnes_to_pounds(conversion.metric_weight_tonne);
                    }
                });
        }
    });

    ui.horizontal(|ui| match conversion.imperial_unit_weight {
        ImperialWeight::Ounce => {
            egui::Grid::new("imperial_weight_conversion_grid")
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("oz:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.imperial_weight_ounce))
                        .changed()
                    {
                        conversion.metric_weight_g =
                            ounces_to_grams(conversion.imperial_weight_ounce);
                        conversion.metric_weight_kg =
                            ounces_to_kilograms(conversion.imperial_weight_ounce);
                        conversion.metric_weight_tonne =
                            ounces_to_tonnes(conversion.imperial_weight_ounce);
                    }
                });
        }
        ImperialWeight::Pound => {
            egui::Grid::new("imperial_weight_conversion_grid")
                .min_col_width(MIN_COLUMN_WIDTH)
                .show(ui, |ui| {
                    ui.label("lb:");
                    if ui
                        .add(egui::DragValue::new(&mut conversion.imperial_weight_pound))
                        .changed()
                    {
                        conversion.metric_weight_g =
                            pounds_to_grams(conversion.imperial_weight_pound);
                        conversion.metric_weight_kg =
                            pounds_to_kilograms(conversion.imperial_weight_pound);
                        conversion.metric_weight_tonne =
                            pounds_to_tonnes(conversion.imperial_weight_pound);
                    }
                });
        }
    });

    ui.horizontal(|ui| {
        match conversion.metric_unit_weight {
            MetricWeight::Gram => {
                ui.label(format!("{} g", conversion.metric_weight_g));
            }
            MetricWeight::Kilogram => {
                ui.label(format!("{} kg", conversion.metric_weight_kg));
            }
            MetricWeight::Tonne => {
                ui.label(format!("{} tonne", conversion.metric_weight_tonne));
            }
        }
        ui.label(" = ");
        match conversion.imperial_unit_weight {
            ImperialWeight::Ounce => {
                ui.label(format!("{} oz", conversion.imperial_weight_ounce));
            }
            ImperialWeight::Pound => {
                ui.label(format!("{} lb", conversion.imperial_weight_pound));
            }
        }
    });
}

pub fn handle_temperature_conversion(
    conversion: &mut ImperialMetricConversionFields,
    ui: &mut egui::Ui,
) {
    ui.heading("Temperature Conversion");

    ui.horizontal(|ui| {
        egui::Grid::new("celsius_conversion_grid")
            .min_col_width(MIN_COLUMN_WIDTH)
            .show(ui, |ui| {
                ui.label("Celsius:");
                if ui
                    .add(
                        egui::DragValue::new(&mut conversion.metric_temperature_celsius)
                            .clamp_range(-273.15..=std::f32::MAX),
                    )
                    .changed()
                {
                    conversion.metric_temperature_kelvin =
                        celsius_to_kelvin(conversion.metric_temperature_celsius);
                    conversion.imperial_temperature_fahrenheit =
                        celsius_to_fahrenheit(conversion.metric_temperature_celsius);
                }
            });
    });

    ui.horizontal(|ui| {
        egui::Grid::new("kelvin_conversion_grid")
            .min_col_width(MIN_COLUMN_WIDTH)
            .show(ui, |ui| {
                ui.label("Kelvin:");
                if ui
                    .add(
                        egui::DragValue::new(&mut conversion.metric_temperature_kelvin)
                            .clamp_range(0.0..=std::f32::MAX),
                    )
                    .changed()
                {
                    conversion.metric_temperature_celsius =
                        kelvin_to_celsius(conversion.metric_temperature_kelvin);
                    conversion.imperial_temperature_fahrenheit =
                        kelvin_to_fahrenheit(conversion.metric_temperature_kelvin);
                }
            });
    });

    ui.horizontal(|ui| {
        egui::Grid::new("fahrenheit_conversion_grid")
            .min_col_width(MIN_COLUMN_WIDTH)
            .show(ui, |ui| {
                ui.label("Fahrenheit:");
                if ui
                    .add(
                        egui::DragValue::new(&mut conversion.imperial_temperature_fahrenheit)
                            .clamp_range(-459.67..=std::f32::MAX),
                    )
                    .changed()
                {
                    conversion.metric_temperature_celsius =
                        fahrenheit_to_celsius(conversion.imperial_temperature_fahrenheit);
                    conversion.metric_temperature_kelvin =
                        fahrenheit_to_kelvin(conversion.imperial_temperature_fahrenheit);
                }
            });
    });

    ui.horizontal(|ui| {
        ui.label(format!(
            "{} °C = {} °K = {} °F",
            conversion.metric_temperature_celsius,
            conversion.metric_temperature_kelvin,
            conversion.imperial_temperature_fahrenheit
        ));
    });
}
