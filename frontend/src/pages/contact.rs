use leptos::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="page contact-page">
            <h1>"Contact Us"</h1>

            <div class="content-section">
                <p>"We'd love to hear from you! Get in touch through any of the following channels:"</p>

                <div class="contact-info">
                    <h2>"Connect With Us"</h2>
                    <ul>
                        <li>
                            <strong>"GitHub: "</strong>
                            <a href="https://github.com/eliasstepanik" target="_blank" rel="noopener noreferrer">
                                "github.com/eliasstepanik"
                            </a>
                        </li>
                        <li>
                            <strong>"Email: "</strong>
                            <a href="mailto:eliasstepanik@proton.me">
                                "eliasstepanik@proton.me"
                            </a>
                        </li>
                    </ul>
                </div>

                <div class="contribution-section">
                    <h2>"Contributing"</h2>
                    <p>
                        "This is an open-source project. We welcome contributions, bug reports, and feature requests. "
                        "Feel free to open issues or submit pull requests on our GitHub repository."
                    </p>
                </div>
            </div>
        </div>
    }
}
