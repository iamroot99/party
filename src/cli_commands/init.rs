//! `party init`

use std::{fs::File, io::Write, path::Path};

use anyhow::{bail, Context};

use crate::{
    cli::InitArgs,
    parser::toml_command::{Task, Tasks},
    party_command::make_default_commands,
    util::{CHECK, DEFAULT_PARTY_CONF},
};

/// Implementation of `party init
pub fn init(init_args: InitArgs) -> anyhow::Result<()> {
    let commands = make_default_commands();

    let tasks: Vec<Task> = commands.into_iter().map(|cmd| cmd.into()).collect();
    let tasks = Tasks { tasks };

    let file_path = init_args.file.unwrap_or(DEFAULT_PARTY_CONF.to_string());
    if Path::new(&file_path).exists() {
        bail!("{} already exists!", file_path);
    }

    let tasks_str = toml::to_string(&tasks).context("Failed to process default commands")?;
    let mut file = File::create(file_path).context("Failed to created configuration file")?;
    file.write_all(tasks_str.as_bytes())?;

    println!(
        "{} Initialisation complete. Check \"party.toml\". {}",
        CHECK, CHECK
    );
    Ok(())
}
