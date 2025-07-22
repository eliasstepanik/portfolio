use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    // Create a reactive signal for the counter
    let (count, set_count) = create_signal(0);

    view! {
        <div class="page home-page">
            <h1>"Welcome to Open Freedom Project"</h1>
            <p>"A modern web application built with Rust and Leptos"</p>

            <div class="feature-section">
                <h2>"Features"</h2>
                <ul>
                    <li>"🦀 Built with Rust for safety and performance"</li>
                    <li>"⚡ WebAssembly for near-native speed"</li>
                    <li>"🔄 Client-side routing with no page reloads"</li>
                    <li>"📱 Responsive design"</li>
                </ul>
            </div>

            <div class="counter-demo">
                <h2>"Interactive Demo"</h2>
                <p>"Click the button to see Leptos reactivity in action:"</p>
                <button
                    class="counter-button"
                    on:click=move |_| set_count.set(count.get() + 1)
                >
                    "Count: " {count}
                </button>
            </div>
        </div>
    }
}
