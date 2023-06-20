use leptos::*;
use leptos_chart::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let chart_v = Chart::new(
        Series::from(vec!["A", "B", "C"]),
        Series::from(vec![1.0, 6.0, 9.]),
    )
    .set_view(820, 620, 3, 100, 100, 20);

    let chart_h = Chart::new(
        Series::from(vec![0.7, 1.5, 2.]),
        Series::from(vec!["A", "B", "C"]),
    )
    .set_view(820, 620, 3, 100, 100, 20);

    view! {cx,
        <div class="mx-auto p-8" style="background:#00000077">

            <h1>"Bar chart example"</h1>
            <BarChart chart=chart_v />

            <h1>"Bar chart example"</h1>
            <BarChart chart=chart_h />

        </div>
    }
}
