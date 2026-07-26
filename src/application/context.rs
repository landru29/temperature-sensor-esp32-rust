use embedded_io::Write;
use esp_idf_hal::modem::Modem;
use esp_idf_svc::{
    nvs::{
        EspDefaultNvsPartition, 
        EspNvs, 
        NvsDefault,
    },
    eventloop::EspSystemEventLoop,
    wifi::{
        EspWifi,
        Configuration,
        ClientConfiguration,
    },
};

use super::uart::UartIo;
use crate::clock::config::TimerConfiguration;
use crate::rest::rest::Server;

const NVS_NAMESPACE: &str = "config";

pub struct Nvs(pub EspNvs<NvsDefault>);

pub struct Context<'d> {
    pub nvs: Nvs,
    pub wifi: EspWifi<'static>,
    pub timer: TimerConfiguration<'d>,
    pub server: Option<Server<'d>>,
}

impl<'d> Context<'d> {
    pub fn new(
        interface: &mut UartIo,
        nvs_partition: EspDefaultNvsPartition,
        modem: Modem<'static>,
        sysloop: EspSystemEventLoop,
        timer: TimerConfiguration<'d>,
    ) -> anyhow::Result<Self> {
        let nvs = Nvs(EspNvs::new(nvs_partition, NVS_NAMESPACE, true)?);

        let mut wifi_configuration = ClientConfiguration::default();

        let mut wifi_configured = false;
        let mut saved_ssid = None;
        let mut saved_password = None;

        nvs.get_network_configuration().map(|(ssid, password)| {
            if let (Some(ssid), Some(password)) = (ssid, password) {
                writeln!(interface, "Stored WiFi configuration: SSID: {}, Password: {}", ssid, password).unwrap();
                wifi_configuration.ssid = ssid.clone();
                wifi_configuration.password = password.clone();
                saved_ssid = Some(ssid);
                saved_password = Some(password);
                wifi_configured = true;
            } else {
                writeln!(interface, "No stored WiFi configuration found.").unwrap();
            }
        }).unwrap_or_else(|e| {
            writeln!(interface, "Error retrieving network configuration: {:?}", e).unwrap();
        });

        let mut wifi = EspWifi::new(modem, sysloop, None)?;
        wifi.set_configuration(&Configuration::Client(wifi_configuration))?;
        wifi.start()?;

        let mut output = Self {
            nvs,
            wifi,
            timer: timer,
            server: None,
        };

        if wifi_configured {
            if let (Some(ssid), Some(password)) = (saved_ssid, saved_password) {
                output.connect_wifi(interface, &ssid, &password)?;
            }
        }

        Ok(output)
    }

    pub fn connect_wifi(&mut self, interface: &mut UartIo, ssid: &str, password: &str) -> anyhow::Result<()> {
        writeln!(interface, "Connecting to WiFi...").unwrap();
        match self.wifi.connect() {
            Ok(_) => {
                writeln!(interface, "Connected to WiFi successfully!").unwrap();
                self.nvs.set_network_configuration(ssid, password);

                match Server::new() {
                    Ok(server) => self.server = Some(server),
                    Err(e) => {
                        writeln!(interface, "Error starting HTTP server: {:?}", e).unwrap();
                        return Err(e);
                    }
                }
            },
            Err(e) => writeln!(interface, "Error connecting to WiFi: {:?}", e).unwrap(),
        }

        Ok(())
    }
}


