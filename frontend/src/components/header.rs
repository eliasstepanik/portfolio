use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="site-header">
            <nav class="navigation">
                <div class="nav-brand">
                    <A href="/" class="logo">
                        "Open Freedom Project"
                    </A>
                </div>
                <ul class="nav-links">
                    <li>
                        <A href="/" class="nav-link">
                            "Home"
                        </A>
                    </li>
                    <li>
                        <A href="/about" class="nav-link">
                            "About"
                        </A>
                    </li>
                    <li>
                        <A href="/contact" class="nav-link">
                            "Contact"
                        </A>
                    </li>
                </ul>
            </nav>
        </header>
    }
}
