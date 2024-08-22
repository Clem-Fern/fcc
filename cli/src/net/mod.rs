use serde::Serialize;

#[derive(clap::ValueEnum, Clone, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GetConfigProtocol {
    SshExec,
    RestConf,
    NetConf,
}
