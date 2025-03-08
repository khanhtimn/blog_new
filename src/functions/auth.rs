use crate::models::user::User;
use leptos::prelude::*;
use leptos::server;

// Explicitly is not Serialize/Deserialize!
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserPasshash(String);

cfg_if::cfg_if! {
if #[cfg(feature = "ssr")] {
    use axum_session_auth::{Authentication, HasPermission};
    use axum_session_sqlx::SessionPgPool;
    use sqlx::PgPool;
    use std::collections::HashSet;
    pub type AuthSession = axum_session_auth::AuthSession<User, i64, SessionPgPool, PgPool>;
    use async_trait::async_trait;
    use bcrypt::{hash, verify, DEFAULT_COST};

    pub fn pool() -> Result<PgPool, ServerFnError> {
        use_context::<PgPool>().ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
    }

    pub fn auth() -> Result<AuthSession, ServerFnError> {
        use_context::<AuthSession>()
            .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
    }

    impl User {
        pub async fn get_with_passhash(id: i64, pool: &PgPool) -> Option<(Self, UserPasshash)> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_one(pool)
                .await
                .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifying them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = $1;",
            )
            .bind(id)
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }

        pub async fn get(id: i64, pool: &PgPool) -> Option<Self> {
            User::get_with_passhash(id, pool)
                .await
                .map(|(user, _)| user)
        }

        pub async fn get_from_username_with_passhash(
            name: String,
            pool: &PgPool,
        ) -> Option<(Self, UserPasshash)> {
            let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE username = $1")
                .bind(name)
                .fetch_one(pool)
                .await
                .ok()?;

            //lets just get all the tokens the user can use, we will only use the full permissions if modifying them.
            let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
                "SELECT token FROM user_permissions WHERE user_id = $1;",
            )
            .bind(sqluser.id)
            .fetch_all(pool)
            .await
            .ok()?;

            Some(sqluser.into_user(Some(sql_user_perms)))
        }

        pub async fn get_from_username(name: String, pool: &PgPool) -> Option<Self> {
            User::get_from_username_with_passhash(name, pool)
                .await
                .map(|(user, _)| user)
        }
    }

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    #[async_trait]
    impl Authentication<User, i64, PgPool> for User {
        async fn load_user(userid: i64, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
            let pool = pool.unwrap();

            User::get(userid, pool)
                .await
                .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }

    #[async_trait]
    impl HasPermission<PgPool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
            self.permissions.contains(perm)
        }
    }

    #[derive(sqlx::FromRow, Clone, Debug)]
    pub struct SqlUser {
        pub id: i64,
        pub username: String,
        pub password: String,
    }

    impl SqlUser {
        pub fn into_user(
            self,
            sql_user_perms: Option<Vec<SqlPermissionTokens>>,
        ) -> (User, UserPasshash) {
            (
                User {
                    id: self.id,
                    username: self.username,
                    permissions: if let Some(user_perms) = sql_user_perms {
                        user_perms
                            .into_iter()
                            .map(|x| x.token)
                            .collect::<HashSet<String>>()
                    } else {
                        HashSet::<String>::new()
                    },
                },
                UserPasshash(self.password),
            )
        }
    }
}
}

#[server]
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    let auth = auth()?;

    Ok(auth.current_user)
}

#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    let (user, UserPasshash(expected_passhash)) =
        User::get_from_username_with_passhash(username, &pool)
            .await
            .ok_or_else(|| {
                // Do a dummy verification to prevent timing attacks
                let _ = verify(
                    &password,
                    "$2b$10$dummyhashfordummyuserCxZB2s5aSzD6lIpZYHN1lxUgZg9aO",
                );
                ServerFnError::new("User does not exist.")
            })?;

    match verify(password, &expected_passhash)? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            sqlx::query("UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE id = $1")
                .bind(user.id)
                .execute(&pool)
                .await?;
            leptos_axum::redirect("/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    password: String,
    password_confirmation: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    // if password.len() < 8 {
    //     return Err(ServerFnError::ServerError("Password must be at least 8 characters".into()));
    // }

    if password != password_confirmation {
        return Err(ServerFnError::ServerError("Passwords did not match".into()));
    }

    let existing_user =
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)")
            .bind(&username)
            .fetch_one(&pool)
            .await?;

    if existing_user {
        return Err(ServerFnError::ServerError("Username already taken".into()));
    }

    // Start transaction
    let mut tx = pool.begin().await?;

    let password_hashed = hash(password, DEFAULT_COST)?;

    let user_id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO users (username, password, created_at)
         VALUES ($1, $2, CURRENT_TIMESTAMP)
         RETURNING id",
    )
    .bind(&username)
    .bind(&password_hashed)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query("INSERT INTO user_permissions (user_id, token) VALUES ($1, 'user.basic')")
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

    // Commit transaction
    tx.commit().await?;

    let mut permissions = HashSet::new();
    permissions.insert("user.basic".to_string());

    let user = User {
        id: user_id,
        username,
        permissions,
    };

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());

    leptos_axum::redirect("/");
    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth = auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}
