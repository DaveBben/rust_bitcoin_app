slint::include_modules!();
mod crpyto_price;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            match crpyto_price::get_bitcoin_price() {
                Ok(price) => { 
                   let price = format!("{:.2}", price); // returns "1.24"
                   ui.set_price(slint::SharedString::from(price).clone());
                },
                Err(_) => {
                    println!("Failed to grab price");
                },
            }
           
        }
    });

    ui.run()
}
