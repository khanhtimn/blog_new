use crate::components::articles::Articles;
use crate::components::education::Education;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::profile::Profile;
use crate::components::projects::Projects;
use crate::components::skills::Skills;
use crate::components::work_experience::WorkExperience;
use crate::layouts::default_layout::DefaultLayout;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-base-200">
            <Header />

            <DefaultLayout>
                <Profile />
                <Projects />
                <WorkExperience />
                <Skills />
                <Education />
                <Articles />
            </DefaultLayout>

            <Footer />
        </div>
    }
}
