use yew::prelude::*;

use crate::common::HAlign;

fn create_default_class_list() -> Vec<String> {
    vec![
        String::from("bg-yellow-50"),
        String::from("bg-orange-50"),
        String::from("bg-green-50"),
        String::from("bg-blue-40"),
    ]
}

#[derive(Properties, PartialEq)]
pub struct HrProps {
    #[prop_or_default]
    pub align: HAlign,
    #[prop_or(10)]
    pub segments: usize,
    #[prop_or(20)]
    pub width: usize,
    #[prop_or(3)]
    pub height: usize,
    #[prop_or_else(create_default_class_list)]
    pub classes: Vec<String>,
}

#[function_component(Hr)]
pub fn hr(props: &HrProps) -> Html {
    let mut segments = (0..props.segments)
        .into_iter()
        .zip(props.classes.iter().cycle())
        .map(|(_, class)| {
            let style = format!("min-width: {}px", props.width);
            html! {
                <div {style} {class}></div>
            }
        })
        .collect::<Vec<_>>();

    if props.align.is_left() {
        segments.reverse();
    }

    let align_class = match &props.align {
        HAlign::Left => "hr-left",
        HAlign::Right => "hr-right",
        HAlign::Center => "hr-left hr-right",
    };

    let style = format!("height: {}px;", props.height);

    html! {
        <div {style} class={format!("colorful-bar {align_class}")}>
            {segments}
        </div>
    }
}
