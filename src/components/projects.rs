use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div id="projects" class="py-10">
            <h2 class="text-2xl font-bold mb-6">Projects</h2>

            // Placeholder
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                // Project 1
                <div class="card bg-base-100 shadow-xl">
                    <figure><img src="/img/project1.jpg" alt="Project 1" /></figure>
                </div>

                // Project 2
                <div class="card bg-base-100 shadow-xl">
                    <figure><img src="/img/project2.jpg" alt="Project 2" /></figure>
                </div>

                // Project 3
                <div class="card bg-base-100 shadow-xl">
                    <figure><img src="/img/project3.jpg" alt="Project 3" /></figure>
                </div>

            </div>
        </div>
    }
}
