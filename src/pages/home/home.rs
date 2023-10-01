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
                        <h3 class="text">"Identify keys"</h3>
                    </A>
               </li>
               <li>
                    <A href="/rhythm-identifier" class="module">
                        <img width="128" height="128" src="/assets/icons/modules/rhythm-identifier.svg" class="logo" />
                        <h3 class="text">"Identify rhythm"</h3>
                    </A>
               </li>
            </ul>
        </div>
    }
}
