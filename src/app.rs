use crate::{api::fetch_code, consts::CH03_05A};
use leptos::prelude::*;
use leptos_meta::{MetaTags, *};
use leptos_router::{
    components::{FlatRoutes, Route, Router, A},
    path, SsrMode,
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
    let ssr = SsrMode::Async;

    view! {
        <Stylesheet id="leptos" href="/pkg/axum_js_ssr.css"/>
        <Title text="Leptos JavaScript Integration Demo with Axum SSR"/>
        <Meta name="color-scheme" content="dark light"/>
        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A attr:class="example" href="/naive">"Naive "<code>"<script>"</code>
                    <small>"truly naive to start"</small></A>
            </nav>
            <main>
                <article>
                    <h1>"Leptos JavaScript Integration Demo"</h1>
                    <FlatRoutes fallback>
                        <Route path=path!("") view=HomePage/>
                        <Route path=path!("naive") view=Naive ssr/>
                    </FlatRoutes>
                </article>
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
            For the demonstrations, "<a href="https://github.com/highlightjs/highlight.js"><code>
            "highlight.js"</code></a>" will be called within this Leptos application through the following
            pages to show what works and what does not, which hopefully clearly show the benefits and
            drawbacks of every single method.  Needless to say, JavaScript must be enabled, and having the
            browser's developer tools/console opened will show the problems as they happen.
        "</p>
    }
}

#[component]
fn CodeDemo() -> impl IntoView {
    let code = Resource::new(|| (), |_| fetch_code());
    let code_view = move || {
        Suspend::new(async move {
            view! {
                <pre><code class="language-rust">{code.await}</code></pre>
            }
        })
    };
    view! {
        <p>"Explanation on what is being demonstrated follows after the following code example table."</p>
        <div id="code-demo">
            <table>
                <thead>
                    <tr>
                        <th>"Inline code block (part of this component)"</th>
                        <th>"Dynamic code block (loaded via server fn)"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><pre><code class="language-rust">{CH03_05A}</code></pre></td>
                        <td>
                            <Suspense fallback=move || view! { <p>"Loading code example..."</p> }>
                                {code_view}
                            </Suspense>
                        </td>
                    </tr>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn Naive() -> impl IntoView {
    view! {
        <h2>"Showing what happens when script inclusion is done naively"</h2>
        <CodeDemo/>
        <script src="/dummy.js"></script>
        <script src="/highlight.min.js"></script>
        <script>"hljs.highlightAll();"</script>
        <p>"
            This page demonstrates what happens (or doesn't happen) when it is assumed that the "<code>
            "highlight.js"</code>" library can just be included from some CDN (well, hosted locally for this
            example) as per their instructions for basic usage in the browser.  The following actions should
            be taken in order to fully experience the things that do not work.
        "</p>
        <ol>
            <li>"
                You may find that during the initial load of this page when first navigating to here from
                \"Home\" (do navigate there, reload to reinitiate this application to properly replicate the
                behavior), none of the code examples below are highlighted, but simply going back using the
                browser's navigation system and forward to here again, the inline code block will become
                highlighted.  The cause is due to "<code>"highlight.js"</code>" being loaded in a standard
                "<code>"<script>"</code>" tag that is part of this component and initially it wasn't loaded
                before the call to "<code>"hljs.highlightAll();"</code>" was made. Later, when the component
                gets re-rendered the second time, the code is finally available to ensure one of them works
                (while also reloading the script, which probably isn't desirable for this use case).
            "</li>
            <li>"
                If you use the browser's navigation system and reload this page, you will find that *both*
                code examples now appear to highlight correctly, yay! However now none of the local links
                work.  This is because the hydration system found markup where text was expected and that's
                panics the wasm module.  This error may be visible using the browser's console, but overall
                this pretty much breaks the page, so use the browser's navigation, hit back, and then refresh
                (because the back action triggered a push state, and with hydration broken, the reactive
                system is also broken, so yes the page is very much in a crashed state).
            "</li>
            <li>"
                Moreover, if you continue to use the browser's navigation system (without reloading here to
                cause the panic), you will find that the the browser's console log is spammed with dummy.js
                loaded - this is caused by the script unloading/reloading every time its "<code>"<script>"
                </code>" tag being re-created by this component.  This may or may not be a desirable
                behavior.
            "</li>
        </ol>
    }
}
