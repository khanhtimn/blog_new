use leptos::prelude::*;

#[component]
pub fn HeaderLayout(children: Children) -> impl IntoView {
    view! {
        <header class="top-0 inset-x-0 flex flex-wrap md:justify-start md:flex-nowrap z-50 w-full bg-base-100 border-b border-base-300">
            <div class="relative max-w-2xl w-full mx-2 py-2.5 md:flex md:items-center md:justify-between md:py-0 md:px-4 md:mx-auto">
                {children()}
            </div>
        </header>
    }
}
