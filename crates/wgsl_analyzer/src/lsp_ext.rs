use lsp_types::{request::Request, TextDocumentIdentifier, TextDocumentPositionParams};
use serde::{Deserialize, Serialize};

pub enum SyntaxTree {}

impl Request for SyntaxTree {
    type Params = SyntaxTreeParams;
    type Result = String;
    const METHOD: &'static str = "wgsl-analyzer/syntaxTree";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxTreeParams {
    pub text_document: TextDocumentIdentifier,
}

pub enum DebugCommand {}

impl Request for DebugCommand {
    type Params = DebugCommandParams;
    type Result = ();
    const METHOD: &'static str = "wgsl-analyzer/debugCommand";
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DebugCommandParams {
    #[serde(flatten)]
    pub position: TextDocumentPositionParams,
}
