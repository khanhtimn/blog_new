pub mod header_layout;

pub mod default_layout {
    use leptos::prelude::*;

    #[component]
    pub fn DefaultLayout(
        #[prop(optional)] class: Option<String>,
        children: Children,
    ) -> impl IntoView {
        let class_string = class.unwrap_or_default();

        view! {
            <div class="min-h-screen bg-base-200">
                <div class="w-full max-w-2xl mx-auto pt-10 md:pt-16 px-4 sm:px-6 lg:px-8">
                // <div class="max-w-4xl mx-auto px-6 sm:px-12 lg:px-20">
                    <div class={class_string.to_owned()}>
                        {children()}
                    </div>
                </div>
            </div>
        }
    }
}
