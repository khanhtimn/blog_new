use leptos::prelude::*;

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-10">
            <h2 class="text-2xl font-bold mb-6">Skills</h2>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-y-8">
                <div>
                    <h3 class="font-medium mb-2">"Tech stacks"</h3>
                    <div class="flex flex-wrap gap-2">
                        <div class="badge badge-outline">"JQuery"</div>
                    </div>
                </div>

                <div class="md:col-span-2">
                    <h3 class="font-medium mb-2">"Soft Skills:"</h3>
                    <p>"Make ass drill beats on FLStudio"</p>
                </div>
            </div>
        </div>
    }
}
