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
            String::from("bg-gray-25"),
            String::from("bg-gray-40"),
            String::from("bg-gray-30"),
            String::from("bg-gray-35"),
        ];

        html! {
            <div class="app-header">
                <Hr width={50} height={8} segments={8} />
                <h1 class="fg-green-60">
                    <span class="fg-gray-50">{ "/home/carson # " }</span>
                    <span class="input">{ &ctx.props().title }</span>
                    { if self.cursor_visible { "\u{2588}" } else { "" } }
                </h1>

                <Menu />
                <Hr align={HAlign::Right} width={20} height={1} segments={4} />
            </div>
        }
    }
}
