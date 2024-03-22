use crate::adapter::*;
use crate::holder::*;


use crate::tool::*;
use crate::ManagingApp;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Magazine {
    pub name: String,
    pub contents: Vec<(u32, Option<Tool>, Option<Holder>, Option<Adapter>)>,
}

pub fn select_magazine(app: &mut ManagingApp, ui: &mut egui::Ui) {
    if app.selections.machine.is_none() {
        return;
    }
    let machine_index = app.selections.machine.unwrap();
    let machine = &mut app.machines[machine_index];
    let name = machine.magazines[machine.current_magazine.unwrap()]
        .name
        .clone();
    egui::ComboBox::from_label("Select magazine")
        .selected_text(name)
        .show_ui(ui, |ui| {
            for (i, magazine) in machine.magazines.iter().enumerate() {
                if ui
                    .selectable_label(machine.current_magazine == Some(i), magazine.name.clone())
                    .clicked()
                {
                    machine.current_magazine = Some(i);
                }
            }
        });
}
