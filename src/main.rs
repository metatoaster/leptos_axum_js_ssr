#[cfg(feature = "ssr")]
mod latency {
    use std::sync::{Mutex, OnceLock};
    pub static LATENCY: OnceLock<
        Mutex<std::iter::Cycle<std::slice::Iter<'_, u64>>>,
    > = OnceLock::new();
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{http::header, response::IntoResponse, routing::get, Router};
    use axum_js_ssr::app::*;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    latency::LATENCY.get_or_init(|| [0, 4, 40, 400].iter().cycle().into());

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

    async fn highlight_js() -> impl IntoResponse {
        let delay = match latency::LATENCY
            .get()
            .expect("latency cycle wasn't set up")
            .try_lock()
        {
            Ok(ref mut mutex) => *mutex.next().expect("cycle always has next"),
            Err(_) => 0,
        };
        log!("loading highlight.min.js with latency of {delay} ms");
        tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
        let highlight_js_src = include_str!(
            "../node_modules/@highlightjs/cdn-assets/highlight.min.js"
        );
        (
            [(header::CONTENT_TYPE, "text/javascript")],
            format!(
                "{highlight_js_src}\nconsole.log('loaded highlight.js with a \
                 minimum latency of {delay} ms')"
            ),
        )
    }

    let app = Router::new()
        .route("/highlight.min.js", get(highlight_js))
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
