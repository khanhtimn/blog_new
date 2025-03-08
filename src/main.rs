use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    http::Request,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::{AuthConfig, AuthSessionLayer};
use axum_session_sqlx::SessionPgPool;
use blog_new::{app::*, functions::auth::AuthSession, models::user::User, state::AppState};
use leptos::{config::get_configuration, logging::log, prelude::provide_context};
use leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes};
use sqlx::{postgres::PgPoolOptions, PgPool};

async fn server_fn_handler(
    State(app_state): State<AppState>,
    auth_session: AuthSession,
    path: Path<String>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    log!("{:?}", path);

    handle_server_fns_with_context(
        move || {
            provide_context(auth_session.clone());
            provide_context(app_state.pool.clone());
        },
        request,
    )
    .await
}

async fn leptos_routes_handler(
    auth_session: AuthSession,
    state: State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let State(app_state) = state.clone();
    let handler = leptos_axum::render_route_with_context(
        app_state.routes.clone(),
        move || {
            provide_context(auth_session.clone());
            provide_context(app_state.pool.clone());
        },
        move || shell(app_state.leptos_options.clone()),
    );
    handler(state, req).await.into_response()
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).expect("couldn't initialize logging");

    let database_url = dotenvy::var("DATABASE_URL").ok().unwrap();

    let pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Could not make pool.");

    // Auth section
    let session_config = SessionConfig::default().with_table_name("axum_sessions");
    let auth_config = AuthConfig::<i64>::default();
    let session_store =
        SessionStore::<SessionPgPool>::new(Some(SessionPgPool::from(pool.clone())), session_config)
            .await
            .unwrap();

    if let Err(e) = sqlx::migrate!().run(&pool).await {
        eprintln!("{e:?}");
    }

    // Setting this to None means we'll be using cargo-leptos and its env vars
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
        routes: routes.clone(),
    };

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(
            AuthSessionLayer::<User, i64, SessionPgPool, PgPool>::new(Some(pool.clone()))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
