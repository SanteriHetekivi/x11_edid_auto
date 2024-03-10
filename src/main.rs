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

    // Connect to the X server.
    let (conn, screen_num) = x11rb::connect(None).expect("Failed to connect to the X server!");

    // Get root window.
    let window_root: u32 = x11rb::connection::Connection::setup(&conn)
        .roots
        .get(screen_num)
        .expect("Failed to get root for screen number!")
        .root;

    // Get screen resources.
    let screen_resources: x11rb::protocol::randr::GetScreenResourcesReply =
        x11rb::protocol::randr::ConnectionExt::randr_get_screen_resources(&conn, window_root)
            .expect("Failed to get screen resources!")
            .reply()
            .expect("Failed to get reply from screen resources!");

    // Generate a map of mode info id to mode info.
    let mut mode_info_map: std::collections::HashMap<u32, x11rb::protocol::randr::ModeInfo> =
        std::collections::HashMap::new();
    for mode_info in screen_resources.modes {
        mode_info_map.insert(mode_info.id, mode_info);
    }

    // Create a map of monitor id to monitor.
    let mut monitor_map: std::collections::HashMap<String, monitor::Monitor> =
        std::collections::HashMap::new();

    // Outputs to monitors.
    println!("Getting monitors...");
    // Loop outputs.
    for output in screen_resources.outputs {
        // Create monitor for output.
        let monitor: monitor::Monitor = monitor::Monitor::new(&conn, output, window_root);
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
                monitor.enable(&mode_info_map, x);
                if x == 0 {
                    monitor.set_primary();
                }
                x += &std::convert::TryInto::try_into(monitor.mode_info(&mode_info_map).width)
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
    if !set {
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

    // Done.
    x11rb::connection::Connection::flush(&conn).expect("Failed to flush connection!");
    std::mem::drop(conn);
    println!("Done!");
}
