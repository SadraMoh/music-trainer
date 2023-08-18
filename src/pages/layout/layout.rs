use leptos::*;
use leptos_router::Outlet;

/// Renders the home page of your application.
#[component]
pub fn Layout(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="layout"><Outlet/></div>
    }
}
