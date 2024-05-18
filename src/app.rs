use crate::adapter::*;
use crate::holder::*;
use crate::library::*;
use crate::machine::*;
use crate::magazine::*;
use egui::Visuals;
use strum::IntoEnumIterator;

use crate::tool::*;

use crate::resources::*;
use crate::Comment;
use egui::widgets::color_picker::*;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ManagingApp {
    pub machines: Vec<Machine>,
    pub selections: Selections,
    pub app_states: ActiveState,
    pub library: Library,
    pub gui_singletons: GuiSingletons,
    pub display_magazine: Magazine,
    #[serde(skip)]
    pub move_selections: MagazineLibraryMovingSelections,
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
            move_selections: MagazineLibraryMovingSelections::default(),
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
                ui.horizontal(|ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                    if ui.button("Settings").clicked() {
                        self.app_states.app_state = AppState::Settings;
                    }
                });
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

            display_magazine(self, ui, ctx);
            match self.app_states.app_state {
                AppState::ShowMagazine => {
                    // display_magazine(self, ui, ctx);
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
                AppState::Settings => settings(self, ctx),
            }

            match &self.app_states.move_state {
                Some(state) => match state {
                    MoveStates::ToolToMagazine => {
                        self.gui_singletons.tool_filter = None;
                        self.gui_singletons.sort_by = SortBy::Slot;
                        select_tool_from_library(self, ctx);
                        move_tool_to_magazine(self);
                    }
                    MoveStates::ToolToLibrary => {
                        self.gui_singletons.tool_filter = None;
                        self.gui_singletons.sort_by = SortBy::Slot;
                        move_tool_to_library(self);
                    }
                    MoveStates::HolderToMagazine => {
                        select_holder_from_library(self, ctx);
                        move_holder_to_magazine(self);
                    }
                    MoveStates::HolderToLibrary => {
                        move_holder_to_library(self);
                    }
                    MoveStates::AdapterToMagazine => {
                        select_adapter_from_library(self, ctx);
                        move_adapter_to_magazine(self);
                    }
                    MoveStates::AdapterToLibrary => {
                        move_adapter_to_library(self);
                    }
                    MoveStates::EditComment => {
                        edit_comment(self, ctx);
                    }
                },
                None => {}
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
    app.app_states.move_state = None;
    app.move_selections.selected_tool_index_magazine = None;
    app.move_selections.selected_holder_index_magazine = None;
    app.move_selections.selected_adapter_index_magazine = None;
    app.move_selections.selected_tool_index_library = None;
    app.move_selections.selected_holder_index_library = None;
    app.move_selections.selected_adapter_index_library = None;
    app.move_selections.selected_comment_index_magazine = None;
}

pub fn filter_by_tool_category(app: &mut ManagingApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let name;
        if let Some(tool_filter) = &app.gui_singletons.tool_filter {
            name = tool_filter.to_string();
        } else {
            name = "All".to_string();
        }

        if app.machines.is_empty() || app.selections.machine.is_none() {
            return;
        }
        let machine = &mut app.machines[app.selections.machine.unwrap()];
        if machine.current_magazine.is_none() {
            return;
        }
        let magazine = &mut machine.magazines[machine.current_magazine.unwrap()];
        egui::ComboBox::from_label("Filter tool category")
            .selected_text(name)
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(app.gui_singletons.tool_filter.is_none(), "All".to_string())
                    .clicked()
                {
                    app.gui_singletons.tool_filter = None;
                    app.display_magazine.contents = magazine.contents.clone(); //get_sorted_by_tool_diameter(&mut magazine.contents);
                }
                if ui
                    .selectable_label(
                        app.gui_singletons.tool_filter == Some(ToolState::Rotating),
                        "Rotating".to_string(),
                    )
                    .clicked()
                {
                    app.gui_singletons.tool_filter = Some(ToolState::Rotating);
                    app.display_magazine.contents = get_filtered_by_tool_category(
                        &mut magazine.contents,
                        ToolCategory::Rotating,
                    );
                }
                if ui
                    .selectable_label(
                        app.gui_singletons.tool_filter == Some(ToolState::Insert),
                        "Insert".to_string(),
                    )
                    .clicked()
                {
                    app.gui_singletons.tool_filter = Some(ToolState::Insert);
                    app.display_magazine.contents = get_filtered_by_tool_category(
                        &mut magazine.contents,
                        ToolCategory::LatheInsert,
                    );
                }
            });
    });
}

