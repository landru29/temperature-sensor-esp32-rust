use std::sync::{Mutex, OnceLock};
use esp_idf_svc::{
    wifi::{
        EspWifi,
        BlockingWifi,
        Configuration,
        ClientConfiguration,
        AuthMethod,
    },
    eventloop::{
        EspEventLoop,
        System,
    },
    sys::{
        esp_wifi_sta_get_ap_info,
        wifi_ap_record_t,
    },
};


use esp_idf_hal::{
    modem::Modem,
};

use super::storage::Nvs;

pub static WIFI: OnceLock<Mutex<BlockingWifi<EspWifi<'static>>>> = OnceLock::new();


pub fn new_wifi(nvs: &Nvs, modem: Modem<'static>, sys_loop: EspEventLoop<System>)-> anyhow::Result<()> {
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(modem, sys_loop.clone(), Some(nvs.partition.clone()))?,
        sys_loop,
    )?;


    let conf = nvs.get_network_configuration()?;
    if let (Some(ssid), Some(password)) = conf {
        connect_wifi(&mut wifi, ssid.as_str(), password.as_str())?;

        WIFI.set(Mutex::new(wifi))
            .map_err(|_| anyhow::anyhow!("WIFI déjà initialisé"))?;
    }

    Ok(())
}

pub fn switch_wifi_network(new_ssid: &str, new_password: &str) -> anyhow::Result<()> {
    let mut wifi = WIFI.get()
        .ok_or_else(|| anyhow::anyhow!("WIFI non initialisé"))?
        .lock()
        .unwrap();

    log::info!("Déconnexion du réseau actuel...");
    if wifi.is_connected()? {
        wifi.disconnect()?;
    }

    let new_config = Configuration::Client(ClientConfiguration {
        ssid: new_ssid.try_into().map_err(|_| anyhow::anyhow!("SSID trop long (max 32 car.)"))?,
        password: new_password.try_into().map_err(|_| anyhow::anyhow!("Mot de passe trop long (max 64 car.)"))?,
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    });

    wifi.set_configuration(&new_config)?;

    log::info!("Connexion à {new_ssid}...");
    wifi.connect()?;
    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    log::info!("Connecté à {new_ssid}, IP: {}", ip_info.ip);

    Ok(())
}

pub fn disconnect() -> anyhow::Result<()>{
    let mut wifi = WIFI.get()
        .ok_or_else(|| anyhow::anyhow!("WIFI non initialisé"))?
        .lock()
        .unwrap();

    log::info!("Déconnexion du réseau actuel...");
    if wifi.is_connected()? {
        wifi.disconnect()?;
    }

    Ok(())
}


fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>, ssid: &str, password: &str) -> anyhow::Result<()> {
    let wifi_config = Configuration::Client(ClientConfiguration {
        ssid: ssid.try_into().unwrap(),
        password: password.try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_config)?;
    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;
 
    Ok(())
}

pub fn get_hostname() -> anyhow::Result<String> {
    if let Some(mutex_wifi) = WIFI.get() {
        let wifi = mutex_wifi.lock().unwrap();
        
        let hostname = wifi
            .wifi()
            .sta_netif()
            .get_hostname()
            .unwrap_or_else(|_| {
                let mut h = heapless::String::<30>::new();
                    h.push_str("unknown").unwrap();
                    h
            });

         anyhow::Ok(hostname.as_str().to_string())
    } else {
        Err(anyhow::anyhow!("WIFI non initialisé"))
    }
}

pub fn interface_str() -> anyhow::Result<(String, String, String)> {
    if let Some(mutex_wifi) = WIFI.get() {
        let wifi = mutex_wifi.lock().unwrap();

        if let Ok(ip_info) = wifi.wifi().sta_netif().get_ip_info() {
            let ip_octets = ip_info.ip.octets();
            let ip_str = format!(
                "IP Address: {}.{}.{}.{}",
                ip_octets[0], ip_octets[1], ip_octets[2], ip_octets[3]
            );

            let netmask_str = format!(
                "Netmask: {}",
                ip_info.subnet.mask
            );

            let gateway_octets = ip_info.subnet.gateway.octets();
            let gateway_str = format!(
                "Gateway: {}.{}.{}.{}",
                gateway_octets[0],
                gateway_octets[1],
                gateway_octets[2],
                gateway_octets[3]
            );

            return anyhow::Ok((ip_str, netmask_str, gateway_str))
        };
    };

    return Err(anyhow::anyhow!("no network interface"));
}

pub fn scan_wifi() -> Vec<String> {
    let mut output = Vec::new();

    if let Some(mutex_wifi) = WIFI.get() {
        let mut wifi = mutex_wifi.lock().unwrap();

        match wifi.scan() {
            Ok(ap_infos) => {
                for ap in ap_infos {
                    output.push(format!(
                        "{:<32} RSSI:{:>4} Ch:{:>2} {:?}",
                        ap.ssid, ap.signal_strength, ap.channel, ap.auth_method
                    ));
                }
            }
            Err(_) => output.push("no network interface".to_string())
        };
    }

    return output
}

pub fn get_current_ssid() -> anyhow::Result<String> {
    let mut ap_info: wifi_ap_record_t = unsafe { std::mem::zeroed() };

    esp_idf_svc::sys::esp!(unsafe { esp_wifi_sta_get_ap_info(&mut ap_info) })?;

    let len = ap_info
        .ssid
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(ap_info.ssid.len());

    Ok(String::from_utf8_lossy(&ap_info.ssid[..len]).into_owned())
}

