use super::storage::Nvs;

const SERVER_THREAD_STACK_SIZE: usize = 8192;

// pub struct Nvs(pub EspNvs<NvsDefault>);

pub struct Context {
    pub nvs: Nvs,
    // pub wifi: EspWifi<'static>,
    // pub timer: TimerConfiguration<'d>,
    // pub server: Option<Server<'d>>,
    // pub server_thread: Option<thread::JoinHandle<()>>,
}

impl Context {
    pub fn new(
        // interface: &mut UartIo,
        nvs: Nvs,
        // modem: Modem<'static>,
        // sysloop: EspSystemEventLoop,
        // timer: TimerConfiguration<'d>,
    ) -> anyhow::Result<Self> {

        // let mut wifi_configuration = ClientConfiguration::default();

        // let mut wifi_configured = false;
        // let mut saved_ssid = None;
        // let mut saved_password = None;

        // nvs.get_network_configuration().map(|(ssid, password)| {
        //     if let (Some(ssid), Some(password)) = (ssid, password) {
        //         writeln!(interface, "Stored WiFi configuration: SSID: {}, Password: {}", ssid, password).unwrap();
        //         wifi_configuration.ssid = ssid.clone();
        //         wifi_configuration.password = password.clone();
        //         saved_ssid = Some(ssid);
        //         saved_password = Some(password);
        //         wifi_configured = true;
        //     } else {
        //         writeln!(interface, "No stored WiFi configuration found.").unwrap();
        //     }
        // }).unwrap_or_else(|e| {
        //     writeln!(interface, "Error retrieving network configuration: {:?}", e).unwrap();
        // });

        // let mut wifi = EspWifi::new(modem, sysloop, None)?;
        // wifi.set_configuration(&Configuration::Client(wifi_configuration))?;
        // wifi.start()?;

        let output = Self {
            nvs,
            // wifi,
            // timer: timer,
            // server_thread: None,
            // server: None,
        };

        // if wifi_configured {
        //     if let (Some(ssid), Some(password)) = (saved_ssid, saved_password) {
        //         output.connect_wifi(interface, &ssid, &password)?;
        //     }
        // }

        Ok(output)
    }

    // pub fn get_network_configuration(& mut self)-> Option<(String<32>, String<64>)> {
    //     match self.nvs.get_network_configuration() {
    //         Ok(data) => {
    //             if let (Some(ssid), Some(password)) = data {
    //                 Some((ssid, password))
    //             }else {
    //                 None
    //             }
    //         },
    //         Err(_) => None,
    //     }
    // }

    // pub fn set_network_configuration(&self, ssid: &str, password: &str) -> Result<(), ApplicationError> {
    //     self.nvs.set_network_configuration(ssid, password)
    // }

    // pub fn connect_wifi(&mut self, interface: &mut UartIo, ssid: &str, password: &str) -> anyhow::Result<()> {
    //     writeln!(interface, "Connecting to WiFi...").unwrap();
    //     match self.wifi.connect() {
    //         Ok(_) => {
    //             writeln!(interface, "Connected to WiFi successfully!").unwrap();
    //             self.nvs.set_network_configuration(ssid, password);

    //             let handle = thread::Builder::new()
    //                 .stack_size(SERVER_THREAD_STACK_SIZE)
    //                 .spawn(|| {
    //                     match Server::new() {
    //                         Ok(_server) => {
    //                             log::info!("HTTP server started successfully.");
    //                             // Le serveur doit rester vivant tant que le thread tourne.
    //                             loop {
    //                                 thread::sleep(Duration::from_secs(1));
    //                             }
    //                         }
    //                         Err(e) => {
    //                             log::error!("Error starting HTTP server: {:?}", e);
    //                         }
    //                     }
    //                 })?;

    //                 self.server_thread = Some(handle);

    //             // match Server::new() {
    //             //     Ok(server) => self.server = Some(server),
    //             //     Err(e) => {
    //             //         writeln!(interface, "Error starting HTTP server: {:?}", e).unwrap();
    //             //         return Err(e);
    //             //     }
    //             // }
    //         },
    //         Err(e) => writeln!(interface, "Error connecting to WiFi: {:?}", e).unwrap(),
    //     }

    //     Ok(())
    // }
}


