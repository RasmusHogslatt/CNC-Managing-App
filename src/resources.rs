use std::fmt;

use egui::Color32;

use strum::{Display, EnumIter, EnumString};

use crate::{
    adapter::Adapter, calculations::calculations::*, collet::Collet, drill::Drill, holder::Holder,
    hydraulic::Hydraulic, mill::Mill, tool::Tool, trigoninsert::TrigonInsert, Machine,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Selections {
    pub machine: Option<usize>,

    pub selected_rotating_tool_index: usize,
    pub selected_insert_tool_index: usize,
    pub selected_holder_index: usize,
    pub selected_adapter_index: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct MagazineLibraryMovingSelections {
    pub selected_tool_index_magazine: Option<usize>,
    pub selected_holder_index_magazine: Option<usize>,
    pub selected_adapter_index_magazine: Option<usize>,
    pub selected_comment_index_magazine: Option<usize>,

    pub selected_tool_index_library: Option<usize>,
    pub selected_holder_index_library: Option<usize>,
    pub selected_adapter_index_library: Option<usize>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct ActiveState {
    pub app_state: AppState,
    pub move_state: Option<MoveStates>,
    pub add_tool_state: Option<ToolState>,
    pub add_holder_state: Option<HolderState>,
    pub add_adapter_state: Option<AdapterState>,
    pub magazine_content_state: Option<MagazineContentType>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub enum AppState {
    #[default]
    ShowMagazine,
    AddTool,
    AddHolder,
    AddAdapter,
    AddMachine,
    ShowLibrary,
    Settings,
    Calculations,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub enum MoveStates {
    #[default]
    ToolToMagazine,
    ToolToLibrary,
    HolderToMagazine,
    HolderToLibrary,
    AdapterToMagazine,
    AdapterToLibrary,
    EditComment,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum ToolState {
    #[default]
    Rotating,
    Insert,
}

impl fmt::Display for ToolState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToolState::Rotating => write!(f, "Rotating"),
            ToolState::Insert => write!(f, "Insert"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum HolderState {
    #[default]
    Standard,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum AdapterState {
    #[default]
    Standard,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum MagazineContentType {
    #[default]
    Tool,
    Holder,
    Adapter,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum SortBy {
    #[default]
    Slot,
    Diameter,
    Degree,
}

impl fmt::Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortBy::Slot => write!(f, "Slot"),
            SortBy::Diameter => write!(f, "Diameter"),
            SortBy::Degree => write!(f, "Degree"),
        }
    }
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    EnumIter,
    EnumString,
    Display,
)]
pub enum ColorSettingsState {
    #[default]
    Rotating,
    Insert,
    Holder,
    Adapter,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct ColorSettings {
    pub index: Option<usize>,
    pub color: Color32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GuiSingletons {
    pub rotating_tools: Vec<Tool>,
    pub insert_tools: Vec<Tool>,
    pub holders: Vec<Holder>,
    pub adapters: Vec<Adapter>,
    pub machine: Machine,

    pub tool_filter: Option<ToolState>,
    pub sort_by: SortBy,

    pub color_settings_state: ColorSettingsState,
    pub color_settings: ColorSettings,

    pub universal_calculations: UniversalCalculations,
}

impl Default for GuiSingletons {
    fn default() -> GuiSingletons {
        // Rotating tools
        let drill = Drill {
            name: "Drill".to_string(),
            diameter: 10.0,
            color: Color32::RED,
        };
        let mill = Mill {
            name: "Mill".to_string(),
            diameter: 10.0,
            color: Color32::BLUE,
        };
        // Insert tools
        let trigon_insert = TrigonInsert {
            name: "TrigonInsert".to_string(),
            degree: 35.0,
            color: Color32::GREEN,
        };
        // Holders
        let collet = Collet {
            name: "Collet".to_string(),
            color: Color32::LIGHT_BLUE,
        };
        // Adapters
        let hydraulic = Hydraulic {
            name: "Hydraulic".to_string(),
            color: Color32::GREEN,
        };
        let mut rotating_tools = Vec::<Tool>::new();
        rotating_tools.push(Tool::Drill(drill.clone()));
        rotating_tools.push(Tool::Mill(mill.clone())); // vec![Tool::Drill(drill), Tool::Mill(mill)];
        let insert_tools = vec![Tool::TrigonInsert(trigon_insert)];
        let holders = vec![Holder::Collet(collet)];
        let adapters = vec![Adapter::Hydraulic(hydraulic)];
        let machine = Machine {
            name: "Machine".to_string(),
            number_of_magazines: 1,
            magazine_size: 1,
            magazines: vec![],
            current_magazine: None,
        };
        GuiSingletons {
            rotating_tools,
            insert_tools,
            holders,
            adapters,
            machine,
            tool_filter: None,
            sort_by: SortBy::Slot,
            color_settings_state: ColorSettingsState::Rotating,
            color_settings: ColorSettings {
                index: None,
                color: Color32::RED,
            },
            universal_calculations: UniversalCalculations::default(),
        }
    }
}
