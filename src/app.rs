use egui::Visuals;

use crate::adapter::*;

use crate::holder::*;
use crate::library::*;
use crate::machine::*;
use crate::magazine::*;

use crate::tool::*;

use crate::resources::*;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ManagingApp {
    pub machines: Vec<Machine>,
    pub selections: Selections,
    pub app_states: ActiveState,
    pub library: Library,
    #[serde(skip)]
    pub gui_singletons: GuiSingletons,
    pub display_magazine: Magazine, // TODO: Clone sorted/filtered content here
}

impl Default for ManagingApp {
    fn default() -> Self {
        Self {
            // create optional mutable reference to one machine in vector
            machines: Vec::new(),
            selections: Selections::default(),
            app_states: ActiveState::default(),
            gui_singletons: GuiSingletons::default(),
            library: Library::default(),
            display_magazine: Magazine {
                name: "None selected".to_string(),
                contents: Vec::new(),
            },
        }
    }
}

impl ManagingApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(Visuals {
            dark_mode: true,
            ..Default::default()
        });

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

pub fn log_states(app: &ManagingApp) {
    println!("App state: {:?}", app.app_states.app_state);
    println!("Add tool state: {:?}", app.app_states.add_tool_state);
    println!("Add holder state: {:?}", app.app_states.add_holder_state);
    println!("Add adapter state: {:?}", app.app_states.add_adapter_state);
    println!(
        "Magazine content state: {:?}",
        app.app_states.magazine_content_state
    );
    println!("Number of machines: {:?}", app.machines.len());
}

impl eframe::App for ManagingApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //log_states(self);
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(300.0)
            .show(ctx, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.separator();
                if ui
                    .add(egui::Button::new("Show library"))
                    .on_hover_text("Library of tools, holders and adapters")
                    .clicked()
                {
                    print!("Show library button clicked!");
                    self.app_states.app_state = AppState::ShowLibrary;
                }
                ui.separator();
                if ui
                    .add(egui::Button::new("Add machine"))
                    .on_hover_text("Configure a new machine")
                    .clicked()
                {
                    print!("Add machine button clicked!");
                    self.app_states.app_state = AppState::AddMachine;
                }
                ui.separator();
                if ui
                    .add(egui::Button::new("Add tool"))
                    .on_hover_text("Configure a new tool")
                    .clicked()
                {
                    print!("Add tool button clicked!");
                    self.app_states.app_state = AppState::AddTool;
                }
                if ui
                    .add(egui::Button::new("Add holder"))
                    .on_hover_text("Configure a new holder")
                    .clicked()
                {
                    print!("Add holder button clicked!");
                    self.app_states.app_state = AppState::AddHolder;
                }
                if ui
                    .add(egui::Button::new("Add adapter"))
                    .on_hover_text("Configure a new adapter")
                    .clicked()
                {
                    print!("Add adapter button clicked!");
                    self.app_states.app_state = AppState::AddAdapter;
                }
                ui.separator();

                select_machine(self, ui);
                select_magazine(self, ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            filter_by_tool_category(self, ui);
            sort_by(self, ui);
            match self.app_states.app_state {
                AppState::ShowMagazine => {
                    display_magazine(&self.display_magazine, ui); // TODO: Change to clone stuff
                }
                AppState::AddTool => add_tool(self, ctx),
                AppState::AddHolder => add_holder(self, ctx),
                AppState::AddAdapter => add_adapter(self, ctx),
                AppState::AddMachine => add_machine(self, ctx),
                AppState::ShowLibrary => {
                    if !self.library.display(ctx) {
                        reset_states(self);
                    }
                }
            }
        });
    }
}

pub fn reset_states(app: &mut ManagingApp) {
    app.app_states.app_state = AppState::ShowMagazine;
    app.app_states.add_tool_state = None;
    app.app_states.add_holder_state = None;
    app.app_states.add_adapter_state = None;
    app.app_states.magazine_content_state = None;
    app.selections.selected_rotating_tool_index = 0;
    app.selections.selected_insert_tool_index = 0;
}

pub fn filter_by_tool_category(app: &mut ManagingApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let mut name = "".to_string();
        if let Some(tool_filter) = &app.gui_singletons.tool_filter {
            name = tool_filter.to_string();
        } else {
            name = "All".to_string();
        }
        egui::ComboBox::from_label("Select tool category")
            .selected_text(name)
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(app.gui_singletons.tool_filter.is_none(), "All".to_string())
                    .clicked()
                {
                    app.gui_singletons.tool_filter = None;
                    // Apply filter and sort
                }
                if ui
                    .selectable_label(
                        app.gui_singletons.tool_filter == Some(ToolState::Rotating),
                        "Rotating".to_string(),
                    )
                    .clicked()
                {
                    app.gui_singletons.tool_filter = Some(ToolState::Rotating);
                    // Apply filter and sort
                }
                if ui
                    .selectable_label(
                        app.gui_singletons.tool_filter == Some(ToolState::Insert),
                        "Insert".to_string(),
                    )
                    .clicked()
                {
                    app.gui_singletons.tool_filter = Some(ToolState::Insert);
                    // Apply filter and sort
                }
            });
    });
}

pub fn sort_by(app: &mut ManagingApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let mut name = "".to_string();
        name = app.gui_singletons.sort_by.to_string();
        egui::ComboBox::from_label("Sort by")
            .selected_text(name)
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(
                        app.gui_singletons.sort_by == SortBy::Slot,
                        SortBy::Slot.to_string(),
                    )
                    .clicked()
                {
                    app.gui_singletons.sort_by = SortBy::Slot;
                    // Apply filter and sort
                }
                if ui
                    .selectable_label(
                        app.gui_singletons.sort_by == SortBy::Diameter,
                        SortBy::Diameter.to_string(),
                    )
                    .clicked()
                {
                    app.gui_singletons.sort_by = SortBy::Diameter;
                    // Apply filter and sort
                }
                if ui
                    .selectable_label(
                        app.gui_singletons.sort_by == SortBy::Degree,
                        SortBy::Degree.to_string(),
                    )
                    .clicked()
                {
                    app.gui_singletons.sort_by = SortBy::Degree;
                    // Apply filter and sort
                }
            });
    });
}
