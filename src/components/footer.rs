use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer footer-center p-10 bg-base-200 text-base-content border-t border-base-300">
            <div>
                <p>"if you want to contact for work, or you just want to chat, you can send me an email at "
                    <a href="mailto:khanhtimn@gmail.com" class="link link-accent">khanhtimn@gmail.com</a>
                </p>
            </div>
        </footer>
    }
}
