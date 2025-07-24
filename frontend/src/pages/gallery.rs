use leptos::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AudioBook {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub duration: String,
    pub audio_url: String,
    pub cover_url: String,
    pub description: String,
}

fn get_placeholder_books() -> Vec<AudioBook> {
    vec![
        AudioBook {
            id: 1,
            title: "Meditations".to_string(),
            author: "Marcus Aurelius".to_string(),
            duration: "3:45:00".to_string(),
            audio_url: "/public/audio/meditations.mp3".to_string(),
            cover_url: "/images/book-cover-1.jpg".to_string(),
            description: "Stoic philosophy and personal reflections from the Roman Emperor"
                .to_string(),
        },
        AudioBook {
            id: 2,
            title: "The Art of War".to_string(),
            author: "Sun Tzu".to_string(),
            duration: "2:30:00".to_string(),
            audio_url: "/public/audio/art-of-war.mp3".to_string(),
            cover_url: "/images/book-cover-2.jpg".to_string(),
            description: "Ancient Chinese military treatise on strategy and tactics".to_string(),
        },
        AudioBook {
            id: 3,
            title: "The Republic".to_string(),
            author: "Plato".to_string(),
            duration: "11:00:00".to_string(),
            audio_url: "/public/audio/republic.mp3".to_string(),
            cover_url: "/images/book-cover-3.jpg".to_string(),
            description: "Philosophical dialogue concerning justice and the ideal state"
                .to_string(),
        },
        AudioBook {
            id: 4,
            title: "The Prince".to_string(),
            author: "Niccolò Machiavelli".to_string(),
            duration: "4:15:00".to_string(),
            audio_url: "/public/audio/prince.mp3".to_string(),
            cover_url: "/images/book-cover-4.jpg".to_string(),
            description: "Political treatise on power, leadership, and statecraft".to_string(),
        },
        AudioBook {
            id: 5,
            title: "Walden".to_string(),
            author: "Henry David Thoreau".to_string(),
            duration: "7:30:00".to_string(),
            audio_url: "/public/audio/walden.mp3".to_string(),
            cover_url: "/images/book-cover-5.jpg".to_string(),
            description: "Reflections on simple living in natural surroundings".to_string(),
        },
        AudioBook {
            id: 6,
            title: "On Liberty".to_string(),
            author: "John Stuart Mill".to_string(),
            duration: "5:00:00".to_string(),
            audio_url: "/public/audio/on-liberty.mp3".to_string(),
            cover_url: "/images/book-cover-6.jpg".to_string(),
            description: "Philosophical work on the nature and limits of power over the individual"
                .to_string(),
        },
    ]
}

#[component]
pub fn GalleryPage() -> impl IntoView {
    // Use create_resource for CSR async data loading
    let audio_books = create_resource(
        || (),
        |_| async {
            // Simulate API delay
            std::future::ready(()).await;
            get_placeholder_books()
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
                            audio_books.get().map(|books| view! {
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
