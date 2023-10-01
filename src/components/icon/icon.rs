use leptos::*;

#[component]
pub fn Icon<'name, 'class>(
    cx: Scope,
    name: &'name str,
    #[prop(default = "")] class: &'class str,
) -> impl IntoView
where
    'name: 'class,
    'class: 'name,
{
    let name = name.to_string();
    let class = class.to_string();

    view! {
        cx,
        <span class=format!{"material-symbols-rounded icon {}", class}>
          {name}
        </span>
    }
}
