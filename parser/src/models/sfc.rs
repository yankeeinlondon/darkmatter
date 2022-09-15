use thiserror::Error;

#[derive(Error, Debug)]
pub enum SfcError {}

pub struct Sfc {
    setup_script: String,
    script_blocks: Vec<String>,
    template: String,
    style_blocks: Vec<String>,
}
