use esp_idf_svc::nvs::{
    EspDefaultNvsPartition, 
    EspNvs, 
    NvsDefault,
};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::wifi::{
    EspWifi,
    Configuration,
    ClientConfiguration,
};
use esp_idf_hal::modem::Modem;

const NVS_NAMESPACE: &str = "config";

pub struct Nvs(pub EspNvs<NvsDefault>);

pub struct Context {
    pub counter: u32,
    pub temperature_threshold: f32,
    pub nvs: Nvs,
    pub wifi: EspWifi<'static>,
}

impl Context {
    pub fn new(
        nvs_partition: EspDefaultNvsPartition,
        modem: Modem<'static>,
        sysloop: EspSystemEventLoop,
    ) -> anyhow::Result<Self> {
        let nvs = Nvs(EspNvs::new(nvs_partition, NVS_NAMESPACE, true)?);

        let temperature_threshold = nvs.get_temperature_threshold();

        let mut wifi = EspWifi::new(modem, sysloop, None)?;
        wifi.set_configuration(&Configuration::Client(ClientConfiguration::default()))?;
        wifi.start()?;

        Ok(Self {
            counter: 0,
            temperature_threshold,
            nvs,
            wifi,
        })
    }
}


