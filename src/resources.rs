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
pub struct ActiveState {
    pub app_state: AppState,
    pub add_tool_state: Option<AddToolState>,
    pub add_holder_state: Option<AddHolderState>,
    pub add_adapter_state: Option<AddAdapterState>,
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
pub enum AddToolState {
    #[default]
    Rotating,
    Insert,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub enum AddHolderState {
    #[default]
    Standard,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub enum AddAdapterState {
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct GuiSingletons {
    pub rotating_tools: Vec<Tool>,
    pub insert_tools: Vec<Tool>,
    pub holders: Vec<Holder>,
    pub adapters: Vec<Adapter>,
    pub machine: Machine,
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
        }
    }
}
