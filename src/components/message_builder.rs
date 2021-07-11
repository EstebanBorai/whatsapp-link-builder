use yew::prelude::*;

pub struct MessageBuilder {
    message: String,
    link: ComponentLink<Self>,
}

pub enum Msg {
    OnMessageInput(String),
}

impl Component for MessageBuilder {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        MessageBuilder {
            message: String::default(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnMessageInput(value) => {
                let old_value = self.message.clone();
                self.message = value;

                true
            }
            _ => true,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <form id="message-builder">
              <input type="text" value={self.message.clone()} oninput=self.link.callback(|event: InputData| Msg::OnMessageInput(event.value)) />
            </form>
        }
    }
}
