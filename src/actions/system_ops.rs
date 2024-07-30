use winapi::um::winuser;
use std::process::Command;
use query_wmi::WMIConnection;

pub fn execute_system_command(command: &str, args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new(command)
        .args(args)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(format!("Command failed: {}", String::from_utf8_lossy(&output.stderr)).into())
    }
}

pub fn execute_wmi_query(connection: &WMIConnection, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let results: Vec<std::collections::HashMap<String, String>> = connection.raw_query(query)?;
    for result in results {
        println!("{:?}", result);
    }
    Ok(())
}

pub fn change_setting(setting: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    match setting {
        "background" => change_background(value),
        "power_plan" => change_power_plan(value),
        // Add more settings as needed
        _ => Err(format!("Unknown setting: {}", setting).into()),
    }
}

pub fn change_background(image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Implement using winapi to change the background
    // This is just a pseudo-code example
    unsafe {
        winuser::SystemParametersInfoA(
            winuser::SPI_SETDESKWALLPAPER,
            0,
            image_path.as_ptr() as *mut _,
            winuser::SPIF_UPDATEINIFILE | winuser::SPIF_SENDCHANGE,
        );
    }
    Ok(())
}

fn change_power_plan(plan: &str) -> Result<(), Box<dyn std::error::Error>> {
    execute_system_command("powercfg", &["/s", plan])?;
    Ok(())
}