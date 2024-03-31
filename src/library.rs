use crate::adapter::*;

use crate::holder::*;

use crate::reset_states;
use crate::resources::*;
use crate::tool::*;
use crate::ManagingApp;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Library {
    pub category: MagazineContentType,
    pub tools: Vec<Tool>,
    pub holders: Vec<Holder>,
    pub adapters: Vec<Adapter>,
}

impl Library {
    pub fn display(&mut self, ctx: &egui::Context) -> bool {
        let mut is_window_open = true;
        egui::Window::new("Library")
            .open(&mut is_window_open)
            .vscroll(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.category, MagazineContentType::Tool, "Tool");
                    ui.radio_value(&mut self.category, MagazineContentType::Holder, "Holder");
                    ui.radio_value(&mut self.category, MagazineContentType::Adapter, "Adapter");
                });
                match self.category {
                    MagazineContentType::Tool => {
                        for (index, tool) in self.tools.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("Tool {}: ", index));
                                ui.separator();
                                tool.display(ui);
                            });
                        }
                    }
                    MagazineContentType::Holder => {
                        for (index, holder) in self.holders.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("Holder {}: ", index));
                                holder.display(ui);
                            });
                        }
                    }
                    MagazineContentType::Adapter => {
                        for (index, adapter) in self.adapters.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("Adapter {}: ", index));
                                adapter.display(ui);
                            });
                        }
                    }
                }
            });
        is_window_open
    }
}

pub fn move_tool_to_magazine(app: &mut ManagingApp) {
    let library_index = app.move_selections.selected_tool_index_library;
    let magazine_index = app.move_selections.selected_tool_index_magazine;

    let machine = &mut app.machines[app.selections.machine.unwrap()];
    let magazine = &mut machine.magazines[machine.current_magazine.unwrap()];
    // Return if any index is none, otherwise create a block scope to borrow the library and magazine

    if library_index.is_none() || magazine_index.is_none() {
        return;
    }

    {
        let library_index = library_index.unwrap();
        let magazine_index = magazine_index.unwrap();
        let library_tool = app.library.tools[library_index].clone();
        let tool_in_magazine = magazine.contents[magazine_index].1.clone();

        match tool_in_magazine {
            Some(tool) => {
                magazine.contents[magazine_index].1 = Some(library_tool);
                app.library.tools.remove(library_index);
                app.library.tools.push(tool);
            }
            None => {
                magazine.contents[magazine_index].1 = Some(library_tool);
                app.library.tools.remove(library_index);
            }
        }
        app.display_magazine = magazine.clone();
        reset_states(app);
    }
}

pub fn move_tool_to_library(app: &mut ManagingApp) {
    let magazine_index = app.move_selections.selected_tool_index_magazine;

    let machine = &mut app.machines[app.selections.machine.unwrap()];
    let magazine = &mut machine.magazines[machine.current_magazine.unwrap()];
    // Return if any index is none, otherwise create a block scope to borrow the library and magazine
    if magazine_index.is_none() {
        return;
    }

    {
        let magazine_index = magazine_index.unwrap();
        let magazine_tool = magazine.contents[magazine_index].1.clone();

        match magazine_tool {
            Some(tool) => {
                app.library.tools.push(tool);
                magazine.contents[magazine_index].1 = None;
            }
            None => {
                magazine.contents[magazine_index].1 = None;
            }
        }
        app.display_magazine = magazine.clone();
        reset_states(app);
    }
}

// Rewrite and verify sizes etc.
pub fn select_tool_from_library(app: &mut ManagingApp, ctx: &egui::Context) {
    if app.library.tools.is_empty() {
        egui::Window::new("Library").show(ctx, |ui| {
            ui.label("No tools in library");
            if ui.button("Close").clicked() {
                reset_states(app);
            }
        });
        return;
    }
    let mut is_window_open = true;
    egui::Window::new("Select tool from library")
        .open(&mut is_window_open)
        .show(ctx, |ui| {
            for (i, tool) in app.library.tools.iter().enumerate() {
                tool.display(ui);
                if ui.button(format!("Move")).clicked() {
                    app.move_selections.selected_tool_index_library = Some(i);
                }
            }
        });
    if !is_window_open {
        app.app_states.move_state = None;
    }
}
