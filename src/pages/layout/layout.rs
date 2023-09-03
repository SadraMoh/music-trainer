use crate::components::Icon;
use leptos::*;
use leptos_router::{Outlet, A};

#[component]
pub fn Layout(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="layout">
            <header class="header">
                <A href="/" class="clickable">
                    <Icon name="home" />
                </A>
            </header>
            <div class="content">
                <Outlet/>
            </div>
        </div>
    }
}
