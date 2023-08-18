use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>"Hello musical world!"</div>
    }
}
