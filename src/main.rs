use eframe::egui;
use egui_node_graph::{NodeDataTrait, DataTypeTrait, NodeResponse, NodeTemplateTrait, 
    WidgetValueTrait, Graph, GraphEditorState, NodeId, NodeTemplateIter, InputParamKind};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct UserState;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum DataType {
    String,
    Number,
    Boolean,
}

impl DataTypeTrait<UserState> for DataType {
    fn data_type_color(&self, _user_state: &mut UserState) -> egui::epaint::Color32 {
        match self {
            DataType::String => egui::epaint::Color32::from_rgb(28, 144, 243),
            DataType::Number => egui::epaint::Color32::from_rgb(243, 149, 28),
            DataType::Boolean => egui::epaint::Color32::from_rgb(149, 243, 28),
        }
    }

    fn name(&self) -> Cow<'static, str> {
        match self {
            DataType::String => "String".into(),
            DataType::Number => "Number".into(),
            DataType::Boolean => "Boolean".into(),
        }
    }
}

#[derive(Clone, Debug)]
enum ValueType {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl Default for ValueType {
    fn default() -> Self {
        ValueType::String(String::new())
    }
}

#[derive(Clone, Debug)]
enum UserResponse {
    None,
}

impl egui_node_graph::UserResponseTrait for UserResponse {}

#[derive(Clone, Debug)]
struct NodeData {
    node_type: NodeType,
    values: HashMap<String, ValueType>,
}

impl NodeDataTrait for NodeData {
    type Response = UserResponse;
    type UserState = UserState;
    type DataType = DataType;
    type ValueType = ValueType;

    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, Self>> {
        match &self.node_type {
            NodeType::Variable => {
                if let Some(ValueType::String(name)) = self.values.get("name") {
                    ui.label(name);
                }
            }
            NodeType::FunctionCall => {
                if let Some(ValueType::String(name)) = self.values.get("function_name") {
                    ui.label(name);
                }
            }
            _ => {}
        }
        vec![]
    }
}

impl WidgetValueTrait for ValueType {
    type Response = UserResponse;
    type UserState = UserState;
    type NodeData = NodeData;

    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        match self {
            ValueType::String(s) => {
                if ui.text_edit_singleline(s).changed() {
                    vec![UserResponse::None]
                } else {
                    vec![]
                }
            }
            ValueType::Number(n) => {
                if ui.add(egui::DragValue::new(n)).changed() {
                    vec![UserResponse::None]
                } else {
                    vec![]
                }
            }
            ValueType::Boolean(b) => {
                if ui.checkbox(b, param_name).changed() {
                    vec![UserResponse::None]
                } else {
                    vec![]
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
enum NodeType {
    MainFunction,
    Variable,
    FunctionCall,
    StringLiteral,
    NumberLiteral,
    BooleanLiteral,
    Print,
}

#[derive(Clone, Debug)]
struct NodeTemplate {
    node_type: NodeType,
    name: String,
}

impl Default for NodeTemplate {
    fn default() -> Self {
        Self {
            node_type: NodeType::MainFunction,
            name: "Main Function".into(),
        }
    }
}

impl NodeTemplateTrait for NodeTemplate {
    type NodeData = NodeData;
    type DataType = DataType;
    type ValueType = ValueType;
    type UserState = UserState;
    
    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        self.name.as_str().into()
    }

    fn node_graph_label(&self, _user_state: &mut Self::UserState) -> String {
        self.name.clone()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        NodeData {
            node_type: self.node_type.clone(),
            values: HashMap::new(),
        }
    }

    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        match &graph.nodes[node_id].user_data.node_type {
            NodeType::MainFunction => {
                graph.add_output_param(
                    node_id,
                    "output".to_string(),
                    DataType::String,
                );
            },
            NodeType::Variable => {
                graph.add_input_param(
                    node_id,
                    "Value".to_string(),
                    DataType::String,
                    ValueType::String(String::new()),
                    InputParamKind::ConnectionOnly,
                    true,
                );
                graph.add_output_param(
                    node_id,
                    "output".to_string(),
                    DataType::String,
                );
            },
            NodeType::FunctionCall => {
                graph.add_input_param(
                    node_id,
                    "Arguments".to_string(),
                    DataType::String,
                    ValueType::String(String::new()),
                    InputParamKind::ConnectionOnly,
                    true,
                );
                graph.add_output_param(
                    node_id,
                    "output".to_string(),
                    DataType::String,
                );
            },
            _ => {}
        }
    }
}

#[derive(Default)]
pub struct NodeContainer {
    pub node_templates: Vec<NodeTemplate>,
}

impl NodeTemplateIter for NodeContainer {
    type Item = NodeTemplate;
    
    fn all_kinds(&self) -> Vec<Self::Item> {
        vec![
            NodeTemplate {
                node_type: NodeType::MainFunction,
                name: "Main Function".into(),
            },
            NodeTemplate {
                node_type: NodeType::Variable,
                name: "Variable".into(),
            },
            NodeTemplate {
                node_type: NodeType::FunctionCall,
                name: "Function Call".into(),
            },
            NodeTemplate {
                node_type: NodeType::StringLiteral,
                name: "String Literal".into(),
            },
            NodeTemplate {
                node_type: NodeType::NumberLiteral,
                name: "Number Literal".into(),
            },
            NodeTemplate {
                node_type: NodeType::BooleanLiteral,
                name: "Boolean Literal".into(),
            },
            NodeTemplate {
                node_type: NodeType::Print,
                name: "Print".into(),
            },
        ]
    }
}

#[derive(Default)]
pub struct MyEditorState {
    graph: Graph<NodeData, DataType, ValueType>,
    graph_state: GraphEditorState<NodeData, DataType, ValueType, NodeContainer, UserState>,
    node_container: NodeContainer,
}

pub struct NodeCodegenApp {
    state: MyEditorState,
    user_state: UserState,
}

impl NodeCodegenApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: MyEditorState::default(),
            user_state: UserState::default(),
        }
    }
}

impl eframe::App for NodeCodegenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.state.graph_state.draw_graph_editor(
                ui,
                &mut self.state.graph,
                &mut self.user_state,
                &self.state.node_container,
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Code Generator",
        native_options,
        Box::new(|cc| Ok(Box::new(NodeCodegenApp::new(cc)))),
    )
}