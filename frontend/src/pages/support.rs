use leptos::*;

#[component]
pub fn SupportPage() -> impl IntoView {
    view! {
        <div class="page support-page">
            // Hero section with background image
            <section class="block block--desktop-first-visible block--mobile-first-visible">
                <div class="block-background block-background--fixed">
                    <img
                        src="/public/images/hero-support.jpg"
                        alt="Support the Open Freedom Project"
                        class="block-background__image block-background__image--fixed"
                    />
                    <div class="block-background__overlay block-background__overlay--fixed"></div>
                </div>

                <div class="block-layout-container">
                    <div class="hero-layout">
                        <div class="hero-title">
                            <h1 class="text-white">"Coming Soon!"</h1>
                        </div>

                        <div class="hero-subtitle">
                            <h2 class="text-white">"Your help means the world to us"</h2>
                        </div>
                        
                        <div style="margin-top: 3rem;">
                            <p class="text-white" style="font-size: 1.25rem; max-width: 600px; margin: 0 auto;">
                                "We're currently setting up our donation system to ensure it's transparent, secure, and easy to use. Check back soon!"
                            </p>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}