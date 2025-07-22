use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="footer-content">
                <p>"© 2024 Open Freedom Project. Built with Rust and Leptos."</p>
                <p class="footer-tech">
                    "Powered by WebAssembly"
                </p>
            </div>
        </footer>
    }
}
