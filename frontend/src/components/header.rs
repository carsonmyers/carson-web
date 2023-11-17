use gloo_timers::callback::Interval;
use yew::prelude::*;
use yew::{AttrValue, Properties};

use crate::common::HAlign;
use crate::components::Hr;
use crate::components::Menu;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub title: AttrValue,
    pub blink_interval: u32,
}

pub enum Msg {
    BlinkCursor,
}

pub struct Header {
    cursor_visible: bool,
    _interval: Interval,
}

impl Component for Header {
    type Message = Msg;
    type Properties = HeaderProps;

    fn create(ctx: &Context<Self>) -> Self {
        let interval = {
            let millis = ctx.props().blink_interval;
            let link = ctx.link().clone();
            Interval::new(millis, move || {
                link.send_message(Msg::BlinkCursor);
            })
        };

        Header {
            cursor_visible: false,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::BlinkCursor => {
                self.cursor_visible = !self.cursor_visible;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let grays = vec![
            String::from("bg-gray-50"),
            String::from("bg-gray-30"),
            String::from("bg-gray-45"),
            String::from("bg-gray-25"),
        ];

        html! {
            <div class="app-header">
                <Hr width={50} height={10} segments={8} />
                <h1>
                    <span class="prompt">{ "/home/carson # " }</span>
                    { &ctx.props().title }
                    <span class="cursor">{ if self.cursor_visible { "\u{2588}" } else { "" }}</span>
                </h1>

                <Hr classes={grays} align={HAlign::Right} width={20} segments={6} />
                <Menu />
                <Hr align={HAlign::Right} width={20} segments={6} />
            </div>
        }
    }
}
