mod cli;
mod config;
mod actions;
mod utils;
mod network;
use crate::network::connection::Connection;
use crate::config::Task;

fn execute_tasks(connection: &mut Connection, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
    for task in tasks {
        println!("Executing task: {}", task.name);
        match task.action.as_str() {
            "copy_file" => {
                let src = task.params.get("src").and_then(|v| v.as_str()).ok_or("Missing 'src' parameter")?;
                let dest = task.params.get("dest").and_then(|v| v.as_str()).ok_or("Missing 'dest' parameter")?;
                actions::file_ops::copy_file(src, dest)?;
            },
            "change_setting" => {
                let setting = task.params.get("setting").and_then(|v| v.as_str()).ok_or("Missing 'setting' parameter")?;
                let value = task.params.get("value").and_then(|v| v.as_str()).ok_or("Missing 'value' parameter")?;
                actions::system_ops::change_setting(setting, value)?;
            },
            "execute_command" => {
                let command = task.params.get("command").and_then(|v| v.as_str()).ok_or("Missing 'command' parameter")?;
                let args = task.params.get("args").and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
                    .unwrap_or_default();
                actions::system_ops::execute_system_command(command, &args)?;
            },
            // Add more action types as needed
            _ => return Err(format!("Unknown action: {}", task.action).into()),
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::parse_args();
    let config_data = config::load_config(&args.config_file)?;
    
    let targets = if let Some(target_file) = args.target_file {
        utils::load_targets(&target_file)?
    } else {
        vec![format!("{}:{}", args.host.unwrap(), args.port.unwrap())]
    };

    for target in targets {
        let mut connection = network::connection::establish_connection(&target, &args.protocol)?;
        execute_tasks(&mut connection, &config_data.tasks)?;
    }

    Ok(())
}