#[tracing::instrument]
pub fn get_input(name: &str) -> anyhow::Result<String> {

    tracing::info!("the info mint!");
    // let input = std::env::var(format!("INPUT_{}", name.to_uppercase()))?;

    Ok("".to_string())
}

pub fn get_input_warn(name: impl Into<String>, default: impl Into<String>) -> String {
    std::env::var(format!("INPUT_{}", name.into().to_uppercase())).unwrap_or(default.into())
}

pub fn log_warn_with_params(msg: impl Into<String>, parameters: Vec<CommandParameter>) {
    println!(
        "{}",
        GhaCommand {
            cmd_type: CommandType::Msg {
                msg: msg.into(),
                msg_type: MsgType::Warning { parameters },
            },
        }
        .build()
    )
}

#[derive(Debug, Clone)]
pub struct GhaCommand {
    pub cmd_type: CommandType,
}

impl GhaCommand {
    pub fn build(&self) -> String {
        let command = match self.cmd_type.clone() {
            CommandType::Msg { msg, msg_type } => match msg_type {
                MsgType::Debug => format!("::debug::{}", msg),
                MsgType::Notice { parameters } => {
                    let encoded_parameters = parameters
                        .iter()
                        .map(|param| param.encode())
                        .collect::<Vec<_>>()
                        .join(",");

                    format!("::notice {encoded_parameters}::{msg}")
                }
                MsgType::Warning { parameters } => {
                    let encoded_parameters = parameters
                        .iter()
                        .map(|param| param.encode())
                        .collect::<Vec<_>>()
                        .join(",");

                    format!("::warning {encoded_parameters}::{msg}")
                }
                MsgType::Error { parameters } => {
                    let encoded_parameters = parameters
                        .iter()
                        .map(|param| param.encode())
                        .collect::<Vec<_>>()
                        .join(",");

                    format!("::error {encoded_parameters}::{msg}")
                }
            },
            CommandType::StartGroup(_) => todo!(),
            CommandType::EndGroup => todo!(),
            CommandType::AddMask(_) => todo!(),
        };
        command
    }
}

#[derive(Debug, Clone)]
pub enum MsgType {
    Debug,
    Notice { parameters: Vec<CommandParameter> },
    Warning { parameters: Vec<CommandParameter> },
    Error { parameters: Vec<CommandParameter> },
}

#[derive(Debug, Clone)]
pub enum CommandType {
    Msg { msg: String, msg_type: MsgType },
    StartGroup(String),
    EndGroup,
    AddMask(String),
}

#[derive(Debug, Clone)]
pub enum CommandParameter {
    /// Custom title
    Title(String),
    /// Name of file.
    File(String),
    /// Starting column number.
    Col(u64),
    /// End column number.
    EndCol(u64),
    /// Starting line number.
    Line(u64),
    /// End line number.
    EndLine(u64),
}

impl CommandParameter {
    pub fn encode(&self) -> String {
        match self {
            CommandParameter::Title(value) => format!("title={value}"),
            CommandParameter::File(value) => format!("file={value}"),
            CommandParameter::Col(num) => format!("col={num}"),
            CommandParameter::EndCol(num) => format!("endColumn={num}"),
            CommandParameter::Line(num) => format!("line={num}"),
            CommandParameter::EndLine(num) => format!("endLine={num}"),
        }
    }
}
