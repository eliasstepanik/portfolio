use leptos::*;

mod app;
mod components;
mod pages;

use app::App;

fn main() {
    // Set up console error panic hook for better error messages in the browser
    console_error_panic_hook::set_once();

    // Mount the App component to the body
    mount_to_body(|| view! { <App/> })
}
