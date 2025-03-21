use esp_idf_svc::io::Write;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = esp_idf_svc::hal::prelude::Peripherals::take()?;
    let sysloop = esp_idf_svc::eventloop::EspSystemEventLoop::take()?;
    let mut wifi = esp_idf_svc::wifi::EspWifi::new(peripherals.modem, sysloop, None)?;

    let ap_config = esp_idf_svc::wifi::Configuration::AccessPoint(
        esp_idf_svc::wifi::AccessPointConfiguration {
            ssid: heapless::String::try_from("Star Destroyer").unwrap(),
            ssid_hidden: false,
            channel: 1,
            auth_method: esp_idf_svc::wifi::AuthMethod::None,
            password: heapless::String::new(),
            max_connections: 1,
            ..Default::default()
        },
    );

    wifi.set_configuration(&ap_config)?;
    wifi.start()?;

    log::info!("Wifi started!");

    let laser_mosfet = std::sync::Arc::new(std::sync::Mutex::new(
        esp_idf_svc::hal::gpio::PinDriver::output(peripherals.pins.gpio23)?,
    ));
    let buildup1 = std::sync::Arc::new(std::sync::Mutex::new(
        esp_idf_svc::hal::gpio::PinDriver::output(peripherals.pins.gpio8)?,
    ));
    let buildup2 = std::sync::Arc::new(std::sync::Mutex::new(
        esp_idf_svc::hal::gpio::PinDriver::output(peripherals.pins.gpio7)?,
    ));
    let buildup3 = std::sync::Arc::new(std::sync::Mutex::new(
        esp_idf_svc::hal::gpio::PinDriver::output(peripherals.pins.gpio6)?,
    ));
    let hyperspace_left = std::sync::Arc::new(std::sync::Mutex::new(
        esp_idf_svc::hal::gpio::PinDriver::output(peripherals.pins.gpio18)?,
    ));
    let hyperspace_right = std::sync::Arc::new(std::sync::Mutex::new(
        esp_idf_svc::hal::gpio::PinDriver::output(peripherals.pins.gpio19)?,
    ));

    let mut server = esp_idf_svc::http::server::EspHttpServer::new(
        &esp_idf_svc::http::server::Configuration::default(),
    )?;

    server.fn_handler(
        "/",
        esp_idf_svc::http::Method::Get,
        |request| -> core::result::Result<(), esp_idf_svc::hal::io::EspIOError> {
            let html = r#"
                <!DOCTYPE html>
                <html>
                    <head>
                        <meta charset="utf-8">
                        <title>Star Destroyer</title>
                    </head>
                    <body>
                        <h1>
                        <form action="/shoot">
                            <input type="submit" value="Fire the STAR DESTROYER!" />
                        </form>
                        <form action="/hyperspace">
                            <input type="submit" value="Enter Hyperspace!" />
                        </form>
                        </h1>
                    </body>
                </html>"#;

            let mut response = request.into_ok_response()?;
            response.write_all(html.as_bytes())?;
            Ok(())
        },
    )?;

    server.fn_handler(
        "/shoot",
        esp_idf_svc::http::Method::Get,
        move |request| -> core::result::Result<(), esp_idf_svc::hal::io::EspIOError> {
            let html = r#"
                <!DOCTYPE html>
                <html>
                    <head>
                        <meta charset="utf-8">
                        <meta http-equiv="refresh" content="0; url=/" />
                        <title>Star Destroyer</title>
                    </head>
                    <body>
                        <p><a href="/">Redirect</a></p>
                    </body>
                </html>"#;

            let mut response = request.into_ok_response()?;
            response.write_all(html.as_bytes())?;

            buildup1.lock().unwrap().set_high()?;
            std::thread::sleep(std::time::Duration::from_millis(250));
            buildup2.lock().unwrap().set_high()?;
            std::thread::sleep(std::time::Duration::from_millis(250));
            buildup3.lock().unwrap().set_high()?;
            std::thread::sleep(std::time::Duration::from_millis(250));

            buildup1.lock().unwrap().set_low()?;
            buildup2.lock().unwrap().set_low()?;
            buildup3.lock().unwrap().set_low()?;

            laser_mosfet.lock().unwrap().set_high()?;
            std::thread::sleep(std::time::Duration::from_secs(1));
            laser_mosfet.lock().unwrap().set_low()?;

            Ok(())
        },
    )?;

    server.fn_handler(
        "/hyperspace",
        esp_idf_svc::http::Method::Get,
        move |request| -> core::result::Result<(), esp_idf_svc::hal::io::EspIOError> {
            let html = r#"
                <!DOCTYPE html>
                <html>
                    <head>
                        <meta charset="utf-8">
                        <meta http-equiv="refresh" content="0; url=/" />
                        <title>Star Destroyer</title>
                    </head>
                    <body>
                        <p><a href="/">Redirect</a></p>
                    </body>
                </html>"#;

            let mut response = request.into_ok_response()?;
            response.write_all(html.as_bytes())?;

            hyperspace_right.lock().unwrap().set_high()?;
            hyperspace_left.lock().unwrap().set_high()?;

            std::thread::sleep(std::time::Duration::from_secs(4));

            hyperspace_right.lock().unwrap().set_low()?;
            hyperspace_left.lock().unwrap().set_low()?;

            Ok(())
        },
    )?;

    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
