use leptos::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="page about-page">
            // Hero Section
            <section class="block block--desktop-first-visible block--mobile-first-visible">
                <div class="block-background block-background--fixed">
                    <img
                        src="/public/images/hero-about.jpg"
                        alt="Open Freedom Project - About Us"
                        class="block-background__image block-background__image--fixed"
                    />
                    <div class="block-background__overlay block-background__overlay--fixed"></div>
                </div>

                <div class="block-layout-container">
                    <div class="hero-layout">
                        <div class="hero-title">
                            <h1 class="text-white">"Get to know us"</h1>
                        </div>

                        <div class="hero-subtitle">
                            <p class="text-white">
                                "We're a small, independent project committed to making essential ideas freely available — without ads, paywalls, or noise."
                            </p>
                            <p class="text-white" style="margin-top: 1rem;">
                                "At the heart of our work is a simple practice: read important texts out loud, as they are, and make them accessible to anyone who wants to listen. Nothing added. Nothing sold."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Mission and Vision Section
            <section class="section">
                <div class="block-layout-container">
                    <div class="mission-vision-layout">
                        <div class="mission-box">
                            <h2>"Our Mission"</h2>
                            <p>
                                "To make objective knowledge about internal and external freedom freely accessible to everyone — through readings of public-domain and openly licensed texts — using transparent, donation-based, and ad-free technology"
                            </p>
                        </div>

                        <div class="vision-box">
                            <h2>"Our Vision"</h2>
                            <ul class="vision-list">
                                <li>"A culture where knowledge flows freely, not behind gates."</li>
                                <li>"Where understanding — not opinion — shapes conversation."</li>
                                <li>"Where people give because they care, not because they're sold to."</li>
                                <li>"Where clarity is a common good, not a luxury."</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </section>

            // Philosophy Section
            <section class="philosophy-section">
                <div class="block-background">
                    <img
                        src="/public/images/philosophy-bg.jpg"
                        alt="Philosophy background"
                        class="block-background__image"
                    />
                    <div class="block-background__overlay"></div>
                </div>

                <div class="block-layout-container">
                    <div class="philosophy-container">
                        <div class="section-title">
                            <h2 class="text-white">"Our Philosophy"</h2>
                            <p class="text-white" style="font-size: 1.25rem; margin-top: 1rem;">
                                "We believe that genuine freedom begins with clarity — not just of thought, but of mind, ethics, and action."
                            </p>
                            <p class="text-white" style="margin-top: 2rem;">
                                "We grounded this project in 5 ideas:"
                            </p>
                        </div>

                        <div class="philosophy-grid">
                            <div class="philosophy-item">
                                <h4>"1. Inner Freedom"</h4>
                                <p>
                                    "Freedom from confusion, suffering, reactivity — is possible through direct understanding, not belief."
                                </p>
                            </div>

                            <div class="philosophy-item">
                                <h4>"2. Outer Freedom"</h4>
                                <p>
                                    "Freedom from oppression, domination, and structural harm — depends on knowledge, critical thought, and collective awareness."
                                </p>
                            </div>

                            <div class="philosophy-item">
                                <h4>"3. Generosity Is Viable"</h4>
                                <p>
                                    "We reject manipulation, monetization, and control in favor of voluntary support and trust."
                                </p>
                            </div>

                            <div class="philosophy-item">
                                <h4>"4. Technology Can Serve Liberation"</h4>
                                <p>
                                    "When used with care, transparency, and ethical intention."
                                </p>
                            </div>

                            <div class="philosophy-item">
                                <h4>"5. Truth Should Be Free"</h4>
                                <p>
                                    "Not hidden behind paywalls, status, or ideology."
                                </p>
                            </div>
                        </div>

                        <div class="philosophy-closing">
                            <p class="text-white">
                                "The Open Freedom Project is an attempt to live this worldview — by sharing real knowledge, not selling content."
                            </p>
                            <p class="text-white">"By being honest, not strategic."</p>
                            <p class="text-white">"By doing something good and letting it speak for itself."</p>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
