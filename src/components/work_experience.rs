use leptos::prelude::*;

#[component]
pub fn WorkExperience() -> impl IntoView {
    view! {
        <div id="work" class="container mx-auto px-4 py-10">
            <h2 class="text-2xl font-bold mb-6">"Work experience"</h2>

            <div>
                // Placeholder
                <div class="group relative flex gap-x-5">
                    <div class="relative group-last:after:hidden after:absolute after:top-8 after:bottom-2 after:start-3 after:w-px after:-translate-x-[0.5px] after:bg-gray-200 dark:after:bg-neutral-700">
                        <div class="avatar">
                            <div class="relative z-10 size-6 flex justify-center items-center">
                                <svg xmlns="http://www.w3.org/2000/svg" shrink-0 size-6 width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-briefcase-business"><path d="M12 12h.01"/><path d="M16 6V4a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v2"/><path d="M22 13a18.15 18.15 0 0 1-20 0"/><rect width="20" height="14" x="2" y="6" rx="2"/></svg>
                            </div>
                        </div>
                    </div>
                    <div class="grow pb-8 group-last:pb-0">
                        <h3 class="mb-1 text-xs text-primary">"2023 - Present"</h3>
                        <p class="font-semibold">"Web Designer & Web Developer"</p>
                        <p class="mt-1 text-sm">"Well I made this in Rust so.."</p>

                        <ul class="list-disc ms-6 mt-3 space-y-1.5">
                            <li class="ps-1 text-sm">"this is for showcase"</li>
                            <li class="ps-1 text-sm">"this is for showcase"</li>
                            <li class="ps-1 text-sm">"this is for showcase"</li>
                            <li class="ps-1 text-sm">"this is for showcase"</li>
                        </ul>
                    </div>
                </div>


                <div class="group relative flex gap-x-5">
                    <div class="relative group-last:after:hidden after:absolute after:top-8 after:bottom-2 after:start-3 after:w-px after:-translate-x-[0.5px] after:bg-gray-200 dark:after:bg-neutral-700">
                        <div class="avatar">
                            <div class="relative z-10 size-6 flex justify-center items-center">
                                <svg xmlns="http://www.w3.org/2000/svg" shrink-0 size-6 width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-gamepad-2"><line x1="6" x2="10" y1="11" y2="11"/><line x1="8" x2="8" y1="9" y2="13"/><line x1="15" x2="15.01" y1="12" y2="12"/><line x1="18" x2="18.01" y1="10" y2="10"/><path d="M17.32 5H6.68a4 4 0 0 0-3.978 3.59c-.006.052-.01.101-.017.152C2.604 9.416 2 14.456 2 16a3 3 0 0 0 3 3c1 0 1.5-.5 2-1l1.414-1.414A2 2 0 0 1 9.828 16h4.344a2 2 0 0 1 1.414.586L17 18c.5.5 1 1 2 1a3 3 0 0 0 3-3c0-1.545-.604-6.584-.685-7.258-.007-.05-.011-.1-.017-.151A4 4 0 0 0 17.32 5z"/></svg>
                            </div>
                        </div>
                    </div>
                    <div class="grow pb-8 group-last:pb-0">
                        <h3 class="mb-1 text-xs text-primary">"2012 - Present"</h3>
                        <p class="font-semibold">"Minecraft Mod Developer"</p>
                        <p class="mt-1 text-sm">"Try out Khuacraft guys.."</p>

                        <div class="mt-3">

                            <a class="card card-side bg-base-100 shadow-sm" href="https://www.curseforge.com/minecraft/modpacks/khuacraft">
                                <div class="relative flex items-center overflow-hidden">
                                    <figure>
                                        <img class="w-32 sm:w-48 h-full absolute inset-0 object-cover rounded-s-lg" src="https://cdn.modrinth.com/data/i5C6oOoT/9f27544bd86d85f61223eedd115eca22dcba198c_96.webp" alt="Khuacraft" />
                                    </figure>
                                    <div class="grow p-4 ms-32 sm:ms-48">
                                        <div class="min-h-24 flex flex-col justify-center">
                                            <div class="card-body">
                                                <h2 class="card-title">"Goofy ahh minecraft modpack"</h2>
                                                <p>"View Khuacraft on curseforge"</p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </a>

                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
