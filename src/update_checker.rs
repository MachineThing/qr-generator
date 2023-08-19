use gdk4::prelude::*;
use gtk4::AlertDialog;
use gtk4::gio::Cancellable;

pub fn check_for_updates(parent: Option<&impl IsA<gtk4::Window>>) {
    println!("Getting updates...");
    let version = "\"v".to_owned()+option_env!("CARGO_PKG_VERSION").unwrap()+"\"";
    
    let client = reqwest::blocking::Client::builder()
        .user_agent("Machinething-Update-Check")
        .build()
        .expect("Failed to build HTTP client");

    let body = client.get("https://api.github.com/repos/MachineThing/qr-generator/releases/latest")
        .send();
    
    match body {
        Ok(result) => {
            let gh_version = &result.json::<serde_json::Value>().unwrap()["tag_name"];
            if gh_version.to_string() != version {
                // Not up to date!
                let alert = AlertDialog::builder()
                    .message("QR Code Generator is not up to date")
                    .detail(format!("You can download the newest version ({}) at GitHub by clicking on the update button.", gh_version.to_string()))
                    .buttons(["Ignore", "Update"])
                    .cancel_button(0)
                    .default_button(1)
                    .modal(true)
                    .build();
                
                alert.choose(parent, Cancellable::NONE, |dialog| {
                    if dialog.unwrap() == 1 {
                        // User wants to update!
                        println!("Going to github");
                        let _ = webbrowser::open("https://github.com/MachineThing/qr-generator/releases/latest");
                    }
                });

                println!("QrGen is not up to date! {} v {}", gh_version.to_string(), version);
            } else {
                println!("QrGen is up to date!");
            }
        },
        Err(_) => {
            println!("Cannot check for updates at this time. Internet is probably down.");
        }
    }
}