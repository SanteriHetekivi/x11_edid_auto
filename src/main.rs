// Forbid unsafe code.
#![forbid(unsafe_code)]

// Errors.
mod errors;

// Connection struct.
mod connection;

// Monitor struct.
mod monitor;

// Configuration.
#[derive(Debug, serde::Deserialize)]
struct Config {
    monitor_groups: Vec<Vec<String>>,
}

fn run() -> Result<(), errors::X11EDIDAutoError> {
    // Get arguments.
    let args: Vec<String> = std::env::args().collect();

    // If had invalid arguments
    if args.len() != 2 {
        // return with error.
        return Err(errors::X11EDIDAutoError::InvalidArgumentsError(
            errors::InvalidArgumentsError::new(args[0].clone()),
        ));
    }

    // Get config file path from arguments.
    let config_file_path: &str = args[1].trim();

    // Generate config from file.
    let config: Config = toml::from_str(&std::fs::read_to_string(config_file_path)?)?;

    // If no monitor groups given
    if config.monitor_groups.is_empty() {
        // return with error.
        return Err(errors::X11EDIDAutoError::NoMonitorGroupsGivenError(
            errors::NoMonitorGroupsGivenError::new(config_file_path.to_string()),
        ));
    }

    // Get connection.
    let connection: connection::Connection = connection::Connection::new()?;

    // Create a map of monitor id to monitor.
    let mut monitor_map: std::collections::HashMap<String, monitor::Monitor> =
        std::collections::HashMap::new();

    // Outputs to monitors.
    println!("Getting monitors...");
    // Loop outputs.
    for output in connection.outputs()? {
        // Create monitor for output.
        let monitor: monitor::Monitor = monitor::Monitor::new(&connection, output)?;
        // If monitor has EDID
        if monitor.has_edid() {
            // add it to map.
            monitor_map.insert(monitor.monitor_id(), monitor);
        }
    }

    // If no monitors found
    if monitor_map.is_empty() {
        // return with error.
        return Err(errors::X11EDIDAutoError::NoMonitorsFoundError(
            errors::NoMonitorsFoundError::new(),
        ));
    }

    // Loop monitor groups.
    println!("Monitor groups...");
    let mut set: bool = false;
    let mut index = 0;
    // While not set and still has monitor groups.
    while index < config.monitor_groups.len() && !set {
        // Get monitor group.
        let monitor_group = &config.monitor_groups[index];

        // If all of the groups monitors are connected.
        if monitor_group
            .iter()
            .all(|monitor_edid| monitor_map.contains_key(monitor_edid))
        {
            println!(
                "{:?}. monitor group had all of it's monitors {:?} present!",
                index + 1,
                monitor_group
            );

            // Enable monitors in monitor group.
            let mut x: i16 = 0;
            for monitor_edid in monitor_group {
                let monitor: &monitor::Monitor = match monitor_map.get(monitor_edid) {
                    Some(monitor) => monitor,
                    None => {
                        return Err(errors::X11EDIDAutoError::MonitorNotFoundError(
                            errors::MonitorNotFoundError::new(monitor_edid.to_string()),
                        ))
                    }
                };
                monitor.enable(x)?;
                if x == 0 {
                    monitor.set_primary()?;
                }
                x += std::convert::TryInto::<i16>::try_into(monitor.mode_info()?.width).map_err(
                    |try_from_int_error: std::num::TryFromIntError| {
                        errors::TryIntoI16Error::new(
                            "Monitor width".to_string(),
                            try_from_int_error,
                        )
                    },
                )?;
            }

            // Disable monitors that are not in monitor group.
            println!("Disabling unused monitors...");
            for (monitor_edid, monitor) in &monitor_map {
                if !monitor_group.contains(&monitor_edid) {
                    monitor.disable()?;
                }
            }

            // Inform that monitor group was set so can stop looping.
            set = true;
        }
        // If monitor group did not have all of it's monitors present
        else {
            // inform user.
            eprintln!(
                "{:?}. monitor group did not have all of it's monitors {:?} present!",
                index + 1,
                monitor_group
            );
        }

        // Increment index.
        index += 1;
    }

    // If did set monitor group
    if set {
        // end connection.
        connection.end()?;
    // else if did not set any monitor group
    } else {
        // return error.
        return Err(
            errors::X11EDIDAutoError::NoMonitorGroupWithAllMonitorsPresentError(
                errors::NoMonitorGroupWithAllMonitorsPresentError::new(
                    monitor_map
                        .values()
                        .map(|monitor| monitor.monitor_info())
                        .collect(),
                ),
            ),
        );
    }
    // Done.
    println!("Done!");
    Ok(())
}

// Automatic monitor configuration based on EDID.
fn main() {
    match run() {
        Ok(()) => std::process::exit(0),
        Err(error) => {
            eprintln!("Got error {}", error);
            std::process::exit(1);
        }
    }
}