pub fn sort_by(app: &mut ManagingApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let name;
        name = app.gui_singletons.sort_by.to_string();
        if app.machines.is_empty() || app.selections.machine.is_none() {
            return;
        }
        let machine = &mut app.machines[app.selections.machine.unwrap()];
        if machine.current_magazine.is_none() {
            return;
        }
        let magazine = &mut machine.magazines[machine.current_magazine.unwrap()];

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

                    app.display_magazine.contents = get_sorted_by_slot(
                        &mut magazine.contents,
                        app.gui_singletons.tool_filter.clone(),
                    );
                }
                if ui
                    .selectable_label(
                        app.gui_singletons.sort_by == SortBy::Diameter,
                        SortBy::Diameter.to_string(),
                    )
                    .clicked()
                {
                    app.gui_singletons.tool_filter = Some(ToolState::Rotating);
                    app.gui_singletons.sort_by = SortBy::Diameter;
                    app.display_magazine.contents =
                        get_sorted_by_tool_diameter(&mut magazine.contents);
                }
                if ui
                    .selectable_label(
                        app.gui_singletons.sort_by == SortBy::Degree,
                        SortBy::Degree.to_string(),
                    )
                    .clicked()
                {
                    app.gui_singletons.sort_by = SortBy::Degree;
                    app.gui_singletons.tool_filter = Some(ToolState::Insert);
                    app.display_magazine.contents = get_sorted_by_degree(&mut magazine.contents);
                }
            });
    });
}

pub fn settings(app: &mut ManagingApp, ctx: &egui::Context) {
    let mut is_window_open = true;

    egui::Window::new("Settings")
        .open(&mut is_window_open)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::widgets::color_picker::color_picker_color32(
                    ui,
                    &mut app.gui_singletons.color_settings.color,
                    Alpha::Opaque,
                );
            });
            // Set category of content color to change
            egui::ComboBox::from_label("Content type")
                .selected_text(format!(
                    "{:?}",
                    &mut app.gui_singletons.color_settings_state
                ))
                .show_ui(ui, |ui| {
                    for state in ColorSettingsState::iter() {
                        if ui
                            .selectable_label(
                                &mut app.gui_singletons.color_settings_state == &state,
                                state.to_string(),
                            )
                            .clicked()
                        {
                            *&mut app.gui_singletons.color_settings_state = state;
                            app.gui_singletons.color_settings.index = Some(0);
                        }
                    }
                });

            // Select what color specifically to change
            match app.gui_singletons.color_settings_state {
                ColorSettingsState::Rotating => {
                    egui::ComboBox::from_label("Select type of Rotating tool")
                        .selected_text(
                            app.gui_singletons.rotating_tools
                                [app.gui_singletons.color_settings.index.unwrap_or(0)]
                            .get_name(),
                        )
                        .show_ui(ui, |ui| {
                            for (i, tool) in app.gui_singletons.rotating_tools.iter().enumerate() {
                                if ui
                                    .selectable_label(
                                        app.gui_singletons.color_settings.index == Some(i),
                                        tool.get_name(),
                                    )
                                    .clicked()
                                {
                                    app.gui_singletons.color_settings.index = Some(i);
                                }
                            }
                        });
                }
                ColorSettingsState::Insert => {
                    egui::ComboBox::from_label("Select type of Insert tool")
                        .selected_text(
                            app.gui_singletons.insert_tools
                                [app.gui_singletons.color_settings.index.unwrap_or(0)]
                            .get_name(),
                        )
                        .show_ui(ui, |ui| {
                            for (i, tool) in app.gui_singletons.insert_tools.iter().enumerate() {
                                if ui
                                    .selectable_label(
                                        app.gui_singletons.color_settings.index == Some(i),
                                        tool.get_name(),
                                    )
                                    .clicked()
                                {
                                    app.gui_singletons.color_settings.index = Some(i);
                                }
                            }
                        });
                }
                ColorSettingsState::Holder => {
                    egui::ComboBox::from_label("Select type of Holder")
                        .selected_text(
                            app.gui_singletons.holders
                                [app.gui_singletons.color_settings.index.unwrap_or(0)]
                            .get_name(),
                        )
                        .show_ui(ui, |ui| {
                            for (i, tool) in app.gui_singletons.holders.iter().enumerate() {
                                if ui
                                    .selectable_label(
                                        app.gui_singletons.color_settings.index == Some(i),
                                        tool.get_name(),
                                    )
                                    .clicked()
                                {
                                    app.gui_singletons.color_settings.index = Some(i);
                                }
                            }
                        });
                }
                ColorSettingsState::Adapter => {
                    egui::ComboBox::from_label("Select type of Adapter")
                        .selected_text(
                            app.gui_singletons.adapters
                                [app.gui_singletons.color_settings.index.unwrap_or(0)]
                            .get_name(),
                        )
                        .show_ui(ui, |ui| {
                            for (i, tool) in app.gui_singletons.adapters.iter().enumerate() {
                                if ui
                                    .selectable_label(
                                        app.gui_singletons.color_settings.index == Some(i),
                                        tool.get_name(),
                                    )
                                    .clicked()
                                {
                                    app.gui_singletons.color_settings.index = Some(i);
                                }
                            }
                        });
                }
            }
            if ui.button("Apply color").clicked() {
                match app.gui_singletons.color_settings_state {
                    ColorSettingsState::Rotating => {
                        if let Some(index) = app.gui_singletons.color_settings.index {
                            app.gui_singletons.rotating_tools[index]
                                .set_color(app.gui_singletons.color_settings.color);
                        }
                        apply_colors_to_contents(
                            &mut app.display_magazine.contents,
                            &app.gui_singletons,
                        );
                        apply_colors(app);
                    }
                    ColorSettingsState::Insert => {
                        if let Some(index) = app.gui_singletons.color_settings.index {
                            app.gui_singletons.insert_tools[index]
                                .set_color(app.gui_singletons.color_settings.color);
                        }
                        apply_colors_to_contents(
                            &mut app.display_magazine.contents,
                            &app.gui_singletons,
                        );
                        apply_colors(app);
                    }
                    ColorSettingsState::Holder => {
                        if let Some(index) = app.gui_singletons.color_settings.index {
                            app.gui_singletons.holders[index]
                                .set_color(app.gui_singletons.color_settings.color);
                        }
                        apply_colors_to_contents(
                            &mut app.display_magazine.contents,
                            &app.gui_singletons,
                        );
                        apply_colors(app);
                    }

                    ColorSettingsState::Adapter => {
                        if let Some(index) = app.gui_singletons.color_settings.index {
                            app.gui_singletons.adapters[index]
                                .set_color(app.gui_singletons.color_settings.color);
                        }
                        apply_colors_to_contents(
                            &mut app.display_magazine.contents,
                            &app.gui_singletons,
                        );
                        apply_colors(app);
                    }
                }
                // Call functions to update all colors everywhere
            }
        });
    if !is_window_open {
        reset_states(app);
    }
}

