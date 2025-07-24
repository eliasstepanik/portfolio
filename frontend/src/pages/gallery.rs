use leptos::*;
use gloo_net::http::Request;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AudioBook {
    pub id: i32,  // Changed to match backend
    pub title: String,
    pub author: String,
    pub duration: String,
    pub audio_url: String,
    pub cover_url: String,
    pub description: String,
}

async fn fetch_audiobooks() -> Result<Vec<AudioBook>, String> {
    Request::get("http://localhost:3000/api/audiobooks")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch: {}", e))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {}", e))
}

#[component]
pub fn GalleryPage() -> impl IntoView {
    // Use create_resource for CSR async data loading
    let audio_books = create_resource(
        || (),
        |_| async {
            fetch_audiobooks().await
        },
    );

    view! {
        <div class="page gallery-page">
            // Hero Section
            <section class="block block--desktop-first-visible block--mobile-first-visible">
                <div class="block-background block-background--fixed">
                    <img
                        src="/public/images/hero-gallery.jpg"
                        alt="Audio Book Gallery"
                        class="block-background__image block-background__image--fixed"
                    />
                    <div class="block-background__overlay block-background__overlay--fixed"></div>
                </div>

                <div class="block-layout-container">
                    <div class="hero-layout">
                        <div class="hero-title">
                            <h1 class="text-white">"Audio Book Gallery"</h1>
                        </div>

                        <div class="hero-subtitle">
                            <p class="text-white">
                                "Listen to audiobook-style readings of public-domain and open-licensed texts on mindfulness, autonomy, and liberation."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Audio Books Grid Section
            <section class="section">
                <div class="block-layout-container">
                    <div class="section-title">
                        <h2>"Available Audio Books"</h2>
                    </div>

                    <Suspense fallback=move || view! {
                        <div style="text-align: center; padding: 40px;">
                            <p style="font-size: 1.2rem; color: #666;">"Loading audio books..."</p>
                        </div>
                    }>
                        {move || {
                            audio_books.get().map(|books| match books {
                                Ok(books) => view! {
                                    <div class="philosophy-grid">
                                        <For
                                            each=move || books.clone()
                                            key=|book| book.id
                                            children=move |book| view! {
                                            <div class="audio-book-item philosophy-item">
                                                <div class="audio-book-cover" style="margin-bottom: 20px;">
                                                    <img
                                                        src=book.cover_url.clone()
                                                        alt=book.title.clone()
                                                        style="width: 100%; height: 300px; object-fit: cover; border-radius: 10px; box-shadow: 0 4px 8px rgba(0,0,0,0.1);"
                                                    />
                                                </div>
                                                <h4>{book.title.clone()}</h4>
                                                <p style="font-weight: 600; color: #666; margin-bottom: 10px;">
                                                    "by "{book.author.clone()}
                                                </p>
                                                <p style="color: #888; font-size: 0.9rem; margin-bottom: 15px;">
                                                    "Duration: "{book.duration.clone()}
                                                </p>
                                                <p style="margin-bottom: 20px;">{book.description.clone()}</p>
                                                <audio controls=true style="width: 100%;">
                                                    <source src=book.audio_url.clone() type="audio/mpeg"/>
                                                    "Your browser does not support the audio element."
                                                </audio>
                                            </div>
                                        }
                                    />
                                    </div>
                                },
                                Err(e) => view! {
                                    <div style="text-align: center; padding: 40px;">
                                        <p style="font-size: 1.2rem; color: #d32f2f;">"Error loading audio books"</p>
                                        <p style="color: #666;">{e.to_string()}</p>
                                    </div>
                                }
                            })
                        }}
                    </Suspense>
                </div>
            </section>

            // Call to Action Section
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
                            "Enjoy these free audio books? Your support helps us create more."
                        </h3>

                        <a href="/support" class="grid-button grid-button--primary">
                            "Support Us"
                        </a>
                    </div>
                </div>
            </section>
        </div>
    }
}
