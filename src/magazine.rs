use std::cmp::Ordering;

use crate::adapter::*;
use crate::holder::*;

use crate::comment::*;
use crate::reset_states;
use crate::tool::*;
use crate::ManagingApp;
use crate::MoveStates;
use crate::ToolState;
use egui_extras::*;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Magazine {
    pub name: String,
    pub contents: Vec<(
        usize,
        Option<Tool>,
        Option<Holder>,
        Option<Adapter>,
        Comment,
    )>,
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
    contents: &mut Vec<(
        usize,
        Option<Tool>,
        Option<Holder>,
        Option<Adapter>,
        Comment,
    )>,
    filter: Option<ToolState>,
) -> Vec<(
    usize,
    Option<Tool>,
    Option<Holder>,
    Option<Adapter>,
    Comment,
)> {
    let mut filtered = match filter {
        Some(ToolState::Rotating) => {
            get_filtered_by_tool_category(contents, ToolCategory::Rotating)
        }
        Some(ToolState::Insert) => {
            get_filtered_by_tool_category(contents, ToolCategory::LatheInsert)
        }
        _ => contents.clone(),
    };

    let mut sorted: Vec<(
        usize,
        Option<Tool>,
        Option<Holder>,
        Option<Adapter>,
        Comment,
    )> = filtered
        .iter_mut()
        .map(|(i, tool, holder, adapter, comment)| {
            (
                *i,
                tool.clone(),
                holder.clone(),
                adapter.clone(),
                comment.clone(),
            )
        })
        .collect();

    sorted.sort_by(|(i_a, ..), (i_b, ..)| i_a.partial_cmp(i_b).unwrap_or(Ordering::Equal));
    sorted
}

pub fn get_sorted_by_tool_diameter(
    contents: &mut Vec<(
        usize,
        Option<Tool>,
        Option<Holder>,
        Option<Adapter>,
        Comment,
    )>,
) -> Vec<(
    usize,
    Option<Tool>,
    Option<Holder>,
    Option<Adapter>,
    Comment,
)> {
    let mut filtered: Vec<(
        usize,
        Option<Tool>,
        Option<Holder>,
        Option<Adapter>,
        Comment,
    )> = contents
        .iter_mut()
        .filter(|(_u32, tool, _holder, _adapter, _comment)| {
            tool.clone()
                .map(|t| t.get_category() == ToolCategory::Rotating)
                .unwrap_or(false)
        })
        .map(|(i, tool, holder, adapter, comment)| {
            (
                *i,
                tool.clone(),
                holder.clone(),
                adapter.clone(),
                comment.clone(),
            )
        })
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
    contents: &mut Vec<(
        usize,
        Option<Tool>,
        Option<Holder>,
        Option<Adapter>,
        Comment,
    )>,
) -> Vec<(
    usize,
    Option<Tool>,
    Option<Holder>,
    Option<Adapter>,
    Comment,
)> {
    let mut filtered: Vec<(
        usize,
        Option<Tool>,
        Option<Holder>,
        Option<Adapter>,
        Comment,
    )> = contents
        .iter_mut()
        .filter(|(_u32, tool, _holder, _adapter, _comment)| {
            tool.clone()
                .map(|t| t.get_category() == ToolCategory::LatheInsert)
                .unwrap_or(false)
        })
        .map(|(i, tool, holder, adapter, comment)| {
            (
                *i,
                tool.clone(),
                holder.clone(),
                adapter.clone(),
                comment.clone(),
            )
        })
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
    contents: &mut Vec<(
        usize,
        Option<Tool>,
        Option<Holder>,
        Option<Adapter>,
        Comment,
    )>,
    category: ToolCategory,
) -> Vec<(
    usize,
    Option<Tool>,
    Option<Holder>,
    Option<Adapter>,
    Comment,
)> {
    contents
        .iter_mut()
        .filter(|(_usize, tool, _holder, _adapter, _comment)| {
            tool.clone()
                .map(|t| t.get_category() == category)
                .unwrap_or(false)
        })
        .map(|(i, tool, holder, adapter, comment)| {
            (
                *i,
                tool.clone(),
                holder.clone(),
                adapter.clone(),
                comment.clone(),
            )
        })
        .collect()
}

pub fn display_magazine(app: &mut ManagingApp, ui: &mut egui::Ui, _ctx: &egui::Context) {
    ui.label(app.display_magazine.name.clone());
    TableBuilder::new(ui)
        .columns(Column::auto().resizable(true).clip(false), 5)
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.heading("Slot");
            });
            header.col(|ui| {
                ui.heading("Tool");
            });
            header.col(|ui| {
                ui.heading("Holder");
            });
            header.col(|ui| {
                ui.heading("Adapter");
            });
            header.col(|ui| {
                ui.heading("Comment");
            });
        })
        .body(|mut body| {
            for (_i, (index, tool, holder, adapter, comment)) in
                app.display_magazine.contents.iter().enumerate()
            {
                body.row(30.0, |mut row| {
                    row.col(|ui| {
                        ui.label(format!("Slot {}", index));
                    });
                    row.col(|ui| {
                        ui.horizontal(|ui| {
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
                        });
                    });
                    row.col(|ui| {
                        ui.horizontal(|ui| {
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
                        });
                    });
                    row.col(|ui| {
                        ui.horizontal(|ui| {
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
                    });
                    row.col(|ui| {
                        ui.horizontal(|ui| {
                            ui.set_width(100.0);
                            if ui.button("Edit").clicked() {
                                app.move_selections.selected_comment_index_magazine = Some(*index);
                                app.app_states.move_state = Some(MoveStates::EditComment);
                            }
                            comment.display(ui);
                        });
                    });
                });
            }
        });
}

pub fn edit_comment(app: &mut ManagingApp, ctx: &egui::Context) {
    let magazine_index = app.move_selections.selected_comment_index_magazine;
    let machine = &mut app.machines[app.selections.machine.unwrap()];
    let magazine = &mut machine.magazines[machine.current_magazine.unwrap()];
    if magazine_index.is_none() {
        return;
    }
    let comment = &mut magazine.contents[magazine_index.unwrap()].4.comment;
    let mut edit_done = false;
    egui::Window::new("Edit comment").show(ctx, |ui| {
        ui.text_edit_multiline(comment);
        if ui.button("OK").clicked() {
            edit_done = true;
        }
    });
    if edit_done {
        app.display_magazine = magazine.clone();
        reset_states(app);
    }
}
