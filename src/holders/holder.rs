use crate::app::*;
use crate::collet::*;
use crate::resources::*;
use crate::ManagingApp;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum HolderCategory {
    #[default]
    Empty,
    MillingHolder,
    DrillingHolder,
    TurningHolder,
    SpecialtyHolder,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Holder {
    Collet(Collet),
}

impl Holder {
    pub fn holder_edit(&mut self, ui: &mut egui::Ui, add: &mut bool) {
        match self {
            Holder::Collet(collet) => collet.holder_edit(ui, add),
        }
    }

    pub fn display(&self, ui: &mut egui::Ui) {
        match self {
            Holder::Collet(collet) => collet.display(ui),
        }
    }
}

pub fn modify_holder(holder: &mut Holder, ui: &mut egui::Ui, add: &mut bool) {
    holder.holder_edit(ui, add);
}

pub fn add_holder(app: &mut ManagingApp, ctx: &egui::Context) {
    let mut is_window_open = true;
    egui::Window::new("Add holder")
        .open(&mut is_window_open)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Standard").clicked() {
                    app.app_states.add_holder_state = Some(HolderState::Standard);
                }
            });
            let mut should_add_holder = false;
            match app.app_states.add_holder_state {
                Some(HolderState::Standard) => {
                    add_standard_holder(app, ui, &mut should_add_holder);
                }
                None => {
                    // add standard holder by default
                }
            }
        });
    if !is_window_open {
        reset_states(app);
    }
}

pub fn add_standard_holder(app: &mut ManagingApp, ui: &mut egui::Ui, should_add_holder: &mut bool) {
    let selected_holder_name = match app
        .gui_singletons
        .holders
        .get(app.selections.selected_holder_index)
    {
        Some(Holder::Collet(collet)) => collet.name.clone(),
        None => String::from("Select holder"),
    };
    egui::ComboBox::from_label("Select type of holder")
        .selected_text(selected_holder_name)
        .show_ui(ui, |ui| {
            // get index to selected holder
            for (i, holder) in app.gui_singletons.holders.iter().enumerate() {
                let label = match holder {
                    Holder::Collet(collet) => collet.name.clone(),
                };
                if ui
                    .selectable_label(app.selections.selected_holder_index == i, label)
                    .clicked()
                {
                    app.selections.selected_holder_index = i;
                }
            }
        });
    ui.separator();
    modify_holder(
        &mut app.gui_singletons.holders[app.selections.selected_holder_index],
        ui,
        should_add_holder,
    );
    if *should_add_holder {
        let cloned_holder =
            app.gui_singletons.holders[app.selections.selected_holder_index].clone();
        app.library.holders.push(cloned_holder);
        app.app_states.add_holder_state = None;
        app.app_states.app_state = AppState::ShowMagazine;
        *should_add_holder = false;
    }
}
