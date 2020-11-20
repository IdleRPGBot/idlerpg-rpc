use rustcord::{EventHandlers, RichPresenceBuilder, Rustcord, User};
use std::{io, thread::sleep, time::Duration};

mod profile;

const REFRESH_INTERVAL: Duration = Duration::from_millis(5000);
static mut USER_ID: String = String::new();

struct Handlers;

impl EventHandlers for Handlers {
    fn ready(this_user: User) {
        unsafe {
            USER_ID = this_user.user_id;
        }
    }
}

fn main() -> Result<(), io::Error> {
    let discord = Rustcord::init::<Handlers>("424606447867789312", true, None)?;

    loop {
        discord.run_callbacks();
        sleep(REFRESH_INTERVAL);
        unsafe {
            if !USER_ID.is_empty() {
                let profile_data = profile::get_profile(&USER_ID);
                let mut presence_builder = RichPresenceBuilder::new()
                    .state(&profile_data.get_state())
                    .details(&profile_data.get_details())
                    .large_image_key("logo")
                    .large_image_text(&profile_data.get_big_image_text())
                    .small_image_key(profile_data.get_small_image())
                    .small_image_text(&profile_data.get_small_image_text());
                if let Some(time) = profile_data.get_time() {
                    presence_builder = presence_builder.start_time(time.0).end_time(time.1);
                }
                let presence = presence_builder.build();
                discord.update_presence(presence)?;
            }
        }
    }
}
