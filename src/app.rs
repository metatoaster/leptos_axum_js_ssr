use leptos::prelude::*;
use leptos_meta::{MetaTags, *};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
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
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let fallback = || view! { "Page not found." }.into_view();

    view! {
        <Stylesheet id="leptos" href="/pkg/axum_js_ssr.css"/>
        <Title text="Leptos JavaScript Integration Demo with Axum SSR"/>
        <Meta name="color-scheme" content="dark light"/>
        <Router>
            <nav>
                <a href="/">"Home"</a>
            </nav>
            <main>
                <h1>"Leptos JavaScript Integration Demo"</h1>
                <FlatRoutes fallback>
                    <Route path=path!("") view=HomePage/>
                </FlatRoutes>
            </main>
        </Router>
    }
}

// TODO
// call highlight.js highlightAll naively (doesn't work, explain how/why)
// - static code (should work if navigated into)
// - code loaded via server function (should fail because there was delay loading)
// - all should cause hydration to fail
// call highlight.js highlightAll via events
// wrap highlight.js via wasm-bindgen, call highlight(code, {...}) directly

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <p>"
            This example application demonstrates a number of ways that JavaScript may be included and used
            with Leptos, with the following links leading to examples which may or may not work as expected
            to show the right and wrong ways to integrate JavaScript into Leptos.
        "</p>
        <p>"
            For the demonstrations below, "<a href="https://github.com/highlightjs/highlight.js"><code>
            "highlight.js"</code></a>" will be called within this Leptos application through the following
            pages to show what works and what does not, which hopefully clearly show the benefits and
            drawbacks of every single method.
        "</p>
    }
}
