use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use anyhow::Result;

pub struct Server<'d> {
    pub http: EspHttpServer<'d>,
}

impl<'d> Server<'d> {
    pub fn new() -> Result<Self> {
        let mut server = EspHttpServer::new(&Configuration::default())?;

        // Route pour servir page.html sur /
        server.fn_handler("/", esp_idf_svc::http::Method::Get, |req| {
            let html = include_str!("../rest/page.html");
            let mut resp = req.into_response(200, Some("OK"), &[("content-type", "text/html; charset=utf-8")])?;
            resp.write(html.as_bytes())?;
            Ok::<(), anyhow::Error>(())
        })?;

        // Route pour servir la favicon sur /favicon.ico
        server.fn_handler("/favicon.ico", esp_idf_svc::http::Method::Get, |req| {
            let icon = include_bytes!("../rest/favicon.ico");
            let mut resp = req.into_response(200, Some("OK"), &[("content-type", "image/x-icon")])?;
            resp.write(icon)?;
            Ok::<(), anyhow::Error>(())
        })?;

        // Route pour servir les données JSON sur /data
        server.fn_handler("/data", esp_idf_svc::http::Method::Get, |req| { 
            let data = format!("{{\"s\": {}, \"t\": {}, \"d\": {}}}", 42, 12.5, 0.4);
            let mut resp = req.into_response(200, Some("OK"), &[("content-type", "application/json")])?;
            resp.write(data.as_bytes())?;

            Ok::<(), anyhow::Error>(())
        })?;

        Ok(Server { http: server })
    }
}
