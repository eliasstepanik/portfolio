use leptos::*;
use leptos_router::*;
use wasm_bindgen::JsCast;
use web_sys::window;

#[component]
pub fn Header() -> impl IntoView {
    // Signal for mobile menu open/close state
    let (menu_open, set_menu_open) = create_signal(false);

    // Signal for dark mode state
    let (is_dark, set_is_dark) = create_signal(false);

    // Signal for header scroll state
    let (scrolled, set_scrolled) = create_signal(false);

    // Get current location for active page highlighting
    let location = use_location();
    let pathname = move || location.pathname.get();

    // Helper function to determine if a link is active
    let is_active = move |path: &str| pathname() == path;

    // Check initial theme from localStorage
    create_effect(move |_| {
        if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
            if let Ok(Some(theme)) = storage.get_item("theme") {
                set_is_dark.set(theme == "dark");
                if theme == "dark" {
                    if let Some(doc) = window().and_then(|w| w.document()) {
                        let _ = doc
                            .document_element()
                            .and_then(|el| el.set_attribute("data-theme", "dark").ok());
                    }
                }
            }
        }
    });

    let toggle_dark_mode = move |_| {
        let new_theme = !is_dark.get();
        set_is_dark.set(new_theme);

        if let Some(doc) = window().and_then(|w| w.document()) {
            let theme = if new_theme { "dark" } else { "light" };
            let _ = doc
                .document_element()
                .and_then(|el| el.set_attribute("data-theme", theme).ok());

            if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
                let _ = storage.set_item("theme", theme);
            }
        }
    };

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
                    <span class="logo-text">"ES Portfolio"</span>
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
                            <div class=move || if is_active("/projects") {
                                "item-content-wrapper item-content-wrapper--active block-header-item__item"
                            } else {
                                "item-content-wrapper block-header-item__item"
                            }>
                                <A href="/projects" class="item-content">
                                    "Projects"
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
                        <li class="block-header-item">
                            <button
                                class="dark-mode-button"
                                on:click=toggle_dark_mode
                            >
                                {move || if is_dark.get() { "☀️" } else { "🌙" }}
                            </button>
                        </li>
                    </ul>
                </nav>
            </div>

            // Mobile layout
            <div class="block-header-layout-mobile">
                <A href="/" class="block-header-logo block-header__logo">
                    <span class="logo-text">"ES Portfolio"</span>
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
                                <div class=move || if is_active("/projects") {
                                    "item-content-wrapper item-content-wrapper--active block-header-item__item"
                                } else {
                                    "item-content-wrapper block-header-item__item"
                                }>
                                    <A href="/projects" class="item-content" on:click=move |_| set_menu_open.set(false)>
                                        "Projects"
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
