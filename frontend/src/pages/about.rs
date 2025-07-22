use leptos::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="page about-page">
            <h1>"About Open Freedom Project"</h1>

            <div class="content-section">
                <h2>"Our Mission"</h2>
                <p>
                    "The Open Freedom Project demonstrates the power of modern Rust web development. "
                    "We showcase how to build performant, type-safe web applications using cutting-edge technologies."
                </p>
            </div>

            <div class="content-section">
                <h2>"Technology Stack"</h2>
                <ul>
                    <li><strong>"Rust"</strong>" - Systems programming language focused on safety and performance"</li>
                    <li><strong>"Leptos"</strong>" - Full-stack web framework with fine-grained reactivity"</li>
                    <li><strong>"WebAssembly"</strong>" - Compile Rust to run in the browser at near-native speed"</li>
                    <li><strong>"Trunk"</strong>" - Build tool for Rust WASM applications"</li>
                </ul>
            </div>

            <div class="content-section">
                <h2>"Why Rust for Web?"</h2>
                <p>
                    "Rust brings memory safety, thread safety, and zero-cost abstractions to web development. "
                    "Combined with WebAssembly, it enables building web applications that are both fast and reliable."
                </p>
            </div>
        </div>
    }
}
