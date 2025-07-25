use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="page home-page">
            // Hero Section
            <section class="block block--desktop-first-visible block--mobile-first-visible">
                <div class="block-background block-background--fixed">
                    <img
                        src="https://images.unsplash.com/photo-1629904853716-f0bc54eea481?auto=format&fit=crop&w=3840"
                        alt="Developer Portfolio"
                        class="block-background__image block-background__image--fixed"
                    />
                    <div class="block-background__overlay block-background__overlay--fixed"></div>
                </div>

                <div class="block-layout-container">
                    <div class="block-layout hero-layout">
                        <div class="hero-title">
                            <h2>
                                "Building the Future with Code"
                            </h2>
                        </div>

                        <div class="hero-subtitle">
                            <p>
                                "Full-stack developer passionate about computer graphics, game development, and systems programming. Creating efficient, elegant solutions to complex problems."
                            </p>
                        </div>

                        <div class="hero-button">
                            <a href="/projects" class="grid-button grid-button--primary">
                                "View Projects"
                            </a>
                        </div>
                    </div>
                </div>
            </section>

            // Topics Section
            <section class="section">
                <div class="block-layout-container">
                    <div class="block-layout">
                        <div class="section-title">
                            <h2>"Core Expertise"</h2>
                        </div>
                        // Topic 1 - Systems Programming & Performance
                        <div class="topic-container">
                            <div class="topic-image">
                                <div class="image image--grid">
                                    <img
                                        src="/public/images/topic-1.jpg"
                                        alt="Systems Programming"
                                        class="image__image"
                                    />
                                </div>
                            </div>

                            <div class="topic-content">
                                <h5><strong>"Systems Programming & Performance"</strong></h5>
                                <p>
                                    "Building high-performance applications with Rust, focusing on memory safety, concurrency, and zero-cost abstractions. Experienced in optimizing algorithms and data structures for maximum efficiency."
                                </p>
                            </div>
                        </div>

                        // Topic 2 - Computer Graphics & Rendering
                        <div class="topic-container">
                            <div class="topic-content">
                                <h5><strong>"Computer Graphics & Rendering"</strong></h5>
                                <p>
                                    "Implementing ray tracing and physically-based rendering techniques. Passionate about creating visually stunning experiences through advanced graphics algorithms and GPU programming."
                                </p>
                            </div>

                            <div class="topic-image">
                                <div class="image image--grid">
                                    <img
                                        src="/public/images/topic-2.jpg"
                                        alt="Computer Graphics"
                                        class="image__image"
                                    />
                                </div>
                            </div>
                        </div>

                        // Topic 3 - Game Development & Simulation
                        <div class="topic-container">
                            <div class="topic-image">
                                <div class="image image--grid">
                                    <img
                                        src="/public/images/topic-3.jpg"
                                        alt="Game Development"
                                        class="image__image"
                                    />
                                </div>
                            </div>

                            <div class="topic-content">
                                <h5><strong>"Game Development & Simulation"</strong></h5>
                                <p>
                                    "Creating interactive experiences using modern game engines and frameworks. Specialized in physics simulations, procedural generation, and efficient entity-component systems."
                                </p>
                            </div>
                        </div>

                        // Topic 4 - Web Development & APIs
                        <div class="topic-container">
                            <div class="topic-content">
                                <h5><strong>"Web Development & APIs"</strong></h5>
                                <p>
                                    "Building modern web applications with Rust-based frameworks like Leptos and Axum. Experienced in creating responsive UIs, RESTful APIs, and real-time applications."
                                </p>
                            </div>

                            <div class="topic-image">
                                <div class="image image--grid">
                                    <img
                                        src="/public/images/topic-4.jpg"
                                        alt="Web Development"
                                        class="image__image"
                                    />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Contact CTA Section
            <section class="support-cta">
                <div class="block-background">
                    <img
                        src="/public/images/contact-bg.jpg"
                        alt="Get in Touch"
                        class="block-background__image"
                    />
                    <div class="block-background__overlay"></div>
                </div>

                <div class="block-layout-container">
                    <div class="support-layout">
                        <h3 class="text-white">
                            "Let's build something amazing together. I'm always open to discussing new projects and opportunities."
                        </h3>

                        <A href="/contact" class="grid-button grid-button--primary">
                            "Get in Touch"
                        </A>
                    </div>
                </div>
            </section>
        </div>
    }
}
