use crate::{api::fetch_code, consts::{CH03_05A, LEPTOS_HYDRATED}};
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
                <A attr:class="example" href="/naive-alt">"Leptos "<code>"<Script>"</code>
                    <small>"naively using load event"</small></A>
                <A attr:class="example" href="/naive-hook">"Leptos "<code>"<Script>"</code>
                    <small>"... correcting placement"</small></A>
                <A attr:class="example" href="/naive-fallback">"Leptos "<code>"<Script>"</code>
                    <small>"... with fallback"</small></A>
                <A attr:class="example" href="/custom-event">"Hydrated Event"
                    <small>"using "<code>"js_sys"</code>"/"<code>"web_sys"</code></small></A>
            </nav>
            <main>
                <article>
                    <h1>"Leptos JavaScript Integration Demo"</h1>
                    <FlatRoutes fallback>
                        <Route path=path!("") view=HomePage/>
                        <Route path=path!("naive") view=Naive ssr/>
                        <Route path=path!("naive-alt") view=|| view! { <NaiveEvent/> } ssr/>
                        <Route path=path!("naive-hook") view=|| view! { <NaiveEvent hook=true/> } ssr/>
                        <Route path=path!("naive-fallback") view=|| view! {
                            <NaiveEvent hook=true fallback=true/>
                        } ssr/>
                        <Route path=path!("custom-event") view=CustomEvent ssr/>
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

#[derive(Clone, Debug)]
struct CodeDemoHook {
    js_hook: String,
}

