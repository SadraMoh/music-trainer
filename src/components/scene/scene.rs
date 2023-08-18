use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn Scene(cx: Scope, children: Children) -> impl IntoView {
    view! {
        cx,
        <div class="scene">
          {children(cx)}
        </div>
    }
}
