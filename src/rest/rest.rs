use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use anyhow::Result;
use serde_json::json;

pub struct Server<'d> {
    pub http: EspHttpServer<'d>,
}

impl <'d> Server<'d> {
    pub fn new() -> Result<Self> {
        let mut server = EspHttpServer::new(&Configuration::default())?;

    // Route pour servir page.html sur /
    server.fn_handler("/", esp_idf_svc::http::Method::Get, |req| {
        let html = include_str!("../rest/page.html");
        req.into_response(200, Some(html), &[("content-type", "text/html; charset=utf-8")])?;
        Ok::<(), anyhow::Error>(())
    })?;

    // Route pour servir les données JSON sur /data
    server.fn_handler("/data", esp_idf_svc::http::Method::Get, |req| {
        let data = json!({
            "s": 42,
            "t": 12.5,
            "d": 0.4
        });
        let json_str = data.to_string();
        req.into_response(200, Some(json_str.as_str()), &[("content-type", "application/json")])?;
        Ok::<(), anyhow::Error>(())
    })?;

    Ok(Server { http: server })
    }
}

// pub fn start_server() -> Result<EspHttpServer> {
//     let mut server = EspHttpServer::new(&Configuration::default())?;

//     // Route pour servir page.html sur /
//     server.fn_handler("/", esp_idf_svc::http::Method::Get, |req| {
//         let html = include_str!("../rest/page.html");
//         req.into_response(200, Some(html), &[("content-type", "text/html; charset=utf-8")])?;
//         Ok(())
//     })?;

//     // Route pour servir les données JSON sur /data
//     server.fn_handler("/data", esp_idf_svc::http::Method::Get, |req| {
//         let data = json!({
//             "s": 42,
//             "t": 12.5,
//             "d": 0.4
//         });
//         let json_str = data.to_string();
//         req.into_response(200, Some(json_str.as_str()), &[("content-type", "application/json")])?;
//         Ok(())
//     })?;

//     Ok(server)
// }
