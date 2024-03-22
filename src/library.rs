use crate::adapter::*;
use crate::holder::*;

use crate::resources::*;
use crate::tool::*;

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
