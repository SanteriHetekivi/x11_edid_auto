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

// Automatic monitor configuration based on EDID.
fn main() {
    // Get arguments.
    let args: Vec<String> = std::env::args().collect();

    // If had invalid arguments
    if args.len() != 2 {
        // inform user
        eprintln!("Usage: {} <config file>", args[0]);
        // and exit with failure.
        std::process::exit(1);
    }

    // Get config file path from arguments.
    let config_file_path: &str = args[1].trim();

    // Generate config from file.
    let config: Config = toml::from_str(
        &std::fs::read_to_string(config_file_path)
            .expect(&format!("Could not read config file: {}", config_file_path)),
    )
    .expect(&format!(
        "Could not deserialize config file: {}",
        config_file_path
    ));

    // If no monitor groups given
    if config.monitor_groups.is_empty() {
        // inform user
        eprintln!(
            "No monitor groups given in config file: {}",
            config_file_path
        );
        // and exit with failure.
        std::process::exit(1);
    }

    // Get connection.
    let connection: connection::Connection =
        connection::Connection::new().expect("Failed to connect!");

    // Create a map of monitor id to monitor.
    let mut monitor_map: std::collections::HashMap<String, monitor::Monitor> =
        std::collections::HashMap::new();

    // Outputs to monitors.
    println!("Getting monitors...");
    // Loop outputs.
    for output in connection.outputs().expect("Failed to get outputs!") {
        // Create monitor for output.
        let monitor: monitor::Monitor = monitor::Monitor::new(&connection, output);
        // If monitor has EDID
        if monitor.has_edid() {
            // add it to map.
            monitor_map.insert(monitor.monitor_id(), monitor);
        }
    }

    // If no monitors found
    if monitor_map.is_empty() {
        // inform user
        eprintln!("No monitors found!");
        // and exit with failure.
        std::process::exit(1);
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
                let monitor: &monitor::Monitor = monitor_map
                    .get(monitor_edid)
                    .expect(&format!("Failed to get monitor for EDID: {}", monitor_edid));
                monitor.enable(x);
                if x == 0 {
                    monitor.set_primary();
                }
                x += &std::convert::TryInto::try_into(monitor.mode_info().width)
                    .expect("Failed to convert width of the monitor to i16!");
            }

            // Disable monitors that are not in monitor group.
            println!("Disabling unused monitors...");
            for (monitor_edid, monitor) in &monitor_map {
                if !monitor_group.contains(&monitor_edid) {
                    monitor.disable();
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

    // If did not set any monitor groups
    if set {
        connection.end();
    } else {
        // inform user
        eprintln!("No monitor group with all of it's monitors present found!");
        // and print all of the available monitors
        println!("Available monitors:");
        for monitor in monitor_map.values() {
            monitor.print_monitor();
        }
        // and exit with failure.
        std::process::exit(1);
    }

    // Cleanup.
    std::mem::drop(connection);

    // Done.
    println!("Done!");
}
