use std::cmp::Ordering;

use crate::adapter::*;
use crate::holder::*;

use crate::tool::*;
use crate::ManagingApp;
use crate::MoveStates;
use crate::ToolState;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Magazine {
    pub name: String,
    pub contents: Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)>,
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
                    app.display_magazine = magazine.clone();
                }
            }
        });
}

pub fn get_sorted_by_slot(
    contents: &mut Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)>,
    filter: Option<ToolState>,
) -> Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)> {
    let mut filtered = match filter {
        Some(ToolState::Rotating) => {
            get_filtered_by_tool_category(contents, ToolCategory::Rotating)
        }
        Some(ToolState::Insert) => {
            get_filtered_by_tool_category(contents, ToolCategory::LatheInsert)
        }
        _ => contents.clone(),
    };

    let mut sorted: Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)> = filtered
        .iter_mut()
        .map(|(i, tool, holder, adapter)| (*i, tool.clone(), holder.clone(), adapter.clone()))
        .collect();

    sorted.sort_by(|(i_a, ..), (i_b, ..)| i_a.partial_cmp(i_b).unwrap_or(Ordering::Equal));
    sorted
}

pub fn get_sorted_by_tool_diameter(
    contents: &mut Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)>,
) -> Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)> {
    let mut filtered: Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)> = contents
        .iter_mut()
        .filter(|(_u32, tool, _holder, _adapter)| {
            tool.clone()
                .map(|t| t.get_category() == ToolCategory::Rotating)
                .unwrap_or(false)
        })
        .map(|(i, tool, holder, adapter)| (*i, tool.clone(), holder.clone(), adapter.clone()))
        .collect();

    filtered.sort_by(|(_i_a, tool_a, ..), (_i_b, tool_b, ..)| {
        tool_a
            .clone()
            .unwrap()
            .get_diameter()
            .partial_cmp(&tool_b.clone().unwrap().get_diameter())
            .unwrap_or(Ordering::Equal)
    });
    filtered
}

pub fn get_sorted_by_degree(
    contents: &mut Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)>,
) -> Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)> {
    let mut filtered: Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)> = contents
        .iter_mut()
        .filter(|(_u32, tool, _holder, _adapter)| {
            tool.clone()
                .map(|t| t.get_category() == ToolCategory::LatheInsert)
                .unwrap_or(false)
        })
        .map(|(i, tool, holder, adapter)| (*i, tool.clone(), holder.clone(), adapter.clone()))
        .collect();

    filtered.sort_by(|(_i_a, tool_a, ..), (_i_b, tool_b, ..)| {
        tool_a
            .clone()
            .unwrap()
            .get_degree()
            .partial_cmp(&tool_b.clone().unwrap().get_degree())
            .unwrap_or(Ordering::Equal)
    });
    filtered
}

pub fn get_filtered_by_tool_category(
    contents: &mut Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)>,
    category: ToolCategory,
) -> Vec<(usize, Option<Tool>, Option<Holder>, Option<Adapter>)> {
    contents
        .iter_mut()
        .filter(|(_usize, tool, _holder, _adapter)| {
            tool.clone()
                .map(|t| t.get_category() == category)
                .unwrap_or(false)
        })
        .map(|(i, tool, holder, adapter)| (*i, tool.clone(), holder.clone(), adapter.clone()))
        .collect()
}

pub fn display_magazine(app: &mut ManagingApp, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.label(app.display_magazine.name.clone());
    for (_i, (index, tool, holder, adapter)) in app.display_magazine.contents.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.label(format!("Slot {}", index));

            ui.separator();
            if ui.button("Add").clicked() {
                app.move_selections.selected_tool_index_magazine = Some(*index);
                app.app_states.move_state = Some(MoveStates::ToolToMagazine);
            }
            if ui.button("Remove").clicked() {
                app.move_selections.selected_tool_index_magazine = Some(*index);
                app.app_states.move_state = Some(MoveStates::ToolToLibrary);
            }
            if let Some(tool) = tool {
                tool.display(ui);
            } else {
                ui.label("Empty");
            }

            ui.separator();
            if ui.button("Add").clicked() {
                app.move_selections.selected_holder_index_magazine = Some(*index);
                app.app_states.move_state = Some(MoveStates::HolderToMagazine);
            }
            if ui.button("Remove").clicked() {
                app.move_selections.selected_holder_index_magazine = Some(*index);
                app.app_states.move_state = Some(MoveStates::HolderToLibrary);
            }
            if let Some(holder) = holder {
                holder.display(ui);
            } else {
                ui.label("Empty");
            }

            ui.separator();
            if ui.button("Add").clicked() {
                app.move_selections.selected_adapter_index_magazine = Some(*index);
                app.app_states.move_state = Some(MoveStates::AdapterToMagazine);
            }
            if ui.button("Remove").clicked() {
                app.move_selections.selected_adapter_index_magazine = Some(*index);
                app.app_states.move_state = Some(MoveStates::AdapterToLibrary);
            }
            if let Some(adapter) = adapter {
                adapter.display(ui);
            } else {
                ui.label("Empty");
            }
        });
    }
}
