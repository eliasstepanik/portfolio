use leptos::*;
use leptos_router::*;

use crate::components::{Footer, Header};
use crate::pages::{AboutPage, ContactPage, GalleryPage, HomePage, SupportPage};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header/>
            <main class="main-content">
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/gallery" view=GalleryPage/>
                    <Route path="/support" view=SupportPage/>
                    <Route path="/contact" view=ContactPage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