pub fn apply_colors(app: &mut ManagingApp) {
    // apply_colors_to_contents(&mut app.display_magazine.contents, &app.gui_singletons);
    for machine in app.machines.iter_mut() {
        for magazine in machine.magazines.iter_mut() {
            apply_colors_to_contents(&mut magazine.contents, &app.gui_singletons);
        }
    }
}

pub fn apply_colors_to_contents(
    contents: &mut Vec<(
        usize,
        Option<Tool>,
        Option<Holder>,
        Option<Adapter>,
        Comment,
    )>,
    gui_singletons: &GuiSingletons,
) {
    for content in contents.iter_mut() {
        if let Some(tool) = &mut content.1 {
            for rotating_tool in gui_singletons.rotating_tools.iter() {
                if rotating_tool.get_type() == tool.get_type() {
                    tool.set_color(rotating_tool.get_color());
                }
            }
            for insert_tool in gui_singletons.insert_tools.iter() {
                if insert_tool.get_type() == tool.get_type() {
                    tool.set_color(insert_tool.get_color());
                }
            }
            if let Some(holder) = &mut content.2 {
                for holder_singleton in gui_singletons.holders.iter() {
                    if holder_singleton.get_type() == holder.get_type() {
                        holder.set_color(holder_singleton.get_color());
                    }
                }
            }
            if let Some(adapter) = &mut content.3 {
                for adapter_singleton in gui_singletons.adapters.iter() {
                    if adapter_singleton.get_type() == adapter.get_type() {
                        adapter.set_color(adapter_singleton.get_color());
                    }
                }
            }
        }
    }
}
