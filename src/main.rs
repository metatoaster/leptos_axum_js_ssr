#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{http::header, response::IntoResponse, routing::get, Router};
    use axum_js_ssr::app::*;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_http::services::ServeFile;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    async fn dummy_js() -> impl IntoResponse {
        (
            [(header::CONTENT_TYPE, "text/javascript")],
            "console.log('dummy.js loaded');\n",
        )
    }

    let app = Router::new()
        .route_service(
            "/highlight.min.js",
            ServeFile::new(
                "node_modules/@highlightjs/cdn-assets/highlight.min.js",
            ),
        )
        .route("/dummy.js", get(dummy_js))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
