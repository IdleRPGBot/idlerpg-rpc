use discord_rich_presence::{activity, new_client, DiscordIpc};
use serde_json::json;

use std::{thread::sleep, time::Duration};

mod profile;

const CLIENT_ID: &str = "424606447867789312";
const REFRESH_INTERVAL: Duration = Duration::from_millis(5000);
pub static mut USER_ID: u64 = 0;

fn main() {
    let mut discord = new_client(CLIENT_ID).unwrap();
    println!("Connecting to Discord");
    discord.connect_ipc().unwrap();
    println!("Connected, sending handshake");
    discord
        .send(
            json!({
                "v": 1,
                "client_id": discord.get_client_id()
            }),
            0,
        )
        .unwrap();
    let (_, data) = discord.recv().unwrap();
    let id = data["data"]["user"]["id"]
        .as_str()
        .unwrap()
        .to_owned()
        .parse()
        .unwrap();

    unsafe {
        USER_ID = id;
    }

    loop {
        if unsafe { USER_ID } != 0 {
            let profile_data = profile::get_profile();
            let state = profile_data.get_state();
            let details = profile_data.get_details();
            let small_text = profile_data.get_small_image_text();
            let mut act = activity::Activity::new()
                .state(&state)
                .details(&details)
                .assets(
                    activity::Assets::new()
                        .large_image("logo")
                        .large_text(profile_data.get_big_image_text())
                        .small_image(profile_data.get_small_image())
                        .small_text(&small_text),
                );
            if let Some(ts) = profile_data.get_time() {
                act = act.timestamps(ts)
            }
            let _ = discord.set_activity(act);
        }

        sleep(REFRESH_INTERVAL);
    }
}
