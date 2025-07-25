use leptos::*;
use leptos_router::*;

use crate::components::{Footer, Header};
use crate::pages::{AboutPage, ContactPage, HomePage, ProjectsPage};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header/>
            <main class="main-content">
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/projects" view=ProjectsPage/>
                    <Route path="/contact" view=ContactPage/>
                </Routes>
            </main>
            <Footer/>
        </Router>
    }
}
