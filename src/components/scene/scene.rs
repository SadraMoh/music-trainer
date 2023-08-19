use leptos::*;

#[component]
pub fn Scene(cx: Scope, children: Children) -> impl IntoView {
    // TODO: use scoped scss when the stylers crate is fixed
    // let styles = style_sheet!("./scene.scss");

    view! {
        cx,
        <div class="scene">
          {children(cx)}
        </div>
    }
}
