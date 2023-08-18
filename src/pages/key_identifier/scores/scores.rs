use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn Scores(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>
          <span>
            <span>"12"</span>
            "/"
            <span>"4"</span>
          </span>
          <span>"50%"</span>
        </div>
    }
}
