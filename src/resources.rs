use std::fmt;

use crate::{
    adapter::Adapter, collet::Collet, drill::Drill, holder::Holder, hydraulic::Hydraulic,
    mill::Mill, tool::Tool, trigoninsert::TrigonInsert, Machine,
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GuiSingletons {
    pub rotating_tools: Vec<Tool>,
    pub insert_tools: Vec<Tool>,
    pub holders: Vec<Holder>,
    pub adapters: Vec<Adapter>,
    pub machine: Machine,

    pub tool_filter: Option<ToolState>,
    pub sort_by: SortBy,
}

impl Default for GuiSingletons {
    fn default() -> GuiSingletons {
        // Rotating tools
        let drill = Drill {
            name: "Drill".to_string(),
            diameter: 10.0,
        };
        let mill = Mill {
            name: "Mill".to_string(),
            diameter: 10.0,
        };
        // Insert tools
        let trigon_insert = TrigonInsert {
            name: "TrigonInsert".to_string(),
            degree: 35.0,
        };
        // Holders
        let collet = Collet {
            name: "Collet".to_string(),
        };
        // Adapters
        let hydraulic = Hydraulic {
            name: "Hydraulic".to_string(),
        };
        let rotating_tools = vec![Tool::Drill(drill), Tool::Mill(mill)];
        let insert_tools = vec![Tool::TrigonInsert(trigon_insert)];
        let holders = vec![Holder::Collet(collet)];
        let adapters = vec![Adapter::Hydraulic(hydraulic)];
        let machine = Machine {
            name: "Machine".to_string(),
            number_of_magazines: 0,
            magazine_size: 0,
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
        }
    }
}
