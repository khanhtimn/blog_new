use crate::layouts::header_layout::HeaderLayout;
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <HeaderLayout>
            <div class="navbar justify-center">
                <div class="navbar-center flex flex-wrap justify-center">
                    <ul class="menu menu-horizontal px-1 space-x-2 flex-wrap justify-center">
                        <li><a class="font-medium" href="#">Home</a></li>
                        <li><a class="font-medium" href="#projects">Projects</a></li>
                        <li><a class="font-medium" href="#work">Work</a></li>
                        <li><a class="font-medium" href="#articles">Articles</a></li>
                    </ul>
                </div>
            </div>
        </HeaderLayout>
    }
}
