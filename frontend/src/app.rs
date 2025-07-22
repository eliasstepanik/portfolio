use leptos::*;
use leptos_router::*;

mod components;
mod pages;

use components::{Footer, Header};
use pages::{AboutPage, ContactPage, HomePage};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header/>
            <main class="main-content">
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/contact" view=ContactPage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
