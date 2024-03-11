// Structure for connection.
pub(crate) struct Connection {
    // Connection to X server.
    connection: x11rb::rust_connection::RustConnection,
    // Root window id.
    window_root: u32,
    // Mode info map.
    mode_info_map: std::collections::HashMap<u32, x11rb::protocol::randr::ModeInfo>,
    screen_num: usize,
}

// Methods for connection.
impl Connection {
    // Make new X11 connection.
    pub(crate) fn new() -> Result<Self, crate::errors::ConnectionNewError> {
        // Connect to the X server.
        let (connection, screen_num) = x11rb::connect(None)?;

        // Generate connection.
        Ok(Connection {
            connection,
            window_root: 0,
            mode_info_map: std::collections::HashMap::new(),
            screen_num,
        }
        .generate_window_root()?
        .generate_mode_info_map()?)
    }

    // Generate window root.
    fn generate_window_root(mut self) -> Result<Self, crate::errors::NoRootForScreenNumberError> {
        self.window_root = match x11rb::connection::Connection::setup(&self.connection)
            .roots
            .get(self.screen_num)
        {
            None => {
                return Err(crate::errors::NoRootForScreenNumberError::new(
                    self.screen_num,
                ))
            }
            Some(screen) => screen.root,
        };
        Ok(self)
    }

    // Generate mode info map.
    fn generate_mode_info_map(mut self) -> Result<Self, x11rb::errors::ReplyError> {
        // Generate a map of mode info id to mode info.
        for mode_info in self.screen_resources()?.modes {
            self.mode_info_map.insert(mode_info.id, mode_info);
        }

        Ok(self)
    }

    // Get EDID for monitor's output.
    pub(crate) fn edid(&self, output: u32) -> Vec<u8> {
        x11rb::protocol::randr::ConnectionExt::randr_get_output_property(
            &self.connection,
            output,
            x11rb::protocol::xproto::ConnectionExt::intern_atom(&self.connection, false, b"EDID")
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

    // Get output info for given output id.
    pub(crate) fn get_output_info(
        &self,
        output: u32,
    ) -> x11rb::protocol::randr::GetOutputInfoReply {
        x11rb::protocol::randr::ConnectionExt::randr_get_output_info(&self.connection, output, 0)
            .expect("Failed to get output info!")
            .reply()
            .expect("Failed to get reply from getting output info!")
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
    fn print_crtc_info(crtc_info: &x11rb::protocol::randr::GetCrtcInfoReply) {
        println!("crtc_info:");
        println!("\t x: {:?}", crtc_info.x);
        println!("\t y: {:?}", crtc_info.y);
        println!("\t width: {:?}", crtc_info.width);
        println!("\t height: {:?}", crtc_info.height);
        println!("\t mode: {:?}", crtc_info.mode);
        println!("\t rotation: {:?}", crtc_info.rotation);
    }

    pub(crate) fn mode_info_map(
        &self,
    ) -> &std::collections::HashMap<u32, x11rb::protocol::randr::ModeInfo> {
        &self.mode_info_map
    }

    // Set CRTC config
    pub(crate) fn set_crtc_config(
        &self,
        crtc: u32,
        x: i16,
        y: i16,
        mode: u32,
        rotation: x11rb::protocol::randr::Rotation,
        outputs: &[u32],
        crtc_existing: u32,
    ) -> &Self {
        // Get CRTC info.
        let crtc_info: x11rb::protocol::randr::GetCrtcInfoReply = self.get_crtc_info(crtc);
        // Print CRTC info.
        Self::print_crtc_info(&crtc_info);

        // If is already
        if crtc == crtc_existing
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

        // Flush connection.
        self.flush()
    }

    // Get screen resources.
    fn screen_resources(
        &self,
    ) -> Result<x11rb::protocol::randr::GetScreenResourcesReply, x11rb::errors::ReplyError> {
        Ok(
            x11rb::protocol::randr::ConnectionExt::randr_get_screen_resources(
                &self.connection,
                self.window_root,
            )?
            .reply()?,
        )
    }

    // Get outputs.
    pub(crate) fn outputs(&self) -> Result<Vec<u32>, x11rb::errors::ReplyError> {
        Ok(self.screen_resources()?.outputs)
    }

    // Get current screen resources.
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
    pub(crate) fn get_free_crtc(&self) -> u32 {
        self.screen_resources_current()
            .crtcs
            .iter()
            .find(|crtc| (**crtc != 0 && self.get_crtc_info(**crtc).mode == 0))
            .expect("Did not find available CRTC!")
            .clone()
    }

    // Set output as primary.
    pub(crate) fn set_output_primary(&self, output: u32) -> &Self {
        x11rb::protocol::randr::ConnectionExt::randr_set_output_primary(
            &self.connection,
            self.window_root,
            output,
        )
        .expect("Failed to set output as primary!");
        self
    }

    // Update screen size to fit all monitors.
    fn update_screen_size(&self) -> &Self {
        let mut total_width_px: u16 = 0;
        let mut max_height_px: u16 = 0;
        let mut total_width_mm: u32 = 0;
        let mut max_height_mm: u32 = 0;
        // Loop all of the CRTCs.
        for crtc in self.screen_resources_current().crtcs {
            // Get info.
            let crtc_info = self.get_crtc_info(crtc);

            // Skip ones that are not in use.
            if crtc_info.mode == 0 {
                continue;
            }

            // Make sure that has only one output.
            if crtc_info.outputs.len() != 1 {
                panic!("CRTC has {} outputs!", crtc_info.outputs.len());
            }

            // Get monitor info for output.
            let output_info: x11rb::protocol::randr::GetOutputInfoReply =
                self.get_output_info(crtc_info.outputs[0]);

            // Get width in pixels.
            let width_px: u16 = crtc_info.width;
            // Get height in pixels.
            let height_px: u16 = crtc_info.height;

            // Update total width and heights.
            total_width_px += width_px;
            max_height_px = std::cmp::max(max_height_px, height_px);
            total_width_mm += (std::convert::TryInto::<f64>::try_into(width_px)
                .expect("Failed to transform width in pixels to float")
                * (std::convert::TryInto::<f64>::try_into(output_info.mm_width)
                    .expect("Failed to transform width in pixels to float")
                    / std::convert::TryInto::<f64>::try_into(crtc_info.width)
                        .expect("Failed to transform width in millimeters to float")))
            .ceil() as u32;
            max_height_mm = std::cmp::max(
                max_height_mm,
                (std::convert::TryInto::<f64>::try_into(height_px)
                    .expect("Failed to transform height in pixels to float")
                    * (std::convert::TryInto::<f64>::try_into(output_info.mm_width)
                        .expect("Failed to transform height in pixels to float")
                        / std::convert::TryInto::<f64>::try_into(crtc_info.height)
                            .expect("Failed to transform height in millimeters to float")))
                .ceil() as u32,
            );
        }

        // Set screen size.
        x11rb::protocol::randr::ConnectionExt::randr_set_screen_size(
            &self.connection,
            self.window_root,
            total_width_px,
            max_height_px,
            total_width_mm,
            max_height_mm,
        )
        .expect("Failed to set screen size!");
        println!(
            "Set screen size to {}x{}px, {}x{}mm",
            total_width_px, max_height_px, total_width_mm, max_height_mm
        );
        self
    }

    // Flush connection.
    fn flush(&self) -> &Self {
        x11rb::connection::Connection::flush(&self.connection)
            .expect("Failed to flush connection!");
        self
    }

    // End connection.
    pub(crate) fn end(&self) -> &Self {
        self.update_screen_size();
        self.flush()
    }
}
