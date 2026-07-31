use esp_idf_svc::{
    nvs::{
        EspNvs,
        EspNvsPartition,
        NvsDefault,
    },
};
use heapless::String;
use super::errors::ApplicationError;

const NVS_KEY_SSID: &str = "net_ssid";
const NVS_KEY_PASSWORD: &str = "net_passwd";
const NVS_NAMESPACE: &str = "config";
const NVS_KEY_THRESHOLD: &str = "temp_thresh";

pub struct Nvs{
    pub store: EspNvs<NvsDefault>,
    pub partition: EspNvsPartition<NvsDefault>,
}



impl Nvs {
    pub fn new(nvs_partition: EspNvsPartition<NvsDefault>) -> anyhow::Result<Self> {
        let nvs = EspNvs::new(nvs_partition.clone(), NVS_NAMESPACE, true)?;

        Ok(Self{
            store: nvs,
            partition: nvs_partition,
        })
    }

    pub fn get_network_configuration(
        &self,
    ) -> anyhow::Result<(Option<String<32>>, Option<String<64>>)> {
        let mut ssid_buf = [0u8; 32];
        let mut password_buf = [0u8; 64];

        let ssid = match self.store.get_blob(NVS_KEY_SSID, &mut ssid_buf) {
            Ok(Some(bytes)) => core::str::from_utf8(bytes)
                .ok()
                .and_then(|text| {
                    let mut s = String::<32>::new();
                    s.push_str(text).ok().map(|_| s)
                }),
            _ => None,
        };

        let password = match self.store.get_blob(NVS_KEY_PASSWORD, &mut password_buf) {
            Ok(Some(bytes)) => core::str::from_utf8(bytes)
                .ok()
                .and_then(|text| {
                    let mut s = String::<64>::new();
                    s.push_str(text).ok().map(|_| s)
                }),
            _ => None,
        };

        Ok((ssid, password))
    }

    pub fn set_network_configuration(&self, ssid: &str, password: &str) -> Result<(), ApplicationError> {
        self.store.set_blob(NVS_KEY_SSID, ssid.as_bytes()).map_err(|_| ApplicationError::NetworkStoreError)?;
        self.store.set_blob(NVS_KEY_PASSWORD, password.as_bytes()).map_err(|_| ApplicationError::NetworkStoreError)
    }

    pub fn clear_network_configuration(&self) -> Result<(), ApplicationError> {
        self.store.remove(NVS_KEY_SSID).map_err(|_| ApplicationError::NetworkStoreError)?;
        self.store.remove(NVS_KEY_PASSWORD).map_err(|_| ApplicationError::NetworkStoreError)?;

        Ok(())
    }

    pub fn get_temperature_threshold(&self) -> f32 {
        let mut buf = [0u8; 4];
        match self.store.get_blob(NVS_KEY_THRESHOLD, &mut buf) {
            Ok(Some(bytes)) if bytes.len() == 4 => f32::from_le_bytes(bytes.try_into().unwrap()),
            _ => -50.0, // Default threshold if not found: a value always reached.
        }
    }

    pub fn set_temperature_threshold(&self, value: f32) -> Result<(), ApplicationError> {
        self.store.set_blob(NVS_KEY_THRESHOLD, &value.to_le_bytes()).map_err(|_| ApplicationError::TemperatureStoreError)
    }

    pub fn clear_temperature_threshold(&self) -> Result<(), ApplicationError> {
        self.store.remove(NVS_KEY_THRESHOLD).map_err(|_| ApplicationError::TemperatureStoreError)?;
        Ok(())
    }
}