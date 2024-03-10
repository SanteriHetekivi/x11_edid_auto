// Structure for monitor.
pub(crate) struct Monitor<'a> {
    // Connection to X server.
    connection: &'a crate::connection::Connection,
    // Output id.
    output: u32,
    // EDID.
    edid: Vec<u8>,
}

// Methods for monitor.
impl<'a> Monitor<'a> {
    // Create a new monitor with
    pub(crate) fn new(
        // reference to connection
        connection: &'a crate::connection::Connection,
        // and output id.
        output: u32,
    ) -> Self {
        Monitor {
            connection,
            output,
            edid: connection.edid(output),
        }
    }

    // Has EDID?
    pub(crate) fn has_edid(&self) -> bool {
        !self.edid.is_empty()
    }

    // Manifacturer ID.
    fn manufacturer_id(&self) -> u16 {
        u16::from_be_bytes([self.edid[8], self.edid[9]])
    }

    // Product code.
    fn product_code(&self) -> u16 {
        u16::from_be_bytes([self.edid[10], self.edid[11]])
    }

    // Serial number.
    fn serial_number(&self) -> u32 {
        u32::from_be_bytes([self.edid[12], self.edid[13], self.edid[14], self.edid[15]])
    }

    // Monitor ID.
    pub(crate) fn monitor_id(&self) -> String {
        format!(
            "{:04X}:{:04X}:{:08X}",
            self.manufacturer_id(),
            self.product_code(),
            self.serial_number()
        )
    }

    // Get output info for monitor's output.
    fn output_info(&self) -> x11rb::protocol::randr::GetOutputInfoReply {
        self.connection.get_output_info(self.output)
    }

    // Name for monitor's output.
    fn name(&self) -> String {
        String::from_utf8(self.output_info().name.to_vec())
            .expect("Failed to convert output name to string!")
    }

    // CRTC for monitor's output.
    fn crtc(&self) -> u32 {
        self.output_info().crtc
    }

    // Print monitor information.
    pub(crate) fn print_monitor(&self) -> &Self {
        println!("Monitor:");
        println!("\t edid: {:?}", self.monitor_id());
        println!("\t name: {:?}", self.name());
        println!("\t crtc: {:?}", self.crtc());
        return self;
    }

    // Get mode info for monitor's output.
    pub(crate) fn mode_info(&self) -> x11rb::protocol::randr::ModeInfo {
        let mode_info_map: &std::collections::HashMap<u32, x11rb::protocol::randr::ModeInfo> =
            self.connection.mode_info_map();
        self.output_info()
            .modes
            .iter()
            .filter_map(|mode_id| mode_info_map.get(mode_id)) // Filter modes that exist in mode_info_map
            .max_by(|a, b| {
                a.width
                    .cmp(&b.width)
                    .then_with(|| a.height.cmp(&b.height))
                    .then_with(|| b.dot_clock.cmp(&a.dot_clock))
            })
            .expect("Failed to get max mode info for output!")
            .clone()
    }

    // Set CRTC config.
    pub(crate) fn enable(&self, x: i16) -> &Self {
        // Print monitor info.
        self.print_monitor();

        // Get CRTC.
        let crtc_existing: u32 = self.crtc();
        let crtc: u32;
        if crtc_existing == 0 {
            crtc = self.connection.get_free_crtc();
        } else {
            crtc = crtc_existing;
        }

        // Set CRTC config.
        self.connection.set_crtc_config(
            crtc,
            x,
            0,
            self.mode_info().id,
            x11rb::protocol::randr::Rotation::ROTATE0,
            &[self.output],
            crtc_existing,
        );

        self
    }

    // Disable monitor.
    pub(crate) fn disable(&self) -> &Self {
        // Print monitor info.
        self.print_monitor();

        // Get CRTC.
        let crtc: u32 = self.crtc();
        if crtc == 0 {
            println!("Monitor is already disabled!");
            return self;
        }

        // Set CRTC config.
        self.connection.set_crtc_config(
            crtc,
            0,
            0,
            0,
            x11rb::protocol::randr::Rotation::ROTATE0,
            &[],
            crtc,
        );

        self
    }

    // Set output as primary.
    pub(crate) fn set_primary(&self) -> &Self {
        self.connection.set_output_primary(self.output);
        self
    }
}
