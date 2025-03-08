use leptos::prelude::*;

#[component]
pub fn Education() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-4">
            <h2 class="text-2xl font-bold mb-6">Education</h2>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="card bg-base-100 shadow-xl p-6">
                    <div class="flex items-center mb-4">
                        <div class="avatar mr-4">
                            <div class="w-12 h-12 rounded-lg">
                                <img src="/img/hust.svg" alt="HUST" />
                            </div>
                        </div>
                        <div>
                            <p class="text-sm text-gray-500">2022 - </p>
                            <h3 class="text-lg font-medium">"Bachelor degree in Computer Science"</h3>
                            <p class="text-sm">"Hanoi University of Science and Technology"</p>
                        </div>
                    </div>
                </div>

                <div class="card bg-base-100 shadow-xl p-6">
                    <div class="flex items-center mb-4">
                        <div class="avatar mr-4">
                            <div class="w-12 h-12 rounded-lg">
                                <img src="/img/clc.svg" alt="CLC" />
                            </div>
                        </div>
                        <div>
                            <p class="text-sm text-gray-500">2019 - 2021</p>
                            <h3 class="text-lg font-medium">"English Major"</h3>
                            <p class="text-sm">"Lao Cai Highschool for gifted students"</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
