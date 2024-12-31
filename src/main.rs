use std::borrow::Cow;
use eframe::egui;
use eframe::epaint::Color32;
use egui_node_graph::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RustDataType {
    String,
    Number,
}

#[derive(Clone, Debug)]
pub enum RustValueType {
    String(String),
    Number(f64),
}

impl Default for RustValueType {
    fn default() -> Self {
        Self::String(String::new())
    }
}

#[derive(Clone, Copy)]
pub enum RustNodeTemplate {
    Function,
    Value,
}

#[derive(Clone)]
pub struct RustNodeData {
    template: RustNodeTemplate,
}

impl RustNodeData {
    pub fn template(&self) -> RustNodeTemplate {
        self.template
    }
}

#[derive(Clone, Debug)]
pub enum RustResponse {
    None,
}

impl UserResponseTrait for RustResponse {}

#[derive(Default)]
pub struct RustGraphState;

impl DataTypeTrait<RustGraphState> for RustDataType {
    fn data_type_color(&self, _user_state: &mut RustGraphState) -> Color32 {
        // Just use a default color, no custom colors
        Default::default()
    }

    fn name(&self) -> Cow<'_, str> {
        match self {
            RustDataType::String => Cow::Borrowed("string"),
            RustDataType::Number => Cow::Borrowed("number"),
        }
    }
}

impl NodeTemplateTrait for RustNodeTemplate {
    type NodeData = RustNodeData;
    type DataType = RustDataType;
    type ValueType = RustValueType;
    type UserState = RustGraphState;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        match self {
            RustNodeTemplate::Function => Cow::Borrowed("Function"),
            RustNodeTemplate::Value => Cow::Borrowed("Value"),
        }
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        RustNodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut Graph<RustNodeData, RustDataType, RustValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        match self {
            RustNodeTemplate::Function => {
                graph.add_input_param(
                    node_id,
                    "input".into(),
                    RustDataType::String,
                    RustValueType::String(String::new()),
                    InputParamKind::ConnectionOrConstant,
                    true,
                );
                graph.add_output_param(node_id, "output".into(), RustDataType::String);
            }
            RustNodeTemplate::Value => {
                graph.add_output_param(node_id, "value".into(), RustDataType::String);
            }
        }
    }
}

impl NodeDataTrait for RustNodeData {
    type Response = RustResponse;
    type UserState = RustGraphState;
    type DataType = RustDataType;
    type ValueType = RustValueType;

    fn bottom_ui(
        &self,
        _ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<RustNodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<Self::Response, RustNodeData>> {
        vec![]
    }
}

impl WidgetValueTrait for RustValueType {
    type Response = RustResponse;
    type UserState = RustGraphState;
    type NodeData = RustNodeData;

    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut Self::UserState,
        _node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        match self {
            RustValueType::String(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.text_edit_singleline(value);
                });
            }
            RustValueType::Number(value) => {
                ui.horizontal(|ui| {
                    ui.label(param_name);
                    ui.add(egui::DragValue::new(value));
                });
            }
        }
        vec![]
    }
}

pub struct RustNodeTemplates;
impl NodeTemplateIter for RustNodeTemplates {
    type Item = RustNodeTemplate;
    
    fn all_kinds(&self) -> Vec<Self::Item> {
        vec![RustNodeTemplate::Function, RustNodeTemplate::Value]
    }
}

pub struct RustNodeEditor {
    state: GraphEditorState<RustNodeData, RustDataType, RustValueType, RustNodeTemplate, RustGraphState>,
    user_state: RustGraphState,
}

impl Default for RustNodeEditor {
    fn default() -> Self {
        Self {
            state: GraphEditorState::default(),
            user_state: RustGraphState::default(),
        }
    }
}

impl eframe::App for RustNodeEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let graph_response = self.state.draw_graph_editor(
                ui,
                RustNodeTemplates,
                &mut self.user_state,
            );

            for node_response in graph_response.node_responses {
                if let NodeResponse::User(response) = node_response {
                    match response {
                        RustResponse::None => {}
                    }
                }
            }
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions::default();
    Ok(eframe::run_native(
        "Rust Node Editor",
        options,
        Box::new(|_cc| Box::new(RustNodeEditor::default())),
    ))
}