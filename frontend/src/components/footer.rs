use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <section class="block footer">
            <div class="block-background">
                // Dark background is handled by CSS
            </div>

            <div class="block-layout-container">
                <div class="block-layout footer-layout">
                    // "Open" text
                    <div class="layout-element footer-text-left">
                        <div class="text-box">
                            <h4 class="text-white">"Open"</h4>
                        </div>
                    </div>

                    // Social icons
                    <div class="layout-element footer-social">
                        <div class="social-icons">
                            <a
                                href="https://twitter.com/"
                                target="_blank"
                                rel="noopener"
                                title="Go to Twitter page"
                                class="social-icons__link"
                            >
                                <svg fill="none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                    <path d="M21.533 7.112c.015.213.015.426.015.64 0 6.502-4.949 13.994-13.995 13.994A13.9 13.9 0 0 1 0 19.538c.396.046.777.061 1.188.061a9.85 9.85 0 0 0 6.106-2.102 4.927 4.927 0 0 1-4.599-3.41c.305.045.61.075.93.075a5.2 5.2 0 0 0 1.294-.167A4.92 4.92 0 0 1 .975 9.168v-.061a4.954 4.954 0 0 0 2.223.624 4.915 4.915 0 0 1-2.193-4.097c0-.913.244-1.75.67-2.482A13.981 13.981 0 0 0 11.817 8.3a5.554 5.554 0 0 1-.122-1.126 4.917 4.917 0 0 1 4.92-4.92c1.415 0 2.695.595 3.593 1.554A9.684 9.684 0 0 0 23.33 2.62a4.906 4.906 0 0 1-2.162 2.71A9.86 9.86 0 0 0 24 4.57a10.574 10.574 0 0 1-2.467 2.543Z" fill="currentColor"></path>
                                </svg>
                            </a>

                            <a
                                href="https://www.facebook.com/"
                                target="_blank"
                                rel="noopener"
                                title="Go to Facebook page"
                                class="social-icons__link"
                            >
                                <svg fill="none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                    <path d="M24 12.073c0-6.63-5.371-12-12-12s-12 5.37-12 12c0 5.989 4.388 10.953 10.125 11.854v-8.386H7.077v-3.468h3.048V9.429c0-3.008 1.79-4.669 4.532-4.669 1.314 0 2.687.235 2.687.235v2.951H15.83c-1.49 0-1.955.925-1.955 1.874v2.253h3.328l-.532 3.468h-2.796v8.386C19.612 23.027 24 18.062 24 12.073Z" fill="currentColor"></path>
                                </svg>
                            </a>

                            <a
                                href="https://www.instagram.com/openfreedomproject"
                                target="_blank"
                                rel="noopener"
                                title="Go to Instagram page"
                                class="social-icons__link"
                            >
                                <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                                    <path d="M12.0027 5.84808C8.59743 5.84808 5.85075 8.59477 5.85075 12C5.85075 15.4053 8.59743 18.1519 12.0027 18.1519C15.4079 18.1519 18.1546 15.4053 18.1546 12C18.1546 8.59477 15.4079 5.84808 12.0027 5.84808ZM12.0027 15.9996C9.80212 15.9996 8.00312 14.2059 8.00312 12C8.00312 9.7941 9.79677 8.00046 12.0027 8.00046C14.2086 8.00046 16.0022 9.7941 16.0022 12C16.0022 14.2059 14.2032 15.9996 12.0027 15.9996ZM19.8412 5.59644C19.8412 6.39421 19.1987 7.03135 18.4062 7.03135C17.6085 7.03135 16.9713 6.38885 16.9713 5.59644C16.9713 4.80402 17.6138 4.16153 18.4062 4.16153C19.1987 4.16153 19.8412 4.80402 19.8412 5.59644ZM23.9157 7.05277C23.8247 5.13063 23.3856 3.42801 21.9775 2.02522C20.5747 0.622429 18.8721 0.183388 16.9499 0.0870135C14.9689 -0.0254238 9.03112 -0.0254238 7.05008 0.0870135C5.1333 0.178034 3.43068 0.617075 2.02253 2.01986C0.614389 3.42265 0.180703 5.12527 0.0843279 7.04742C-0.0281093 9.02845 -0.0281093 14.9662 0.0843279 16.9472C0.175349 18.8694 0.614389 20.572 2.02253 21.9748C3.43068 23.3776 5.12794 23.8166 7.05008 23.913C9.03112 24.0254 14.9689 24.0254 16.9499 23.913C18.8721 23.822 20.5747 23.3829 21.9775 21.9748C23.3803 20.572 23.8193 18.8694 23.9157 16.9472C24.0281 14.9662 24.0281 9.03381 23.9157 7.05277ZM21.3564 19.0728C20.9388 20.1223 20.1303 20.9307 19.0755 21.3537C17.496 21.9802 13.7481 21.8356 12.0027 21.8356C10.2572 21.8356 6.50396 21.9748 4.92984 21.3537C3.88042 20.9361 3.07195 20.1276 2.64897 19.0728C2.02253 17.4934 2.16709 13.7455 2.16709 12C2.16709 10.2546 2.02789 6.50129 2.64897 4.92717C3.06659 3.87776 3.87507 3.06928 4.92984 2.6463C6.50931 2.01986 10.2572 2.16443 12.0027 2.16443C13.7481 2.16443 17.5014 2.02522 19.0755 2.6463C20.1249 3.06392 20.9334 3.8724 21.3564 4.92717C21.9828 6.50665 21.8383 10.2546 21.8383 12C21.8383 13.7455 21.9828 17.4987 21.3564 19.0728Z" fill="currentColor"></path>
                                </svg>
                            </a>
                        </div>
                    </div>

                    // Logo in center
                    <div class="layout-element footer-logo">
                        <div class="image">
                            <img
                                src="/images/logo2.png"
                                alt="Open Freedom Project Logo"
                                class="image__image footer-logo-img"
                            />
                        </div>
                    </div>

                    // "Freedom" text
                    <div class="layout-element footer-text-center">
                        <div class="text-box">
                            <h4 class="text-white">"Freedom"</h4>
                        </div>
                    </div>

                    // "Project" text
                    <div class="layout-element footer-text-right">
                        <div class="text-box">
                            <h4 class="text-white">"Project"</h4>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
