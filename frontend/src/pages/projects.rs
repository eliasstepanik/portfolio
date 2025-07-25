use gloo_net::http::Request;
use leptos::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub github_url: String,
    pub demo_url: Option<String>,
    pub technologies: Vec<String>,
    pub primary_language: String,
    pub stars_count: i32,
    pub featured: bool,
    pub image_url: Option<String>,
}

async fn fetch_projects(filter: Option<String>) -> Result<Vec<Project>, String> {
    let mut url = "http://localhost:3000/api/projects".to_string();
    if let Some(lang) = filter {
        url.push_str(&format!("?language={lang}"));
    }

    Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Failed to parse JSON: {e}"))
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let (language_filter, set_language_filter) = create_signal(None::<String>);

    let projects = create_resource(
        move || language_filter.get(),
        |filter| async move { fetch_projects(filter).await },
    );

    let languages = ["All", "Rust", "C#", "TypeScript", "Python"];

    view! {
        <div class="page projects-page">
            // Hero Section
            <section class="block block--desktop-first-visible block--mobile-first-visible">
                <div class="block-background block-background--fixed">
                    <img
                        src="/public/images/hero-projects.jpg"
                        alt="Projects Portfolio"
                        class="block-background__image block-background__image--fixed"
                    />
                    <div class="block-background__overlay block-background__overlay--fixed"></div>
                </div>

                <div class="block-layout-container">
                    <div class="hero-layout">
                        <div class="hero-title">
                            <h1 class="text-white">"Projects & Experiments"</h1>
                        </div>

                        <div class="hero-subtitle">
                            <p class="text-white">
                                "Exploring computer graphics, game development, and systems programming through open-source projects."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Projects Grid Section
            <section class="section">
                <div class="block-layout-container">
                    <div class="section-title">
                        <h2>"Featured Projects"</h2>
                    </div>

                    // Language Filter
                    <div class="filter-container">
                        {languages.iter().map(|&lang| {
                            view! {
                                <button
                                    class=move || {
                                        let active = {
                                            let current = language_filter.get();
                                            (lang == "All" && current.is_none()) ||
                                            current.as_deref() == Some(lang)
                                        };
                                        if active {
                                            "filter-button filter-button--active"
                                        } else {
                                            "filter-button"
                                        }
                                    }
                                    on:click=move |_| {
                                        if lang == "All" {
                                            set_language_filter.set(None);
                                        } else {
                                            set_language_filter.set(Some(lang.to_string()));
                                        }
                                    }
                                >
                                    {lang}
                                </button>
                            }
                        }).collect_view()}
                    </div>

                    <Suspense fallback=move || view! {
                        <div class="loading-message">
                            <p class="loading-text">"Loading projects..."</p>
                        </div>
                    }>
                        {move || {
                            projects.get().map(|projects| match projects {
                                Ok(projects) => view! {
                                    <div class="philosophy-grid">
                                        <For
                                            each=move || projects.clone()
                                            key=|project| project.id
                                            children=move |project| view! {
                                                <div class="project-card philosophy-item">
                                                    {project.image_url.as_ref().map(|url| view! {
                                                        <div class="project-image">
                                                            <img
                                                                src=url.clone()
                                                                alt=project.name.clone()
                                                            />
                                                        </div>
                                                    })}

                                                    <div class="project-header">
                                                        <h4>{project.name.clone()}</h4>
                                                        {project.featured.then(|| view! {
                                                            <span class="featured-badge">
                                                                "Featured"
                                                            </span>
                                                        })}
                                                    </div>

                                                    <p class="project-description">
                                                        {project.description.clone()}
                                                    </p>

                                                    <div class="tech-stack">
                                                        {project.technologies.iter().map(|tech| view! {
                                                            <span class="tech-badge">
                                                                {tech}
                                                            </span>
                                                        }).collect_view()}
                                                    </div>

                                                    <div class="project-stats">
                                                        <span>
                                                            "⭐ " {project.stars_count}
                                                        </span>
                                                        <span>{project.primary_language.clone()}</span>
                                                    </div>

                                                    <div class="project-links">
                                                        <a href=project.github_url.clone()
                                                           target="_blank"
                                                           class="grid-button grid-button--secondary">
                                                            "View on GitHub"
                                                        </a>
                                                        {project.demo_url.as_ref().map(|url| view! {
                                                            <a href=url.clone()
                                                               target="_blank"
                                                               class="grid-button grid-button--primary">
                                                                "Live Demo"
                                                            </a>
                                                        })}
                                                    </div>
                                                </div>
                                            }
                                        />
                                    </div>
                                },
                                Err(e) => view! {
                                    <div class="error-message">
                                        <p class="error-text">"Error loading projects"</p>
                                        <p class="error-details">{e.to_string()}</p>
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
                        src="/public/images/contact-bg.jpg"
                        alt="Get in Touch"
                        class="block-background__image"
                    />
                    <div class="block-background__overlay"></div>
                </div>

                <div class="block-layout-container">
                    <div class="support-layout">
                        <h3 class="text-white">
                            "Interested in collaborating or have questions about my work?"
                        </h3>

                        <a href="/contact" class="grid-button grid-button--primary">
                            "Get in Touch"
                        </a>
                    </div>
                </div>
            </section>
        </div>
    }
}
