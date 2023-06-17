use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::{apppage::AppPage, homepage::HomePage};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/step_web.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/home" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/app" view=|cx| view! { cx, <AppPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

