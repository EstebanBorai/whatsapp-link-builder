use yew::prelude::*;

use crate::components::{MessageBuilder, Preview};

pub struct App {
    message: String,
}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {
            message: String::default(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="website">
                <header id="header">
                    <h1>{"WhatsApp Link Builder"}</h1>
                </header>
                <main id="main">
                    <MessageBuilder />
                    <Preview message=self.message.clone() />
                </main>
            </div>
        }
    }
}
