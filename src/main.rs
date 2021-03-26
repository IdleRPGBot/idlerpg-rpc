use discord_rpc_client::{models::Activity, Client};

use std::{thread::sleep, time::Duration};

mod profile;

const CLIENT_ID: u64 = 424606447867789312;
const REFRESH_INTERVAL: Duration = Duration::from_millis(5000);
pub static mut USER_ID: u64 = 0;

fn set_updated_activity(activity: Activity) -> Activity {
    let profile_data = profile::get_profile();
    let mut act = activity
        .state(&profile_data.get_state())
        .details(&profile_data.get_details())
        .assets(|ass| {
            ass.large_image("logo")
                .large_text(profile_data.get_big_image_text())
                .small_image(profile_data.get_small_image())
                .small_text(&profile_data.get_small_image_text())
        });
    if let Some(ts) = profile_data.get_time() {
        act = act.timestamps(|_| ts)
    }
    act
}

fn main() {
    let mut discord = Client::new(CLIENT_ID);
    discord.start();
    unsafe {
        USER_ID = discord
            .ready_data()
            .unwrap()
            .get_user()
            .unwrap()
            .get_id()
            .unwrap()
            .parse()
            .unwrap()
    };

    sleep(REFRESH_INTERVAL);

    loop {
        sleep(REFRESH_INTERVAL);

        if unsafe { USER_ID } != 0 {
            let _ = discord.set_activity(set_updated_activity);
        }
    }
}
