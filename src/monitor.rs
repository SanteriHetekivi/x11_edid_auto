// Structure for monitor.
pub(crate) struct Monitor<'a> {
    // Connection to X server.
    connection: &'a x11rb::rust_connection::RustConnection,
    // Output id.
    output: u32,
    // EDID.
    edid: Vec<u8>,
    // Root window id.
    window_root: u32,
}

// Methods for monitor.
impl<'a> Monitor<'a> {
    // Create a new monitor with
    pub(crate) fn new(
        // reference to connection
        connection: &'a x11rb::rust_connection::RustConnection,
        // output id
        output: u32,
        // and root window id.
        window_root: u32,
    ) -> Self {
        Monitor {
            connection,
            output,
            edid: Self::edid_get(connection, output),
            window_root,
        }
    }

    // Get EDID for monitor's output.
    fn edid_get(connection: &x11rb::rust_connection::RustConnection, output: u32) -> Vec<u8> {
        x11rb::protocol::randr::ConnectionExt::randr_get_output_property(
            &connection,
            output,
            x11rb::protocol::xproto::ConnectionExt::intern_atom(connection, false, b"EDID")
                .expect("Failed to get atom identifier for EDID!")
                .reply()
                .expect("Failed to get reply from getting atom identifier for EDID!")
                .atom,
            x11rb::protocol::xproto::AtomEnum::ANY,
            0,
            u32::MAX,
            false,
            false,
        )
        .expect("Failed to get output property!")
        .reply()
        .expect("Failed to get reply from getting output property!")
        .data
        .to_vec()
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

    fn get_output_info(&self, output: u32) -> x11rb::protocol::randr::GetOutputInfoReply {
        x11rb::protocol::randr::ConnectionExt::randr_get_output_info(&self.connection, output, 0)
            .expect("Failed to get output info!")
            .reply()
            .expect("Failed to get reply from getting output info!")
    }

    // Get output info for monitor's output.
    fn output_info(&self) -> x11rb::protocol::randr::GetOutputInfoReply {
        self.get_output_info(self.output)
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

    // Get CRTC info for monitor's output's CRTC.
    fn get_crtc_info(&self, crtc: u32) -> x11rb::protocol::randr::GetCrtcInfoReply {
        x11rb::protocol::randr::ConnectionExt::randr_get_crtc_info(
            &self.connection,
            crtc,
            x11rb::CURRENT_TIME,
        )
        .expect("Failed to get CRTC info!")
        .reply()
        .expect("Failed to get reply from getting CRTC info!")
    }

    // Print CRTC info.
    fn print_crtc_info(&self, crtc_info: &x11rb::protocol::randr::GetCrtcInfoReply) -> &Self {
        println!("crtc_info:");
        println!("\t x: {:?}", crtc_info.x);
        println!("\t y: {:?}", crtc_info.y);
        println!("\t width: {:?}", crtc_info.width);
        println!("\t height: {:?}", crtc_info.height);
        println!("\t mode: {:?}", crtc_info.mode);
        println!("\t rotation: {:?}", crtc_info.rotation);
        return self;
    }

    // Get mode info for monitor's output with
    pub(crate) fn mode_info(
        &self,
        // mode info map.
        mode_info_map: &std::collections::HashMap<u32, x11rb::protocol::randr::ModeInfo>,
    ) -> x11rb::protocol::randr::ModeInfo {
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

    fn update_screen_size(&self) -> &Self {
        let mut total_width_px: u16 = 0;
        let mut total_height_px: u16 = 0;
        let mut total_width_mm: u32 = 0;
        let mut total_height_mm: u32 = 0;
        for crtc in &self.screen_resources_current().crtcs {
            let crtc_info = self.get_crtc_info(*crtc);
            if crtc_info.mode == 0 {
                continue;
            }
            let width_px: u16 = std::convert::TryInto::<u16>::try_into(crtc_info.x)
                .expect("Failed to transform X coordinate to unsigned integer")
                + crtc_info.width;
            let height_px: u16 = std::convert::TryInto::<u16>::try_into(crtc_info.y)
                .expect("Failed to transform X coordinate to unsigned integer")
                + crtc_info.height;
            total_width_px = std::cmp::max(total_width_px, width_px);
            total_height_px = std::cmp::max(total_height_px, height_px);
            let dpi: f64 = std::convert::TryInto::<f64>::try_into(crtc_info.width)
                .expect("Failed to transform width in pixels to float")
                / std::convert::TryInto::<f64>::try_into(
                    self.get_output_info(crtc_info.outputs[0]).mm_width,
                )
                .expect("Failed to transform width in millimeters to float");
            total_width_mm = std::cmp::max(
                total_width_mm,
                (std::convert::TryInto::<f64>::try_into(width_px)
                    .expect("Failed to transform width in pixels to float")
                    / dpi
                    * 25.4)
                    .ceil() as u32,
            );
            total_height_mm = std::cmp::max(
                total_height_mm,
                (std::convert::TryInto::<f64>::try_into(height_px)
                    .expect("Failed to transform height in pixels to float")
                    / dpi
                    * 25.4)
                    .ceil() as u32,
            );
        }

        x11rb::protocol::randr::ConnectionExt::randr_set_screen_size(
            self.connection,
            self.window_root,
            total_width_px,
            total_height_px,
            total_width_mm,
            total_height_mm,
        )
        .expect("Failed to set screen size!");
        println!(
            "Set screen size to {}x{}px, {}x{}mm",
            total_width_px, total_height_px, total_width_mm, total_height_mm
        );
        self
    }

    // Set CRTC config
    fn set_crtc_config(
        &self,
        crtc: u32,
        x: i16,
        y: i16,
        mode: u32,
        rotation: x11rb::protocol::randr::Rotation,
        outputs: &[u32],
    ) -> &Self {
        // Get CRTC info.
        let crtc_info: x11rb::protocol::randr::GetCrtcInfoReply = self.get_crtc_info(crtc);
        // Print CRTC info.
        self.print_crtc_info(&crtc_info);

        // If is already
        if crtc == self.crtc()
            && crtc_info.x == x
            && crtc_info.y == y
            && crtc_info.mode == mode
            && crtc_info.rotation == rotation
            && crtc_info.outputs == outputs
        {
            // inform
            println!("CRTC config is already set!");
            // and just return.
            return self;
        }

        // Set CRTC config.
        println!("Setting CRTC config to:");
        println!("\t crtc: {:?}", crtc);
        println!("\t x: {:?}", x);
        println!("\t y: {:?}", y);
        println!("\t mode: {:?}", mode);
        println!("\t rotation: {:?}", rotation);
        println!("\t outputs: {:?}", outputs);
        x11rb::protocol::randr::ConnectionExt::randr_set_crtc_config(
            &self.connection,
            crtc,
            x11rb::CURRENT_TIME,
            crtc_info.timestamp,
            x,
            y,
            mode,
            rotation,
            outputs,
        )
        .expect("Failed to set CRTC config!");

        // Updating screen size.
        self.update_screen_size()
    }

    // Get screen resources.
    fn screen_resources_current(&self) -> x11rb::protocol::randr::GetScreenResourcesCurrentReply {
        x11rb::protocol::randr::ConnectionExt::randr_get_screen_resources_current(
            &self.connection,
            self.window_root,
        )
        .expect("Failed to get screen resources current!")
        .reply()
        .expect("Failed to get reply from getting screen resources current!")
    }

    // Get free CRTC.
    fn get_free_crtc(&self) -> u32 {
        self.screen_resources_current()
            .crtcs
            .iter()
            .find(|crtc| (**crtc != 0 && self.get_crtc_info(**crtc).mode == 0))
            .expect("Did not find available CRTC!")
            .clone()
    }

    // Set CRTC config with
    pub(crate) fn enable(
        &self,
        // mode info map
        mode_info_map: &std::collections::HashMap<u32, x11rb::protocol::randr::ModeInfo>,
        // and x coordinate.
        x: i16,
    ) -> &Self {
        // Print monitor info.
        self.print_monitor();

        // Get CRTC.
        let mut crtc: u32 = self.crtc();
        if crtc == 0 {
            crtc = self.get_free_crtc();
        }

        // Set CRTC config.
        self.set_crtc_config(
            crtc,
            x,
            0,
            self.mode_info(mode_info_map).id,
            x11rb::protocol::randr::Rotation::ROTATE0,
            &[self.output],
        )
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
        self.set_crtc_config(
            crtc,
            0,
            0,
            0,
            x11rb::protocol::randr::Rotation::ROTATE0,
            &[],
        )
    }

    // Set output as primary.
    pub(crate) fn set_primary(&self) -> &Self {
        x11rb::protocol::randr::ConnectionExt::randr_set_output_primary(
            &self.connection,
            self.window_root,
            self.output,
        )
        .expect("Failed to set output as primary!");
        self
    }
}
