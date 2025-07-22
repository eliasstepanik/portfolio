use leptos::*;

#[component]
pub fn SupportPage() -> impl IntoView {
    view! {
        <div class="page support-page">
            // Hero section with background image
            <section class="block block--desktop-first-visible block--mobile-first-visible">
                <div class="block-background">
                    <img
                        src="/images/hero-support.jpg"
                        alt="two children holding hands in a field of tall grass"
                        class="block-background__image"
                    />
                    <div class="block-background__overlay"></div>
                </div>

                <div class="block-layout-container">
                    <div class="block-layout support-hero-layout">
                        <div class="layout-element text-center">
                            <div class="text-box">
                                <h1 class="text-white">"Coming Soon!"</h1>
                            </div>
                        </div>

                        <div class="layout-element text-center">
                            <div class="text-box">
                                <h2 class="text-white">"Your help means the world to us"</h2>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
