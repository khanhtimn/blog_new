use crate::functions::auth::{get_user, Login, Logout, Signup};
use crate::models::user::User;
use leptos::prelude::*;

#[derive(Clone)]
pub struct AuthContext {
    pub login: ServerAction<Login>,
    pub logout: ServerAction<Logout>,
    pub signup: ServerAction<Signup>,
    pub user: Resource<Result<Option<User>, ServerFnError>>,
}
/// Get the current user and place it in Context
pub fn provide_auth() {
    let login = ServerAction::<Login>::new();
    let logout = ServerAction::<Logout>::new();
    let signup = ServerAction::<Signup>::new();

    let user = Resource::new(
        move || {
            (
                login.version().get(),
                signup.version().get(),
                logout.version().get(),
            )
        },
        move |_| get_user(),
    );

    provide_context(AuthContext {
        user,
        login,
        logout,
        signup,
    })
}
