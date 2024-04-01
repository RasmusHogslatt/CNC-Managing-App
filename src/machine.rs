use crate::magazine::*;
use crate::reset_states;

use crate::ManagingApp;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Machine {
    pub name: String,
    pub magazines: Vec<Magazine>,
    pub number_of_magazines: usize,
    pub magazine_size: usize,
    pub current_magazine: Option<usize>,
}

pub fn select_machine(app: &mut ManagingApp, ui: &mut egui::Ui) {
    if app.machines.is_empty() {
        ui.label("No machines added");
        return;
    }
    let mut name = "".to_string();
    match app.selections.machine {
        Some(index) => {
            name = app.machines[index].name.clone();
        }
        None => {}
    }

    egui::ComboBox::from_label("Select machine")
        .selected_text(name)
        .show_ui(ui, |ui| {
            for (i, machine) in app.machines.iter().enumerate() {
                if ui
                    .selectable_label(app.selections.machine == Some(i), machine.name.clone())
                    .clicked()
                {
                    app.selections.machine = Some(i);
                }
            }
        });
}

pub fn add_machine(app: &mut ManagingApp, ctx: &egui::Context) {
    let mut is_window_open = true;
    let mut should_add_machine = false;
    egui::Window::new("Add machine")
        .open(&mut is_window_open)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Machine name:");
                ui.text_edit_singleline(&mut app.gui_singletons.machine.name);
            });
            ui.horizontal(|ui| {
                ui.label("Number of magazines:");
                ui.add(
                    egui::Slider::new(&mut app.gui_singletons.machine.number_of_magazines, 1..=4)
                        .text("magazines"),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Magazine size:");
                ui.add(
                    egui::Slider::new(&mut app.gui_singletons.machine.magazine_size, 1..=100)
                        .text("slots"),
                );
            });
            ui.horizontal(|ui| {
                if ui.button("Add machine").clicked() {
                    should_add_machine = true;
                }
            });
            if should_add_machine {
                let mut magazines: Vec<Magazine> = Vec::new();
                for _ in 0..app.gui_singletons.machine.number_of_magazines {
                    let mut current_magazine = Magazine {
                        name: "".to_string(),
                        contents: Vec::new(),
                    };
                    for i in 0..app.gui_singletons.machine.magazine_size {
                        current_magazine.contents.push((i, None, None, None));
                    }
                    current_magazine.name = format!("Magazine {}", magazines.len());
                    magazines.push(current_magazine);
                }
                let machine = Machine {
                    name: app.gui_singletons.machine.name.clone(),
                    magazines: magazines.clone(),
                    number_of_magazines: app.gui_singletons.machine.number_of_magazines,
                    magazine_size: app.gui_singletons.machine.magazine_size,
                    current_magazine: Some(0),
                };
                app.machines.push(machine.clone());

                app.gui_singletons.machine = Machine {
                    name: "Machine".to_string(),
                    magazines: Vec::new(),
                    number_of_magazines: 1,
                    magazine_size: 1,
                    current_magazine: None,
                };
                should_add_machine = false;
                reset_states(app);
            }
        });
    if !is_window_open {
        reset_states(app);
    }
}
