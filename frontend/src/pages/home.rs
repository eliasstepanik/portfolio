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
                        src="https://images.unsplash.com/photo-1673207005767-94da812b594c?auto=format&fit=crop&w=3840"
                        alt="Open Freedom Project"
                        class="block-background__image block-background__image--fixed"
                    />
                    <div class="block-background__overlay block-background__overlay--fixed"></div>
                </div>

                <div class="block-layout-container">
                    <div class="block-layout hero-layout">
                        <div class="hero-title">
                            <h2>
                                "Free Knowledge. Internal Clarity. External Freedom."
                            </h2>
                        </div>

                        <div class="hero-subtitle">
                            <p>
                                "Audiobook-style readings of public-domain and open-licensed texts on mindfulness, autonomy, and liberation — ad-free, donation-based, and accessible to all."
                            </p>
                        </div>

                        <div class="hero-button">
                            <a href="#" class="grid-button grid-button--primary">
                                "Listen"
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
                            <h2>"Topics We Are Focusing On"</h2>
                        </div>
                        // Topic 1 - Mindfulness & Inner Clarity
                        <div class="topic-container">
                            <div class="topic-image">
                                <div class="image image--grid">
                                    <img
                                        src="/public/images/topic-1.jpg"
                                        alt="Mindfulness & Inner Clarity"
                                        class="image__image"
                                    />
                                </div>
                            </div>

                            <div class="topic-content">
                                <h5><strong>"Mindfulness & Inner Clarity"</strong></h5>
                                <p>
                                    "Exploring practices and teachings that help us see clearly, respond wisely, and live with less suffering. Through breath, attention, and reflection, we investigate what it means to be mentally free, not caught in reactivity or confusion."
                                </p>
                            </div>
                        </div>

                        // Topic 2 - Autonomy & Social Freedom
                        <div class="topic-container">
                            <div class="topic-content">
                                <h5><strong>"Autonomy & Social Freedom"</strong></h5>
                                <p>
                                    "Understanding how social, political, and cultural systems shape our lives — and how to reclaim agency without reproducing harm. From civil liberties to mutual aid, we highlight ideas that question power and open paths to collective liberation."
                                </p>
                            </div>
                            
                            <div class="topic-image">
                                <div class="image image--grid">
                                    <img
                                        src="/public/images/topic-2.jpg"
                                        alt="Autonomy & Social Freedom"
                                        class="image__image"
                                    />
                                </div>
                            </div>
                        </div>

                        // Topic 3 - Ecology & Interdependence
                        <div class="topic-container">
                            <div class="topic-image">
                                <div class="image image--grid">
                                    <img
                                        src="/public/images/topic-3.jpg"
                                        alt="Ecology & Interdependence"
                                        class="image__image"
                                    />
                                </div>
                            </div>

                            <div class="topic-content">
                                <h5><strong>"Ecology & Interdependence"</strong></h5>
                                <p>
                                    "Freedom doesn't mean separation — it means living in conscious connection with nature and others. We explore ecological thinking and practices that promote balance, sustainability, and a sense of belonging in the living world."
                                </p>
                            </div>
                        </div>

                        // Topic 4 - Critical Thought & Objectivity
                        <div class="topic-container">
                            <div class="topic-content">
                                <h5><strong>"Critical Thought & Objectivity"</strong></h5>
                                <p>
                                    "Freedom requires thinking clearly and questioning the lenses we've inherited. We focus on writers and thinkers who help us challenge assumptions, understand systems, and seek truth without dogma."
                                </p>
                            </div>
                            
                            <div class="topic-image">
                                <div class="image image--grid">
                                    <img
                                        src="/public/images/topic-4.jpg"
                                        alt="Critical Thought & Objectivity"
                                        class="image__image"
                                    />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Support CTA Section
            <section class="support-cta">
                <div class="block-background">
                    <img
                        src="/public/images/support-bg.jpg"
                        alt="Support the Open Freedom Project"
                        class="block-background__image"
                    />
                    <div class="block-background__overlay"></div>
                </div>

                <div class="block-layout-container">
                    <div class="support-layout">
                        <h3 class="text-white">
                            "Your support keeps this project ad-free, open, and accessible — for everyone, forever."
                        </h3>

                        <A href="/support" class="grid-button grid-button--primary">
                            "Support Us"
                        </A>
                    </div>
                </div>
            </section>
        </div>
    }
}