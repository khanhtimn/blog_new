use crate::functions::auth::Logout;
use leptos::prelude::*;

#[component]
pub fn Logout(action: ServerAction<Logout>) -> impl IntoView {
    view! {
        <div id="loginbox">
            <ActionForm action=action>
                <button type="submit" class="button">
                    "Log Out"
                </button>
            </ActionForm>
        </div>
    }
}
