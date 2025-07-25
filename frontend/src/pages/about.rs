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
                        alt="About Elias Stepanik"
                        class="block-background__image block-background__image--fixed"
                    />
                    <div class="block-background__overlay block-background__overlay--fixed"></div>
                </div>

                <div class="block-layout-container">
                    <div class="hero-layout">
                        <div class="hero-title">
                            <h1 class="text-white">"About Me"</h1>
                        </div>

                        <div class="hero-subtitle">
                            <p class="text-white">
                                "I'm a systems programmer and computer graphics enthusiast passionate about building performant, elegant solutions to complex problems."
                            </p>
                            <p class="text-white section-spacing">
                                "My work spans from low-level graphics programming to distributed systems, always with a focus on performance, correctness, and developer experience."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Skills and Experience Section
            <section class="section">
                <div class="block-layout-container">
                    <div class="mission-vision-layout">
                        <div class="mission-box">
                            <h2>"Technical Skills"</h2>
                            <div class="section-spacing-lg">
                                <h4 class="skill-heading">"Languages"</h4>
                                <p>"Rust, C#, TypeScript, Python, GLSL/WGSL"</p>

                                <h4 class="skill-heading-spaced">"Graphics & Game Dev"</h4>
                                <p>"WebGPU, OpenGL, Vulkan, Bevy Engine, Unity"</p>

                                <h4 class="skill-heading-spaced">"Web Technologies"</h4>
                                <p>"Leptos, React, WebAssembly, REST APIs"</p>

                                <h4 class="skill-heading-spaced">"Tools & Systems"</h4>
                                <p>"Git, Docker, PostgreSQL, Linux, CI/CD"</p>
                            </div>
                        </div>

                        <div class="vision-box">
                            <h2>"What I Do"</h2>
                            <ul class="vision-list">
                                <li>"Build high-performance graphics applications"</li>
                                <li>"Create tools for game developers"</li>
                                <li>"Contribute to open-source projects"</li>
                                <li>"Explore new rendering techniques"</li>
                                <li>"Optimize systems for performance"</li>
                            </ul>
                        </div>
                    </div>

                    // Project Management Section
                    <div class="mission-vision-layout section-spacing-lg">
                        <div class="mission-box">
                            <h2>"Project Management"</h2>
                            <div class="section-spacing-lg">
                                <h4 class="skill-heading">"Methodologies"</h4>
                                <p>"Scrum, Agile, Kanban, Waterfall"</p>

                                <h4 class="skill-heading-spaced">"Experience"</h4>
                                <p>"Led cross-functional teams in software development projects using Scrum framework. Experienced in sprint planning, daily standups, retrospectives, and backlog management."</p>

                                <h4 class="skill-heading-spaced">"Tools"</h4>
                                <p>"Jira, Azure DevOps, GitHub Projects, Trello"</p>
                            </div>
                        </div>

                        <div class="vision-box">
                            <h2>"Leadership Skills"</h2>
                            <ul class="vision-list">
                                <li>"Facilitate effective sprint ceremonies"</li>
                                <li>"Coordinate between stakeholders and development teams"</li>
                                <li>"Remove blockers and optimize team velocity"</li>
                                <li>"Foster collaborative development culture"</li>
                                <li>"Balance technical debt with feature delivery"</li>
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
                        alt="Development philosophy"
                        class="block-background__image"
                    />
                    <div class="block-background__overlay"></div>
                </div>

                <div class="block-layout-container">
                    <div class="philosophy-container">
                        <div class="section-title">
                            <h2 class="text-white">"My Development Philosophy"</h2>
                            <p class="text-white philosophy-text">
                                "Great software is built on solid foundations, clear thinking, and attention to detail."
                            </p>
                            <p class="text-white section-spacing-lg">
                                "I believe in:"
                            </p>
                        </div>

                        <div class="philosophy-grid">
                            <div class="philosophy-item">
                                <h4>"Performance Matters"</h4>
                                <p>
                                    "Efficient code respects users' time and resources. Every millisecond counts."
                                </p>
                            </div>

                            <div class="philosophy-item">
                                <h4>"Simplicity Wins"</h4>
                                <p>
                                    "The best solution is often the simplest one that correctly solves the problem."
                                </p>
                            </div>

                            <div class="philosophy-item">
                                <h4>"Open Source"</h4>
                                <p>
                                    "Sharing knowledge and code makes everyone better. Collaboration drives innovation."
                                </p>
                            </div>

                            <div class="philosophy-item">
                                <h4>"Continuous Learning"</h4>
                                <p>
                                    "Technology evolves rapidly. Staying curious and adaptable is essential."
                                </p>
                            </div>

                            <div class="philosophy-item">
                                <h4>"Quality Over Quantity"</h4>
                                <p>
                                    "Well-crafted, maintainable code beats hasty solutions every time."
                                </p>
                            </div>
                        </div>

                        <div class="philosophy-closing">
                            <p class="text-white">
                                "Currently focused on pushing the boundaries of real-time rendering and building tools that empower developers to create amazing experiences."
                            </p>
                            <p class="text-white">"Always open to collaborating on interesting projects."</p>
                            <p class="text-white">"Let's build something great together."</p>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
