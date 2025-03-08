use leptos::prelude::*;

#[component]
pub fn Articles() -> impl IntoView {
    view! {
        <div id="articles" class="container mx-auto px-4 py-10">
            <h2 class="text-2xl font-bold mb-6">Articles</h2>

            <div class="space-y-6">
                <div class="card bg-base-100 shadow-sm">
                    <div class="card-body">
                        <span class="text-sm text-neutral-400">"2024"</span>
                        <h3 class="card-title">"fake article 1"</h3>
                        <p>"fake summary"</p>
                        <div class="card-actions">
                            <a href="#" class="link link-primary">"Continue reading"</a>
                        </div>
                    </div>
                </div>

                <div class="card bg-base-100 shadow-sm">
                    <div class="card-body">
                        <span class="text-sm text-neutral-400">"2023"</span>
                        <h3 class="card-title">"fake article 2"</h3>
                        <p>"fake summary"</p>
                        <div class="card-actions">
                            <a href="#" class="link link-primary">"Continue reading"</a>
                        </div>
                    </div>
                </div>

                <div class="card bg-base-100 shadow-sm">
                    <div class="card-body">
                        <span class="text-sm text-neutral-400">"2022"</span>
                        <h3 class="card-title">"fake article 3"</h3>
                        <p>"fake summary"</p>
                        <div class="card-actions">
                            <a href="#" class="link link-primary">"Continue reading"</a>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    }
}
