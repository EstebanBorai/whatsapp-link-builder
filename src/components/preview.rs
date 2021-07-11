use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PreviewProps {
    #[prop_or(String::default())]
    pub message: String,
}

pub struct Preview {
    props: PreviewProps,
}

pub enum Msg {}

impl Component for Preview {
    type Message = Msg;
    type Properties = PreviewProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Preview { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="preview">
              {self.props.message.clone()}
            </div>
        }
    }
}
