// Length Conversions
pub fn inches_to_centimeters(inches: f32) -> f32 {
    inches * 2.54
}

pub fn centimeters_to_inches(cm: f32) -> f32 {
    cm / 2.54
}

pub fn centimeters_to_feet(cm: f32) -> f32 {
    cm / 30.48
}

pub fn centimeters_to_yards(cm: f32) -> f32 {
    cm / 91.44
}

pub fn feet_to_meters(feet: f32) -> f32 {
    feet * 0.3048
}

pub fn meters_to_feet(meters: f32) -> f32 {
    meters / 0.3048
}

pub fn millimeters_to_inches(mm: f32) -> f32 {
    mm / 25.4
}

pub fn millimeters_to_feet(mm: f32) -> f32 {
    mm / 304.8
}

pub fn millimeters_to_yards(mm: f32) -> f32 {
    mm / 914.4
}

pub fn inches_to_millimeters(inches: f32) -> f32 {
    inches * 25.4
}

pub fn meters_to_inches(meters: f32) -> f32 {
    meters / 0.0254
}

pub fn inches_to_meters(inches: f32) -> f32 {
    inches * 0.0254
}

pub fn meters_to_yards(meters: f32) -> f32 {
    meters / 0.9144
}

pub fn yards_to_meters(yards: f32) -> f32 {
    yards * 0.9144
}

pub fn yards_to_centimeters(yards: f32) -> f32 {
    yards * 91.44
}

pub fn yards_to_millimeters(yards: f32) -> f32 {
    yards * 914.4
}

pub fn feet_to_centimeters(feet: f32) -> f32 {
    feet * 30.48
}

pub fn feet_to_millimeters(feet: f32) -> f32 {
    feet * 304.8
}

// Area Conversions
pub fn square_inches_to_square_centimeters(inches: f32) -> f32 {
    inches * 6.4516
}

pub fn square_inches_to_square_meters(inches: f32) -> f32 {
    inches * 0.00064516
}

pub fn square_inches_to_square_millimeters(inches: f32) -> f32 {
    inches * 645.16
}

pub fn square_feet_to_square_millimeters(feet: f32) -> f32 {
    feet * 92903.04
}

pub fn square_feet_to_square_centimeters(feet: f32) -> f32 {
    feet * 929.03
}

pub fn square_feet_to_square_meters(feet: f32) -> f32 {
    feet * 0.092903
}

pub fn square_yards_to_square_millimeters(yards: f32) -> f32 {
    yards * 836127.36
}

pub fn square_yards_to_square_centimeters(yards: f32) -> f32 {
    yards * 8361.27
}

pub fn square_yards_to_square_meters(yards: f32) -> f32 {
    yards * 0.836127
}

pub fn square_millimeters_to_square_inches(mm: f32) -> f32 {
    mm / 645.16
}

pub fn square_millimeters_to_square_feet(mm: f32) -> f32 {
    mm / 92903.04
}

pub fn square_millimeters_to_square_yards(mm: f32) -> f32 {
    mm / 836127.36
}

pub fn square_centimeters_to_square_inches(cm: f32) -> f32 {
    cm / 6.4516
}

pub fn square_centimeters_to_square_feet(cm: f32) -> f32 {
    cm / 929.03
}

pub fn square_centimeters_to_square_yards(cm: f32) -> f32 {
    cm / 8361.27
}

pub fn square_meters_to_square_inches(meters: f32) -> f32 {
    meters / 0.00064516
}

pub fn square_meters_to_square_feet(meters: f32) -> f32 {
    meters / 0.092903
}

pub fn square_meters_to_square_yards(meters: f32) -> f32 {
    meters / 0.836127
}

// Weight Conversions
pub fn pounds_to_kilograms(pounds: f32) -> f32 {
    pounds * 0.453592
}

pub fn pounds_to_grams(pounds: f32) -> f32 {
    pounds * 453.592
}

pub fn pounds_to_tonnes(pounds: f32) -> f32 {
    pounds / 2204.62
}

pub fn ounces_to_grams(ounces: f32) -> f32 {
    ounces * 28.3495
}

pub fn ounces_to_kilograms(ounces: f32) -> f32 {
    ounces / 35.274
}

pub fn ounces_to_tonnes(ounces: f32) -> f32 {
    ounces / 35274.0
}

pub fn kilograms_to_pounds(kg: f32) -> f32 {
    kg / 0.453592
}

pub fn kilograms_to_ounces(kg: f32) -> f32 {
    kg * 35.274
}

pub fn grams_to_pounds(grams: f32) -> f32 {
    grams / 453.592
}

pub fn grams_to_ounces(grams: f32) -> f32 {
    grams / 28.3495
}

pub fn tonnes_to_pounds(tonnes: f32) -> f32 {
    tonnes * 2204.62
}

pub fn tonnes_to_ounces(tonnes: f32) -> f32 {
    tonnes * 35274.0
}

// Temperature Conversions
pub fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn fahrenheit_to_kelvin(f: f32) -> f32 {
    (f + 459.67) * 5.0 / 9.0
}

pub fn celsius_to_kelvin(c: f32) -> f32 {
    c + 273.15
}

pub fn kelvin_to_celsius(k: f32) -> f32 {
    k - 273.15
}

pub fn kelvin_to_fahrenheit(k: f32) -> f32 {
    k * 9.0 / 5.0 - 459.67
}

pub fn celsius_to_fahrenheit(c: f32) -> f32 {
    (c * 9.0 / 5.0) + 32.0
}
