use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Main {
    link: ComponentLink<Self>,
    way_string: Option<String>
}

enum MainActions {
    LeadTheWay,
}

impl Component for Main {
    type Message = MainActions;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            way_string: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MainActions::LeadTheWay => self.way_string = Some("value".to_string()) 
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| MainActions::LeadTheWay)>{"Начать поиск"}</button>
                <p>
                    {
                        match &self.way_string {
                            Some(value) => String::from(value),
                            None => String::from("")
                        }
                    }
                </p>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Main>::new().mount_to_body();
}
