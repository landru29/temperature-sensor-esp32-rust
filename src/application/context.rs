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

const NVS_NAMESPACE: &str = "config";

pub struct Nvs(pub EspNvs<NvsDefault>);

pub struct Context<'d> {
    pub nvs: Nvs,
    pub wifi: EspWifi<'static>,
    pub timer: TimerConfiguration<'d>,
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

        nvs.get_network_configuration().map(|(ssid, password)| {
            if let (Some(ssid), Some(password)) = (ssid, password) {
                writeln!(interface, "Stored WiFi configuration: SSID: {}, Password: {}", ssid, password).unwrap();
                wifi_configuration.ssid = ssid;
                wifi_configuration.password = password;
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

        if wifi_configured {
            writeln!(interface, "Connecting to WiFi...").unwrap();
            match wifi.connect() {
                Ok(_) => writeln!(interface, "Connected to WiFi successfully!").unwrap(),
                Err(e) => writeln!(interface, "Error connecting to WiFi: {:?}", e).unwrap(),
            }
        }

        Ok(Self {
            nvs,
            wifi,
            timer: timer,
        })
    }
}


