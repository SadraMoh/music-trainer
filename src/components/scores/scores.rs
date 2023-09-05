use leptos::*;

#[component]
pub fn Scores(
    cx: Scope,
    correct_count: MaybeSignal<u128>,
    wrong_count: MaybeSignal<u128>,
) -> impl IntoView {
    let total_count = move || correct_count() + wrong_count();
    let percentage = move || {
        if total_count() == 0 {
            return 0.;
        }
        (correct_count() as f64 / total_count() as f64) * 100 as f64
    };

    view! {
        cx,
        <div class="scores">
          <span>
            <span>{move || correct_count()}</span>
            "/"
            <span>{move || total_count()}</span>
          </span>
          <span>{move || format!("{}%", percentage().round())}</span>
        </div>
    }
}
