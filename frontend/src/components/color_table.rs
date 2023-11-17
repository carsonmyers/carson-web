use yew::prelude::*;

#[function_component(ColorTable)]
pub fn color_table() -> Html {
    let left_colors = vec!["red", "orange", "yellow"];
    let right_colors = vec!["green", "blue", "purple"];
    let variants = (0..=20).into_iter().map(|v| v * 5).collect::<Vec<_>>();

    let left = color_pairs(left_colors, &variants);
    let right = color_pairs(right_colors, &variants);

    html! {
        <div class="container font-monospace text-center">
            <div class="row">
                <div class="col">
                    {left}
                </div>
                <div class="col">
                    {right}
                </div>
            </div>
        </div>
    }
}

fn color_pairs(colors: Vec<&str>, variants: &[i32]) -> Vec<Html> {
    colors
        .into_iter()
        .flat_map(|color| variants.into_iter().map(move |v| (color, v, *v < 50)))
        .map(|(color, val, use_contrast)| {
            (
                format!("fg-{}-{}", color, val),
                format!("bg-{}-{}", color, val),
                format!("fg-gray-{}", if use_contrast { 95 } else { 5 }),
            )
        })
        .map(|(fg, bg, bg_contrast)| {
            html! {
                <div class="row">
                    <div class={format!("{fg} col")}>{fg}</div>
                    <div class={format!("{bg} {bg_contrast} col")}>{&bg}</div>
                </div>
            }
        })
        .collect::<Vec<_>>()
}
