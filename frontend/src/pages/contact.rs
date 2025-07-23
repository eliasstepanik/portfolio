use leptos::ev::SubmitEvent;
use leptos::*;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn ContactPage() -> impl IntoView {
    // Form field signals
    let (email, set_email) = create_signal(String::new());
    let (name, set_name) = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());

    // Error states
    let (email_error, set_email_error) = create_signal(String::new());
    let (name_error, set_name_error) = create_signal(String::new());
    let (message_error, set_message_error) = create_signal(String::new());

    // Form submission state
    let (submitting, set_submitting) = create_signal(false);
    let (success, set_success) = create_signal(false);

    // Helper function to get input value from event
    let get_value = |ev: web_sys::Event| {
        ev.target()
            .unwrap()
            .unchecked_into::<web_sys::HtmlInputElement>()
            .value()
    };

    let get_textarea_value = |ev: web_sys::Event| {
        ev.target()
            .unwrap()
            .unchecked_into::<web_sys::HtmlTextAreaElement>()
            .value()
    };

    // Validation functions
    let validate_email = move |email: &str| {
        if email.is_empty() {
            "Email is required".to_string()
        } else if !email.contains('@') || !email.contains('.') {
            "Please enter a valid email address".to_string()
        } else {
            String::new()
        }
    };

    let validate_name = move |name: &str| {
        if name.is_empty() {
            "Name is required".to_string()
        } else if name.len() < 2 {
            "Name must be at least 2 characters".to_string()
        } else {
            String::new()
        }
    };

    let validate_message = move |message: &str| {
        if message.is_empty() {
            "Message is required".to_string()
        } else if message.len() < 10 {
            "Message must be at least 10 characters".to_string()
        } else {
            String::new()
        }
    };

    // Form submission handler
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        // Reset success state
        set_success.set(false);

        // Validate all fields
        let email_val = email.get();
        let name_val = name.get();
        let message_val = message.get();

        let email_err = validate_email(&email_val);
        let name_err = validate_name(&name_val);
        let message_err = validate_message(&message_val);

        set_email_error.set(email_err.clone());
        set_name_error.set(name_err.clone());
        set_message_error.set(message_err.clone());

        // If no errors, submit the form
        if email_err.is_empty() && name_err.is_empty() && message_err.is_empty() {
            set_submitting.set(true);

            // Log to console (in a real app, this would send to a server)
            web_sys::console::log_1(
                &format!(
                    "Form submitted!\nEmail: {email_val}\nName: {name_val}\nMessage: {message_val}"
                )
                .into(),
            );

            // Simulate async submission
            set_timeout(
                move || {
                    set_submitting.set(false);
                    set_success.set(true);

                    // Clear form
                    set_email.set(String::new());
                    set_name.set(String::new());
                    set_message.set(String::new());
                },
                std::time::Duration::from_millis(1000),
            );
        }
    };

    view! {
        <div class="page contact-page">
            <section class="block block--desktop-first-visible block--mobile-first-visible">
                <div class="block-background block-background--fixed">
                    <img
                        src="/public/images/hero-contact.jpg"
                        alt="Contact Open Freedom Project"
                        class="block-background__image block-background__image--fixed"
                    />
                    <div class="block-background__overlay block-background__overlay--fixed"></div>
                </div>

                <div class="block-layout-container">
                    <div class="hero-layout">
                        <div class="hero-title">
                            <h1 class="text-white">"Contact us"</h1>
                        </div>

                        <div class="hero-subtitle">
                            <h3 class="text-white">"You have feedback, ideas, or questions?"</h3>
                            <p class="text-white" style="margin-top: 1rem;">
                                "We are excited to hear from you. Feel free to send us a message and we will come back to you as soon as possible."
                            </p>
                        </div>

                        <div class="contact-form-container" style="margin-top: 3rem;">
                            <form class="form contact-form" on:submit=on_submit>
                                <div class="form__control">
                                    // Email field
                                    <div class="input">
                                        <label class="input__label" for="email">
                                            "Your email*"
                                        </label>
                                        <input
                                            type="text"
                                            id="email"
                                            class="input__component"
                                            placeholder="Your email address"
                                            value=email
                                            on:input=move |ev| set_email.set(get_value(ev))
                                            disabled=submitting
                                        />
                                        {move || if !email_error.get().is_empty() {
                                            view! {
                                                <span class="error-message">{email_error.get()}</span>
                                            }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }}
                                    </div>

                                    // Name field
                                    <div class="input">
                                        <label class="input__label" for="name">
                                            "Your name*"
                                        </label>
                                        <input
                                            type="text"
                                            id="name"
                                            class="input__component"
                                            placeholder="Your name"
                                            value=name
                                            on:input=move |ev| set_name.set(get_value(ev))
                                            disabled=submitting
                                        />
                                        {move || if !name_error.get().is_empty() {
                                            view! {
                                                <span class="error-message">{name_error.get()}</span>
                                            }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }}
                                    </div>

                                    // Message field
                                    <div class="input">
                                        <label class="input__label" for="message">
                                            "Message*"
                                        </label>
                                        <textarea
                                            id="message"
                                            class="input__component input__component--textarea"
                                            placeholder="Enter your message"
                                            value=message
                                            on:input=move |ev| set_message.set(get_textarea_value(ev))
                                            disabled=submitting
                                        ></textarea>
                                        {move || if !message_error.get().is_empty() {
                                            view! {
                                                <span class="error-message">{message_error.get()}</span>
                                            }.into_view()
                                        } else {
                                            view! { <span></span> }.into_view()
                                        }}
                                    </div>

                                    // Submit button
                                    <button
                                        type="submit"
                                        class="grid-button grid-button--primary"
                                        disabled=submitting
                                    >
                                        {move || if submitting.get() { "Submitting..." } else { "Submit" }}
                                    </button>

                                    // Success message
                                    {move || if success.get() {
                                        view! {
                                            <div class="success-message">
                                                <p class="text-white">"Thank you for your message! We'll get back to you soon."</p>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! { <div></div> }.into_view()
                                    }}
                                </div>
                            </form>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
