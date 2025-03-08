use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, ProtectedRoute, Route, Router},
    path,
};

use crate::{
    providers::auth::{provide_auth, AuthContext},
    routes::{
        auth::{login::Login, logout::Logout, signup::Signup},
        home::Home,
    },
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_auth();
    let auth_context = use_context::<AuthContext>().expect("Failed to get AuthContext");
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/blog_new.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <FlatRoutes fallback=|| "Page not found.".into_view()>
                    <Route
                    path=path!("")
                    view=Home
                    />

                    <Route
                      path=path!("signup")
                      view=move || {
                          view! { <Signup action=auth_context.signup/> }
                      }
                    />

                    <Route
                    path=path!("login")
                      view=move || {
                          view! { <Login action=auth_context.login/> }
                      }
                    />

                    <Route
                    path=path!("logout")
                      view=move || {
                          view! { <Logout action=auth_context.logout/> }
                      }
                    />

                    <ProtectedRoute
                        path=path!("settings")
                        condition=move || auth_context.user.get().map(|r| r.ok().flatten().is_some())
                        redirect_path=|| "/"
                        view=move || {
                            view! {
                                <h1>"Settings"</h1>
                                <Logout action=auth_context.logout/>
                            }
                        }
                    />
                </FlatRoutes>
            </main>
        </Router>
    }
}
