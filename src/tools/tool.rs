use crate::app::*;
use crate::drill::Drill;
use crate::mill::Mill;
use crate::resources::*;
use crate::trigoninsert::TrigonInsert;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum ToolCategory {
    #[default]
    All,
    Rotating,
    LatheInsert,
    Empty,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Tool {
    Drill(Drill),
    Mill(Mill),
    TrigonInsert(TrigonInsert),
}

impl Tool {
    pub fn tool_edit(&mut self, ui: &mut egui::Ui, add: &mut bool) {
        match self {
            Tool::Drill(drill) => drill.tool_edit(ui, add),
            Tool::Mill(mill) => mill.tool_edit(ui, add),
            Tool::TrigonInsert(trigon_insert) => trigon_insert.tool_edit(ui, add),
        }
    }

    pub fn display(&self, ui: &mut egui::Ui) {
        match self {
            Tool::Drill(drill) => drill.display(ui),
            Tool::Mill(mill) => mill.display(ui),
            Tool::TrigonInsert(trigon_insert) => trigon_insert.display(ui),
        }
    }

    pub fn get_category(&self) -> ToolCategory {
        match self {
            Tool::Drill(_) => ToolCategory::Rotating,
            Tool::Mill(_) => ToolCategory::Rotating,
            Tool::TrigonInsert(_) => ToolCategory::LatheInsert,
        }
    }

    pub fn get_diameter(&self) -> f32 {
        match self {
            Tool::Drill(drill) => drill.diameter,
            Tool::Mill(mill) => mill.diameter,
            Tool::TrigonInsert(_) => 0.0,
        }
    }

    pub fn get_degree(&self) -> f32 {
        match self {
            Tool::Drill(_) => 0.0,
            Tool::Mill(_) => 0.0,
            Tool::TrigonInsert(trigon_insert) => trigon_insert.degree,
        }
    }
}

pub fn add_tool(app: &mut ManagingApp, ctx: &egui::Context) {
    let mut is_window_open = true;
    egui::Window::new("Add tool")
        .open(&mut is_window_open)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Rotating").clicked() {
                    app.app_states.add_tool_state = Some(ToolState::Rotating);
                }
                if ui.button("Insert").clicked() {
                    app.app_states.add_tool_state = Some(ToolState::Insert);
                }
            });
            let mut should_add_tool = false;
            match app.app_states.add_tool_state {
                Some(ToolState::Rotating) => add_rotating_tool(app, ui, &mut should_add_tool),
                Some(ToolState::Insert) => {
                    add_insert_tool(app, ui, &mut should_add_tool);
                }
                None => {}
            }
        });
    if !is_window_open {
        reset_states(app);
    }
}

pub fn add_rotating_tool(app: &mut ManagingApp, ui: &mut egui::Ui, should_add_tool: &mut bool) {
    let selected_tool_name = match app
        .gui_singletons
        .rotating_tools
        .get(app.selections.selected_rotating_tool_index)
    {
        Some(Tool::Drill(drill)) => drill.name.clone(),
        Some(Tool::Mill(mill)) => mill.name.clone(),
        Some(Tool::TrigonInsert(trigon_insert)) => trigon_insert.name.clone(),
        None => String::from("Select tool"),
    };

    egui::ComboBox::from_label("Select type of tool")
        .selected_text(selected_tool_name)
        .show_ui(ui, |ui| {
            // get index to selected tool
            for (i, tool) in app.gui_singletons.rotating_tools.iter().enumerate() {
                let label = match tool {
                    Tool::Drill(drill) => drill.name.clone(),
                    Tool::Mill(mill) => mill.name.clone(),
                    Tool::TrigonInsert(trigon_insert) => trigon_insert.name.clone(),
                };
                if ui
                    .selectable_label(app.selections.selected_rotating_tool_index == i, label)
                    .clicked()
                {
                    app.selections.selected_rotating_tool_index = i;
                }
            }
        });
    ui.separator();
    modify_tool(
        &mut app.gui_singletons.rotating_tools[app.selections.selected_rotating_tool_index],
        ui,
        should_add_tool,
    );
    if *should_add_tool {
        let cloned_tool =
            app.gui_singletons.rotating_tools[app.selections.selected_rotating_tool_index].clone();
        app.library.tools.push(cloned_tool);
        app.app_states.add_tool_state = None;
        app.app_states.app_state = AppState::ShowMagazine;
        *should_add_tool = false;
    }
}

pub fn add_insert_tool(app: &mut ManagingApp, ui: &mut egui::Ui, should_add_tool: &mut bool) {
    let selected_tool_name = match app
        .gui_singletons
        .insert_tools
        .get(app.selections.selected_rotating_tool_index)
    {
        Some(Tool::Drill(drill)) => drill.name.clone(),
        Some(Tool::Mill(mill)) => mill.name.clone(),
        Some(Tool::TrigonInsert(trigon_insert)) => trigon_insert.name.clone(),
        None => String::from("Select tool"),
    };
    egui::ComboBox::from_label("Select type of tool")
        .selected_text(selected_tool_name)
        .show_ui(ui, |ui| {
            // get index to selected tool
            for (i, tool) in app.gui_singletons.insert_tools.iter().enumerate() {
                let label = match tool {
                    Tool::Drill(drill) => drill.name.clone(),
                    Tool::Mill(mill) => mill.name.clone(),
                    Tool::TrigonInsert(trigon_insert) => trigon_insert.name.clone(),
                };
                if ui
                    .selectable_label(app.selections.selected_insert_tool_index == i, label)
                    .clicked()
                {
                    app.selections.selected_insert_tool_index = i;
                }
            }
        });
    ui.separator();
    modify_tool(
        &mut app.gui_singletons.insert_tools[app.selections.selected_insert_tool_index],
        ui,
        should_add_tool,
    );
    if *should_add_tool {
        let cloned_tool =
            app.gui_singletons.insert_tools[app.selections.selected_insert_tool_index].clone();
        app.library.tools.push(cloned_tool);
        app.app_states.add_tool_state = None;
        app.app_states.app_state = AppState::ShowMagazine;
        *should_add_tool = false;
    }
}

pub fn modify_tool(tool: &mut Tool, ui: &mut egui::Ui, add: &mut bool) {
    tool.tool_edit(ui, add);
}
