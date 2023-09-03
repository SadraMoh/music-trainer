use crate::components::Icon;
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn Layout(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="layout">
            <header class="header">
                <button class="clickable">
                    <Icon name="home" />
                </button>
            </header>
            <div class="content">
                <Outlet/>
            </div>
        </div>
    }
}
