#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let _guard = sentry::init(("https://bffc3804b28058c983a12fdce3406e0a@o1298580.ingest.us.sentry.io/4508499734757376", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
      }));
      
    app_lib::run();
}
