use leptos::*;
use leptos_router::A;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="home">
            <div class="headline">
                <h1>"Hello musical world!"</h1>
                <h2>"Choose a module to begin improving your musical skills."</h2>
            </div>
            <ul class="grid">
               <li>
                    <A href="/key-identifier" class="module">
                        <img width="128" height="128" src="/assets/icons/modules/key-identifier.svg" class="logo" />
                        <h4 class="text">"Identify keys"</h4>
                    </A>
               </li>
            </ul>
        </div>
    }
}
