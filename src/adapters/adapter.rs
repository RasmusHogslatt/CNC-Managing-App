use crate::app::*;
use crate::hydraulic::*;
use crate::resources::*;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum AdapterCategory {
    #[default]
    Empty,
    Standard,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Adapter {
    Hydraulic(Hydraulic),
}

impl Adapter {
    pub fn adapter_edit(&mut self, ui: &mut egui::Ui, add: &mut bool) {
        match self {
            Adapter::Hydraulic(hydraulic) => hydraulic.hydraulic_edit(ui, add),
        }
    }

    pub fn display(&mut self, ui: &mut egui::Ui) {
        match self {
            Adapter::Hydraulic(hydraulic) => hydraulic.display(ui),
        }
    }
}

pub fn add_adapter(app: &mut ManagingApp, ctx: &egui::Context) {
    let mut is_window_open = true;
    egui::Window::new("Add adapter")
        .open(&mut is_window_open)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Hydraulic").clicked() {
                    app.app_states.add_adapter_state = Some(AddAdapterState::Standard);
                }
            });
            let mut should_add_adapter = false;
            match app.app_states.add_adapter_state {
                Some(AddAdapterState::Standard) => {
                    add_standard_adapter(app, ui, &mut should_add_adapter);
                }
                None => {
                    // add hydraulic adapter by default
                }
            }
        });
    if !is_window_open {
        reset_states(app);
    }
}

pub fn modify_adapter(adapter: &mut Adapter, ui: &mut egui::Ui, add: &mut bool) {
    adapter.adapter_edit(ui, add);
}

pub fn add_standard_adapter(
    app: &mut ManagingApp,
    ui: &mut egui::Ui,
    should_add_adapter: &mut bool,
) {
    let selected_adapter_name = match app
        .gui_singletons
        .adapters
        .get(app.selections.selected_adapter_index)
    {
        Some(Adapter::Hydraulic(hydraulic)) => hydraulic.name.clone(),
        None => "Select adapter".to_string(),
    };
    egui::ComboBox::from_label("Adapter")
        .selected_text(selected_adapter_name)
        .show_ui(ui, |ui| {
            for (i, adapter) in app.gui_singletons.adapters.iter().enumerate() {
                let label = match adapter {
                    Adapter::Hydraulic(hydraulic) => hydraulic.name.clone(),
                };
                if ui
                    .selectable_label(app.selections.selected_adapter_index == i, label)
                    .clicked()
                {
                    app.selections.selected_adapter_index = i;
                }
            }
        });
    ui.separator();
    modify_adapter(
        &mut app.gui_singletons.adapters[app.selections.selected_adapter_index],
        ui,
        should_add_adapter,
    );
    if *should_add_adapter {
        let cloned_adapter =
            app.gui_singletons.adapters[app.selections.selected_adapter_index].clone();
        app.library.adapters.push(cloned_adapter);
        app.app_states.add_adapter_state = None;
        app.app_states.app_state = AppState::ShowMagazine;
        *should_add_adapter = false;
    }
}