#[component]
fn CodeDemo() -> impl IntoView {
    let code = Resource::new(|| (), |_| fetch_code());
    let code_view = move || {
        Suspend::new(async move {
            let hook = use_context::<CodeDemoHook>().map(|h| {
                view! {
                    <Script>{h.js_hook}</Script>
                }
            });
            view! {
                <pre><code class="language-rust">{code.await}</code></pre>
                {hook}
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

#[component]
fn NaiveEvent(
    #[prop(optional)] hook: bool,
    #[prop(optional)] fallback: bool,
) -> impl IntoView {
    let render_hook = "\
document.querySelector('#hljs-src')
    .addEventListener('load', (e) => { hljs.highlightAll() }, false);";
    let render_call = "\
if (window.hljs) {
    hljs.highlightAll();
} else {
    document.querySelector('#hljs-src')
        .addEventListener('load', (e) => { hljs.highlightAll() }, false);
}";
    let js_hook = if fallback { render_call } else { render_hook };
    let explanation = if hook {
        provide_context(CodeDemoHook { js_hook: js_hook.to_string() });
        if fallback {
            view! {
                <ol>
                    <li>"
                        In this iteration, the following load hook is set in a "<code>"<Script>"</code>"
                        component after the dynamically loaded code example."
                        <pre><code class="language-javascript">{js_hook}</code></pre>
                    </li>
                    <li><strong>CSR</strong>"
                        This works much better now under CSR due to the fallback that checks whether the
                        library is already loaded or not.  Using the library directly if it's already loaded
                        and only register the event otherwise solves the rendering issue under CSR.
                    "</li>
                    <li><strong>SSR</strong>"
                        Much like the second example, hydration will still panic some of the time as per the
                        race condition that was described.
                    "</li>
                </ol>
            }.into_any()
        } else {
            view! {
                <ol>
                    <li>"
                        In this iteration, the following load hook is set in a "<code>"<Script>"</code>"
                        component after the dynamically loaded code example."
                        <pre><code class="language-javascript">{js_hook}</code></pre>
                    </li>
                    <li><strong>CSR</strong>"
                        Unfortunately, this still doesn't work reliably to highlight both code examples, in
                        fact, none of the code examples may highlight at all!  Placing the JavaScript loader
                        hook inside a "<code>Suspend</code>" will significantly increase the likelihood that
                        the event will be fired long before the loader adds the event hook.  As a matter of
                        fact, the highlighting is likely to only work with the largest latencies added for
                        the loading of "<code>"highlight.js"</code>", but at least both code examples will
                        highlight when working.
                    "</li>
                    <li><strong>SSR</strong>"
                        Much like the second example, hydration will still panic some of the time as per the
                        race condition that was described.
                    "</li>
                </ol>
            }.into_any()
        }
    } else {
        view! {
            <ol>
                <li>"
                    In this iteration, the following hook is set in a "<code>"<Script>"</code>" component
                    immediately following the one that loaded "<code>"highlight.js"</code>".
                    "<pre><code class="language-javascript">{js_hook}</code></pre>
                </li>
                <li><strong>CSR</strong>"
                    Unfortunately, the hook is being set directly on this component, rather than inside the
                    view for the dynamic block.  Given the nature of asynchronous loading which results in the
                    uncertainty of the order of events, it may or may not result in the dynamic code block
                    being highlighted under CSR (as there may or may not be a fully formed code block for
                    highlighting to happen).  This is affected by latency, so the loader here emulates a small
                    number of latency values (they repeat in a cycle).  The latency value is logged into the
                    console and it may be referred to witness its effects on what it does under CSR.  Test
                    this by going from home to here and then navigating between them using the browser's back
                    and forward feature for convenience - do ensure the "<code>"highlight.js" </code>" isn't
                    being cached by the browser.
                "</li>
                <li><strong>SSR</strong>"
                    Moreover, hydration will panic if the highlight script is loaded before hydration is
                    completed (from the resulting DOM mismatch after code highlighting).  Refreshing here
                    repeatedly may trigger the panic only some of the time when the "<code>"highlight.js"
                    </code>" script is loaded under the lowest amounts of artificial delay, as even under no
                    latency the hydration can still succeed due to the non-deterministic nature of this race
                    condition.
                "</li>
            </ol>
        }.into_any()
    };
    // FIXME Seems like <Script> require a text node, otherwise hydration error from marker mismatch
    view! {
        <h2>"Using the Leptos "<code>"<Script>"</code>" component asynchronously instead"</h2>
        <CodeDemo/>
        <Script id="hljs-src" async_="true" src="/highlight.min.js">""</Script>
        {(!hook).then(|| view! { <Script>{render_hook}</Script>})}
        <p>"
            What the "<code>"<Script>"</code>" component does is to ensure the "<code>"<script>"</code>" tag
            is placed in the document head in the order it is defined in a given component, rather than at
            where it was placed into the DOM.  Note that it is also a reactive component, much like the first
            example, it gets unloaded under CSR when the component is no longer active, In this improved
            version, "<code>"highlight.js"</code>" is also loaded asynchronously (using the "<code>"async"
            </code>" attribute), to allow an event listener that can delay highlighting to after the library
            is loaded.  This should all work out fine, right?
        "</p>
        {explanation}
        <p>"
            All that being said, all these naive examples still result in hydration being non-functional in
            varying degrees of (non-)reproducibility due to race conditions.  Is there any way to fix this?
            Is "<code>"wasm-bindgen"</code>" the only answer?  What if the goal is to incorporate external
            scripts that change often and thus can't easily have bindings built?  Follow onto the next
            examples to solve some of this, at the very least prevent the panic during hydration.
        "</p>
    }
}

#[component]
fn CustomEvent() -> impl IntoView {
    let js_hook = format!("\
var events = [];
if (!window.hljs) {{
    console.log('pushing listener for hljs load');
    events.push(new Promise((r) =>
        document.querySelector('#hljs-src').addEventListener('load', r, false)));
}}
if (!window.{LEPTOS_HYDRATED}) {{
    console.log('pushing listener for leptos hydration');
    events.push(new Promise((r) => document.addEventListener('{LEPTOS_HYDRATED}', r, false)));
}}
Promise.all(events).then(() => {{
    console.log(`${{events.length}} events have been dispatched; now calling highlightAll()`);
    hljs.highlightAll();
}});
");
    provide_context(CodeDemoHook { js_hook: js_hook.clone() });
    // FIXME Seems like <Script> require a text node, otherwise hydration error from marker mismatch
    view! {
        <h2>"Have Leptos dispatch an event when body is hydrated"</h2>
        <CodeDemo/>
        <Script id="hljs-src" async_="true" src="/highlight.min.js">""</Script>
        <p>"
            So if using events fixes problems with timing issues, couldn't Leptos provide an event to signal
            that the body is hydrated?  Actually, yes, since a typical Leptos application provide a "<code>
            "fn hydate()"</code>" in its "<code>"lib.rs"</code>", that can be modified to provide this very
            thing.  All that it takes is something like the following placed after
        "</p>
        <div><pre><code class="language-rust">{format!(
            r#"
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {{
    use app::App;
    // ... other hooks omitted
    leptos::mount::hydrate_body(App);

    // Now hydrate_body is done, provide ways to inform that
    let window = web_sys::window()
        .expect("window must exist in this context");
    // first set a flag to signal that hydration has happened and other
    // JavaScript code may just run without waiting for the event that
    // is just about to be dispatched, as the event is only a one-time
    // deal but this lives on as a variable that can be checked.
    js_sys::Reflect::set(
        &window,
        &wasm_bindgen::JsValue::from_str({LEPTOS_HYDRATED:?}),
        &wasm_bindgen::JsValue::TRUE,
    ).expect("error setting hydrated status");
    // Then dispatch the event for all the listeners that were added.
    let event = web_sys::Event::new({LEPTOS_HYDRATED:?}")
        .expect("error creating hydrated event");
    let document = window.document()
        .expect("document is missing");
    document.dispatch_event(&event)
        .expect("error dispatching hydrated event");
}}"#
        )}</code></pre></div>
        <p>"
            With the notification that hydration is completed, the following JavaScript code may be called
            inside "<code>"Suspense"</code>" block (in this live example, it's triggered by providing the
            following code via a "<code>"provide_context"</code>" which the code rendering component will then
            use within a "<code>"Suspend"</code>"):
        "</p>
        <div><pre><code class="language-javascript">{js_hook}</code></pre></div>
        <p>"
            No matter what latency there is, whatever the order did the API calls are done, this setup ensures
            the the code gets highlighted only after hydration is done and also after relevant delayed content
            are rendered from API calls.
        "</p>
    }
}
