use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn Layout(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="layout"><Outlet/></div>
    }
}
