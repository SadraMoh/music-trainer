use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{HomePage, KeyIdentifier, Layout, NotFound, RhythmIdentifier};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css" />
        // sets the document title
        <Title text="Music trainer" />

        <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@24,400,1,0" />

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=Layout>
                        <Route path="" view=HomePage />
                        <Route path="key-identifier" view=KeyIdentifier />
                        <Route path="rhythm-identifier" view=RhythmIdentifier />
                    </Route>
                    <Route path="/*any" view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}
