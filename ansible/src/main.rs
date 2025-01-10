use anyhow::{anyhow, Context};
use fcc::{compliance::check_compliance, FlatConfig, FlatConfigCompliance};
use serde::{Deserialize, Serialize};
use std::{env, fs, process, str::FromStr};

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
enum ModuleReturn {
    #[default]
    Failed,
    Changed,
}

#[derive(Deserialize)]
struct ModuleArgs {
    #[serde(default)]
    #[serde(rename = "return")]
    module_return: ModuleReturn,

    policy: String,
    configuration: String,
}

#[derive(Clone, Serialize, Default)]
struct Response {
    msg: String,
    results: Vec<String>,
    changed: bool,
    failed: bool,
}

fn main() {
    let (response, code) = match run_module() {
        Ok(response) => (response, 0),
        Err(err) => (
            Response {
                msg: format!("{}", err),
                changed: false,
                failed: true,
                ..Default::default()
            },
            1,
        ),
    };
    println!("{}", serde_json::to_string(&response).unwrap());
    process::exit(code);
}

fn run_module() -> anyhow::Result<Response> {
    let input_filename = env::args().nth(1).ok_or(anyhow!(
        "module '{}' expects exactly one argument!",
        env::args().next().unwrap()
    ))?;
    let json_input = fs::read_to_string(&input_filename)
        .with_context(|| format!("Could not read file '{}'", input_filename))?;
    let module_args: ModuleArgs = serde_json::from_str(&json_input)?;

    let config = FlatConfig::from_str(&module_args.configuration)?;
    let policy = FlatConfigCompliance::from_str(&module_args.policy)?;

    let results = check_compliance(policy, config);
    let mut compliance_failed = false;
    Ok(Response {
        msg: String::new(),
        results: results
            .iter()
            .map(|f| {
                if f.result.is_ok() {
                    format!("OK-{}", f)
                } else {
                    compliance_failed = true;
                    format!("ERR-{}", f)
                }
            })
            .collect(),
        changed: matches!(module_args.module_return, ModuleReturn::Changed) && compliance_failed,
        failed: matches!(module_args.module_return, ModuleReturn::Failed) && compliance_failed,
    })
}
