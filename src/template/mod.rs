pub mod config_extractor;
pub mod writer;

#[derive(Debug, Clone)]
pub struct Processor {
    template: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateHeader {
    pub path: String,
    #[serde(default)]
    pub action: WriteAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WriteAction {
    CreateFile,

    Append(ExtendLocation),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExtendLocation {
    BeginOfFile,
    EndOfFile,
    After(String),
    Before(String),
}

pub struct Writer {
    header: TemplateHeader,
    rendered_template: String,
}
