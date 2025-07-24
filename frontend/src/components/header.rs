use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::window;

#[component]
pub fn Header() -> impl IntoView {
    // Signal for mobile menu open/close state
    let (menu_open, set_menu_open) = create_signal(false);

    // Signal for header scroll state
    let (scrolled, set_scrolled) = create_signal(false);

    // Get current location for active page highlighting
    let location = use_location();
    let pathname = move || location.pathname.get();

    // Helper function to determine if a link is active
    let is_active = move |path: &str| pathname() == path;

    // Set up scroll listener
    create_effect(move |_| {
        let handle_scroll = move || {
            if let Some(win) = window() {
                let scroll_y = win.scroll_y().unwrap_or(0.0);
                set_scrolled.set(scroll_y > 50.0);
            }
        };

        // Initial check
        handle_scroll();

        // Add scroll listener
        if let Some(win) = window() {
            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                handle_scroll();
            }) as Box<dyn FnMut(_)>);

            let _ =
                win.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });

    view! {
        <header class=move || {
            if scrolled.get() {
                "block-header block-header--scrolled"
            } else {
                "block-header"
            }
        }>
            <div class="block-header-layout-desktop">
                <A href="/" class="block-header-logo block-header__logo">
                    <img
                        src="/public/images/logo2.png"
                        alt="Open Freedom Project"
                        class="block-header-logo__image"
                    />
                </A>

                <nav class="block-header__nav">
                    <ul class="block-header__nav-links">
                        <li class="block-header-item">
                            <div class=move || if is_active("/") {
                                "item-content-wrapper item-content-wrapper--active block-header-item__item"
                            } else {
                                "item-content-wrapper block-header-item__item"
                            }>
                                <A href="/" class="item-content">
                                    "Home"
                                </A>
                            </div>
                        </li>
                        <li class="block-header-item">
                            <div class=move || if is_active("/about") {
                                "item-content-wrapper item-content-wrapper--active block-header-item__item"
                            } else {
                                "item-content-wrapper block-header-item__item"
                            }>
                                <A href="/about" class="item-content">
                                    "About"
                                </A>
                            </div>
                        </li>
                        <li class="block-header-item">
                            <div class=move || if is_active("/gallery") {
                                "item-content-wrapper item-content-wrapper--active block-header-item__item"
                            } else {
                                "item-content-wrapper block-header-item__item"
                            }>
                                <A href="/gallery" class="item-content">
                                    "Gallery"
                                </A>
                            </div>
                        </li>
                        <li class="block-header-item">
                            <div class=move || if is_active("/support") {
                                "item-content-wrapper item-content-wrapper--active block-header-item__item"
                            } else {
                                "item-content-wrapper block-header-item__item"
                            }>
                                <A href="/support" class="item-content">
                                    "Support"
                                </A>
                            </div>
                        </li>
                        <li class="block-header-item">
                            <div class=move || if is_active("/contact") {
                                "item-content-wrapper item-content-wrapper--active block-header-item__item"
                            } else {
                                "item-content-wrapper block-header-item__item"
                            }>
                                <A href="/contact" class="item-content">
                                    "Contact"
                                </A>
                            </div>
                        </li>
                    </ul>
                </nav>
            </div>

            // Mobile layout
            <div class="block-header-layout-mobile">
                <A href="/" class="block-header-logo block-header__logo">
                    <img
                        src="/public/images/logo2.png"
                        alt="Open Freedom Project"
                        class="block-header-logo__image"
                    />
                </A>

                <button
                    type="button"
                    class="burger block-header__hamburger-menu"
                    title="Menu"
                    on:click=move |_| set_menu_open.set(!menu_open.get())
                >
                    <span class="burger__bun"></span>
                    <span class="burger__meat"></span>
                    <span class="burger__bun"></span>
                </button>

                <div class=move || if menu_open.get() {
                    "block-header-layout-mobile__dropdown block-header-layout-mobile__dropdown--open block-header-layout-mobile__dropdown--link-align-right"
                } else {
                    "block-header-layout-mobile__dropdown block-header-layout-mobile__dropdown--link-align-right"
                }>
                    <nav class="block-header__nav">
                        <ul class="block-header__nav-links">
                            <li class="block-header-item">
                                <div class=move || if is_active("/") {
                                    "item-content-wrapper item-content-wrapper--active block-header-item__item"
                                } else {
                                    "item-content-wrapper block-header-item__item"
                                }>
                                    <A href="/" class="item-content" on:click=move |_| set_menu_open.set(false)>
                                        "Home"
                                    </A>
                                </div>
                            </li>
                            <li class="block-header-item">
                                <div class=move || if is_active("/about") {
                                    "item-content-wrapper item-content-wrapper--active block-header-item__item"
                                } else {
                                    "item-content-wrapper block-header-item__item"
                                }>
                                    <A href="/about" class="item-content" on:click=move |_| set_menu_open.set(false)>
                                        "About"
                                    </A>
                                </div>
                            </li>
                            <li class="block-header-item">
                                <div class=move || if is_active("/gallery") {
                                    "item-content-wrapper item-content-wrapper--active block-header-item__item"
                                } else {
                                    "item-content-wrapper block-header-item__item"
                                }>
                                    <A href="/gallery" class="item-content" on:click=move |_| set_menu_open.set(false)>
                                        "Gallery"
                                    </A>
                                </div>
                            </li>
                            <li class="block-header-item">
                                <div class=move || if is_active("/support") {
                                    "item-content-wrapper item-content-wrapper--active block-header-item__item"
                                } else {
                                    "item-content-wrapper block-header-item__item"
                                }>
                                    <A href="/support" class="item-content" on:click=move |_| set_menu_open.set(false)>
                                        "Support"
                                    </A>
                                </div>
                            </li>
                            <li class="block-header-item">
                                <div class=move || if is_active("/contact") {
                                    "item-content-wrapper item-content-wrapper--active block-header-item__item"
                                } else {
                                    "item-content-wrapper block-header-item__item"
                                }>
                                    <A href="/contact" class="item-content" on:click=move |_| set_menu_open.set(false)>
                                        "Contact"
                                    </A>
                                </div>
                            </li>
                        </ul>
                    </nav>
                </div>
            </div>
        </header>
    }
}
