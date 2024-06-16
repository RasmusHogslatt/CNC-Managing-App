crate cnc
├── mod adapters: pub(crate)
│   ├── mod adapter: pub
│   │   ├── enum Adapter: pub
│   │   │   ├── fn adapter_edit: pub
│   │   │   ├── fn display: pub
│   │   │   ├── fn get_category: pub
│   │   │   ├── fn get_color: pub
│   │   │   ├── fn get_name: pub
│   │   │   ├── fn get_type: pub
│   │   │   └── fn set_color: pub
│   │   ├── enum AdapterCategory: pub
│   │   ├── fn add_adapter: pub
│   │   ├── fn add_standard_adapter: pub
│   │   └── fn modify_adapter: pub
│   └── mod hydraulic: pub
│       └── struct Hydraulic: pub
│           ├── fn display: pub
│           ├── fn get_category: pub
│           ├── fn get_color: pub
│           ├── fn get_name: pub
│           ├── fn get_type: pub
│           ├── fn hydraulic_edit: pub
│           └── fn set_color: pub
├── mod app: pub(crate)
│   ├── struct ManagingApp: pub
│   │   ├── fn new: pub
│   │   ├── fn save: pub(self)
│   │   └── fn update: pub(self)
│   ├── fn apply_colors: pub
│   ├── fn apply_colors_to_contents: pub
│   ├── fn filter_by_tool_category: pub
│   ├── fn log_states: pub
│   ├── fn reset_states: pub
│   ├── fn settings: pub
│   └── fn sort_by: pub
├── mod comment: pub(crate)
│   └── struct Comment: pub
│       ├── fn default: pub
│       └── fn display: pub
├── mod holders: pub(crate)
│   ├── mod collet: pub
│   │   └── struct Collet: pub
│   │       ├── fn display: pub
│   │       ├── fn get_category: pub
│   │       ├── fn get_color: pub
│   │       ├── fn get_name: pub
│   │       ├── fn get_type: pub
│   │       ├── fn holder_edit: pub
│   │       └── fn set_color: pub
│   └── mod holder: pub
│       ├── enum Holder: pub
│       │   ├── fn display: pub
│       │   ├── fn get_category: pub
│       │   ├── fn get_color: pub
│       │   ├── fn get_name: pub
│       │   ├── fn get_type: pub
│       │   ├── fn holder_edit: pub
│       │   └── fn set_color: pub
│       ├── enum HolderCategory: pub
│       ├── fn add_holder: pub
│       ├── fn add_standard_holder: pub
│       └── fn modify_holder: pub
├── mod library: pub(crate)
│   ├── struct Library: pub
│   │   └── fn display: pub
│   ├── fn move_adapter_to_library: pub
│   ├── fn move_adapter_to_magazine: pub
│   ├── fn move_holder_to_library: pub
│   ├── fn move_holder_to_magazine: pub
│   ├── fn move_tool_to_library: pub
│   ├── fn move_tool_to_magazine: pub
│   ├── fn select_adapter_from_library: pub
│   ├── fn select_holder_from_library: pub
│   └── fn select_tool_from_library: pub
├── mod machine: pub(crate)
│   ├── struct Machine: pub
│   ├── fn add_machine: pub
│   └── fn select_machine: pub
├── mod magazine: pub(crate)
│   ├── struct Magazine: pub
│   ├── fn display_magazine: pub
│   ├── fn edit_comment: pub
│   ├── fn get_filtered_by_tool_category: pub
│   ├── fn get_sorted_by_degree: pub
│   ├── fn get_sorted_by_slot: pub
│   ├── fn get_sorted_by_tool_diameter: pub
│   └── fn select_magazine: pub
├── mod resources: pub(crate)
│   ├── struct ActiveState: pub
│   ├── enum AdapterState: pub
│   ├── enum AppState: pub
│   ├── struct ColorSettings: pub
│   ├── enum ColorSettingsState: pub
│   ├── struct GuiSingletons: pub
│   ├── enum HolderState: pub
│   ├── enum MagazineContentType: pub
│   ├── struct MagazineLibraryMovingSelections: pub
│   ├── enum MoveStates: pub
│   ├── struct Selections: pub
│   ├── enum SortBy: pub
│   └── enum ToolState: pub
└── mod tools: pub(crate)
    ├── mod drill: pub
    │   └── struct Drill: pub
    │       ├── fn display: pub
    │       ├── fn get_category: pub
    │       ├── fn get_color: pub
    │       ├── fn get_diameter: pub
    │       ├── fn get_name: pub
    │       ├── fn get_type: pub
    │       ├── fn set_color: pub
    │       └── fn tool_edit: pub
    ├── mod mill: pub
    │   └── struct Mill: pub
    │       ├── fn display: pub
    │       ├── fn get_category: pub
    │       ├── fn get_color: pub
    │       ├── fn get_diameter: pub
    │       ├── fn get_name: pub
    │       ├── fn get_type: pub
    │       ├── fn set_color: pub
    │       └── fn tool_edit: pub
    ├── mod tool: pub
    │   ├── enum Tool: pub
    │   │   ├── fn display: pub
    │   │   ├── fn get_category: pub
    │   │   ├── fn get_color: pub
    │   │   ├── fn get_degree: pub
    │   │   ├── fn get_diameter: pub
    │   │   ├── fn get_name: pub
    │   │   ├── fn get_type: pub
    │   │   ├── fn set_color: pub
    │   │   └── fn tool_edit: pub
    │   ├── enum ToolCategory: pub
    │   ├── fn add_insert_tool: pub
    │   ├── fn add_rotating_tool: pub
    │   ├── fn add_tool: pub
    │   └── fn modify_tool: pub
    └── mod trigoninsert: pub
        └── struct TrigonInsert: pub
            ├── fn display: pub
            ├── fn get_category: pub
            ├── fn get_degree: pub
            ├── fn get_name: pub
            ├── fn get_type: pub
            ├── fn set_color: pub
            └── fn tool_edit: pub