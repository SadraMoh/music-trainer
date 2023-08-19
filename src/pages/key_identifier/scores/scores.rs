use leptos::*;

#[component]
pub fn Scores(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="scores">
          <span>
            <span>"12"</span>
            "/"
            <span>"4"</span>
          </span>
          <span>"50%"</span>
        </div>
    }
}
