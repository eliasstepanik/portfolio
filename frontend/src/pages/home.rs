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
                        src="/images/hero-home.jpg"
                        alt="a large rock in the middle of a forest"
                        class="block-background__image block-background__image--fixed"
                    />
                    <div class="block-background__overlay block-background__overlay--fixed"></div>
                </div>

                <div class="block-layout-container">
                    <div class="block-layout hero-layout">
                        <div class="layout-element hero-title">
                            <div class="text-box">
                                <h2 class="text-white">
                                    "Free Knowledge. Internal Clarity. External Freedom."
                                </h2>
                            </div>
                        </div>

                        <div class="layout-element hero-subtitle">
                            <div class="text-box">
                                <p class="body-large text-white">
                                    "Audiobook-style readings of public-domain and open-licensed texts on mindfulness, autonomy, and liberation — ad-free, donation-based, and accessible to all."
                                </p>
                            </div>
                        </div>

                        <div class="layout-element hero-button">
                            <a href="#" class="grid-button grid-button--primary">
                                "Listen"
                            </a>
                        </div>
                    </div>
                </div>
            </section>

            // Topics Section
            <section class="block section">
                <div class="block-background">
                    // White background handled by CSS
                </div>

                <div class="block-layout-container">
                    <div class="block-layout">
                        <div class="layout-element text-center section-title">
                            <div class="text-box">
                                <h2>"Topics We Are Focusing On"</h2>
                            </div>
                        </div>

                        // Topic 1 - Mindfulness & Inner Clarity
                        <div class="layout-element topic-image topic-image-1">
                            <div class="image image--grid">
                                <img
                                    src="/images/topic-1.jpg"
                                    alt="topless man wearing black beaded necklace"
                                    class="image__image"
                                />
                            </div>
                        </div>

                        <div class="layout-element topic-content topic-content-1">
                            <div class="text-box">
                                <h5><strong>"Mindfulness & Inner Clarity"</strong></h5>
                                <p class="body-large">
                                    "Exploring practices and teachings that help us see clearly, respond wisely, and live with less suffering. Through breath, attention, and reflection, we investigate what it means to be mentally free, not caught in reactivity or confusion."
                                </p>
                            </div>
                        </div>

                        // Topic 2 - Autonomy & Social Freedom
                        <div class="layout-element topic-image topic-image-2">
                            <div class="image image--grid">
                                <img
                                    src="/images/topic-2.jpg"
                                    alt="selective focus photography of woman holding yellow petaled flowers"
                                    class="image__image"
                                />
                            </div>
                        </div>

                        <div class="layout-element topic-content topic-content-2">
                            <div class="text-box">
                                <h5><strong>"Autonomy & Social Freedom"</strong></h5>
                                <p class="body-large">
                                    "Understanding how social, political, and cultural systems shape our lives — and how to reclaim agency without reproducing harm. From civil liberties to mutual aid, we highlight ideas that question power and open paths to collective liberation."
                                </p>
                            </div>
                        </div>

                        // Topic 3 - Ecology & Interdependence
                        <div class="layout-element topic-image topic-image-3">
                            <div class="image image--grid">
                                <img
                                    src="/images/topic-3.jpg"
                                    alt="a couple of ladybugs sitting on top of a green leaf"
                                    class="image__image"
                                />
                            </div>
                        </div>

                        <div class="layout-element topic-content topic-content-3">
                            <div class="text-box">
                                <h5><strong>"Ecology & Interdependence"</strong></h5>
                                <p class="body-large">
                                    "Freedom doesn't mean separation — it means living in conscious connection with nature and others. We explore ecological thinking and practices that promote balance, sustainability, and a sense of belonging in the living world."
                                </p>
                            </div>
                        </div>

                        // Topic 4 - Critical Thought & Objectivity
                        <div class="layout-element topic-image topic-image-4">
                            <div class="image image--grid">
                                <img
                                    src="/images/topic-4.jpg"
                                    alt="water falling from clear glass ball"
                                    class="image__image"
                                />
                            </div>
                        </div>

                        <div class="layout-element topic-content topic-content-4">
                            <div class="text-box">
                                <h5><strong>"Critical Thought & Objectivity"</strong></h5>
                                <p class="body-large">
                                    "Freedom requires thinking clearly and questioning the lenses we've inherited. We focus on writers and thinkers who help us challenge assumptions, understand systems, and seek truth without dogma."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Support CTA Section
            <section class="block section support-cta">
                <div class="block-background">
                    <img
                        src="/images/support-bg.jpg"
                        alt="a person hugging a tree in a forest"
                        class="block-background__image"
                    />
                    <div class="block-background__overlay"></div>
                </div>

                <div class="block-layout-container">
                    <div class="block-layout support-layout">
                        <div class="layout-element text-center">
                            <div class="text-box">
                                <h3 class="text-white">
                                    "Your support keeps this project ad-free, open, and accessible — for everyone, forever."
                                </h3>
                            </div>
                        </div>

                        <div class="layout-element text-center">
                            <A href="/support" class="grid-button grid-button--primary">
                                "Support Us"
                            </A>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
