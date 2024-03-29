use std::cmp::Ordering;

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
                    app.display_magazine = magazine.clone();
                }
            }
        });
}

pub fn get_sorted_by_tool_diameter(
    contents: &mut Vec<(u32, Option<Tool>, Option<Holder>, Option<Adapter>)>,
) -> Vec<(u32, Option<Tool>, Option<Holder>, Option<Adapter>)> {
    let mut filtered: Vec<(u32, Option<Tool>, Option<Holder>, Option<Adapter>)> = contents
        .iter_mut()
        .filter(|(u32, tool, holder, adapter)| {
            tool.clone()
                .map(|t| t.get_category() == ToolCategory::Rotating)
                .unwrap_or(false)
        })
        .map(|(i, tool, holder, adapter)| (*i, tool.clone(), holder.clone(), adapter.clone()))
        .collect();

    filtered.sort_by(|(i_a, tool_a, ..), (i_b, tool_b, ..)| {
        tool_a
            .clone()
            .unwrap()
            .get_diameter()
            .partial_cmp(&tool_b.clone().unwrap().get_diameter())
            .unwrap_or(Ordering::Equal)
    });
    filtered
}

pub fn display_magazine(magazine: &Magazine, ui: &mut egui::Ui) {
    ui.label(magazine.name.clone());
    for (i, (index, tool, holder, adapter)) in magazine.contents.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.label(format!("Slot {}", index));
            if let Some(tool) = tool {
                tool.display(ui);
            } else {
                ui.label("Empty");
            }
            if let Some(holder) = holder {
                holder.display(ui);
            } else {
                ui.label("Empty");
            }
            if let Some(adapter) = adapter {
                adapter.display(ui);
            } else {
                ui.label("Empty");
            }
        });
    }
}
