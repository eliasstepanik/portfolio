use leptos::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="page about-page">
            // Hero Section
            <section class="block block--desktop-first-visible block--mobile-first-visible">
                <div class="block-background">
                    <img
                        src="/images/hero-about.jpg"
                        alt="waterfall in forest"
                        class="block-background__image"
                    />
                    <div class="block-background__overlay"></div>
                </div>

                <div class="block-layout-container">
                    <div class="block-layout about-hero-layout">
                        <div class="layout-element text-center">
                            <div class="text-box">
                                <h1 class="text-white">"Get to know us"</h1>
                            </div>
                        </div>

                        <div class="layout-element text-center">
                            <div class="text-box">
                                <p class="body-large text-white">
                                    "We're a small, independent project committed to making essential ideas freely available — without ads, paywalls, or noise."
                                </p>
                                <br />
                                <p class="body-large text-white">
                                    "At the heart of our work is a simple practice: read important texts out loud, as they are, and make them accessible to anyone who wants to listen. Nothing added. Nothing sold."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Mission and Vision Section
            <section class="block section">
                <div class="block-background">
                    // White background
                </div>

                <div class="block-layout-container">
                    <div class="block-layout mission-vision-layout">
                        <div class="layout-element mission-section">
                            <div class="text-box">
                                <h2>"Our Mission"</h2>
                                <p class="body-large">
                                    "To make objective knowledge about internal and external freedom freely accessible to everyone — through readings of public-domain and openly licensed texts — using transparent, donation-based, and ad-free technology"
                                </p>
                            </div>
                        </div>

                        <div class="layout-element vision-section">
                            <div class="text-box">
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
                </div>
            </section>

            // Philosophy Section
            <section class="block section philosophy-section">
                <div class="block-background">
                    <img
                        src="/images/philosophy-bg.jpg"
                        alt="man standing on fence overlooking mountain range"
                        class="block-background__image"
                    />
                    <div class="block-background__overlay"></div>
                </div>

                <div class="block-layout-container">
                    <div class="block-layout philosophy-layout">
                        <div class="layout-element text-center">
                            <div class="text-box">
                                <h2 class="text-white">"Our Philosophy"</h2>
                                <p class="body-large text-white">
                                    "We believe that genuine freedom begins with clarity — not just of thought, but of mind, ethics, and action."
                                </p>
                                <p class="text-white philosophy-subtitle">
                                    "We grounded this project in 5 ideas:"
                                </p>
                            </div>
                        </div>

                        <div class="layout-element philosophy-grid">
                            <div class="philosophy-item">
                                <div class="text-box">
                                    <p class="text-white">
                                        <strong>"1. Inner Freedom"</strong>" — freedom from confusion, suffering, reactivity — is possible through direct understanding, not belief."
                                    </p>
                                </div>
                            </div>

                            <div class="philosophy-item">
                                <div class="text-box">
                                    <p class="text-white">
                                        <strong>"2. Outer Freedom"</strong>" — freedom from oppression, domination, and structural harm — depends on knowledge, critical thought, and collective awareness."
                                    </p>
                                </div>
                            </div>

                            <div class="philosophy-item">
                                <div class="text-box">
                                    <p class="text-white">
                                        <strong>"3. Generosity Is Viable"</strong>" — we reject manipulation, monetization, and control in favor of voluntary support and trust."
                                    </p>
                                </div>
                            </div>

                            <div class="philosophy-item">
                                <div class="text-box">
                                    <p class="text-white">
                                        <strong>"4. Technology Can Serve Liberation"</strong>" — when used with care, transparency, and ethical intention."
                                    </p>
                                </div>
                            </div>

                            <div class="philosophy-item">
                                <div class="text-box">
                                    <p class="text-white">
                                        <strong>"5. Truth Should Be Free"</strong>" — not hidden behind paywalls, status, or ideology."
                                    </p>
                                </div>
                            </div>
                        </div>

                        <div class="layout-element text-center philosophy-closing">
                            <div class="text-box">
                                <p class="text-white">
                                    "The " <strong>"Open Freedom Project"</strong> " is an attempt to live this worldview — by sharing real knowledge, not selling content."
                                </p>
                                <p class="text-white">
                                    "By being honest, not strategic."
                                </p>
                                <p class="text-white">
                                    "By doing something good and letting it speak for itself."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
