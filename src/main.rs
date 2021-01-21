use discord_game_sdk::{Activity, Discord, EventHandler};
use std::{thread::sleep, time::Duration};

mod profile;

const CLIENT_ID: i64 = 424606447867789312;
const REFRESH_INTERVAL: Duration = Duration::from_millis(5000);
static mut USER_ID: i64 = 0;

#[derive(Default)]
struct Handler;

impl EventHandler for Handler {
    fn on_current_user_update(&mut self, discord: &Discord<'_, Self>) {
        println!("[UPDATER] Got data from Discord");
        unsafe {
            USER_ID = discord.current_user().unwrap().id();
        }
    }
}

struct IdleRPGRPC<'a> {
    pub discord: Discord<'a, Handler>,
}

impl IdleRPGRPC<'_> {
    fn new() -> Self {
        let mut discord = Discord::<Handler>::new(CLIENT_ID).expect("Error creating Game SDK");
        *discord.event_handler_mut() = Some(Handler::default());
        Self { discord }
    }

    fn get_loading_activity(&self) -> Activity {
        let mut activity_builder = Activity::empty();
        activity_builder.with_state("Loading data...");
        activity_builder.with_large_image_key("logo");
        activity_builder.with_large_image_tooltip("Boo!");
        activity_builder
    }

    fn get_updated_activity(&self) -> Activity {
        let profile_data = profile::get_profile(unsafe { USER_ID });
        let mut activity_builder = Activity::empty();
        activity_builder.with_state(&profile_data.get_state());
        activity_builder.with_details(&profile_data.get_details());
        activity_builder.with_large_image_key("logo");
        activity_builder.with_large_image_tooltip(&profile_data.get_big_image_text());
        activity_builder.with_small_image_key(profile_data.get_small_image());
        activity_builder.with_small_image_tooltip(&profile_data.get_small_image_text());
        if let Some(time) = profile_data.get_time() {
            activity_builder.with_start_time(time.0);
            activity_builder.with_end_time(time.1);
        }
        activity_builder
    }

    fn main_loop(&mut self) {
        let first = self.get_loading_activity();
        self.discord.update_activity(&first, |_, result| {
            if let Err(error) = result {
                eprintln!("[LOOP] Failed to update activity: {}", error);
            }
        });
        println!("[LOOP] Waiting to get data from Discord...");
        loop {
            self.discord.run_callbacks().unwrap();
            sleep(REFRESH_INTERVAL);
            let user_id = unsafe { USER_ID };
            if user_id != 0 {
                let activity = self.get_updated_activity();
                self.discord.update_activity(&activity, |_, result| {
                    if let Err(error) = result {
                        eprintln!("[LOOP] Failed to update activity: {}", error);
                    }
                });
                println!("[LOOP] Updated presence");
            }
        }
    }
}

fn main() {
    println!("[MAIN] Starting...");
    let mut app = IdleRPGRPC::new();
    app.main_loop();
}
