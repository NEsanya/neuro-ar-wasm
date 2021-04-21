use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
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
        // let window: web_sys::Window = web_sys::window().expect("no global `window` exists");
        // let navigator = window.navigator();
        // let mut settings = web_sys::MediaStreamConstraints::new();
        // settings.video(&JsValue::TRUE);
        // let media_stream = navigator
        //     .media_devices().expect("No media devices")
        //     .get_user_media_with_constraints(&settings).expect("No camera");
        
        // async {
        //     let result = wasm_bindgen_futures::JsFuture::from(media_stream).await.expect("Then error");
        //     let video = window.document().expect("Document dont awaible")
        //         .create_element("video")
        //         .expect("Cannot create video")
        //         .dyn_into::<web_sys::HtmlVideoElement>()
        //         .expect("Cannot set uppcast");
        //     video.set_src_object(Some(&web_sys::MediaStream::new_with_tracks(&result).expect("")));
        //     video.set_onloadedmetadata(Some(&js_sys::Function::new_with_args("err", "")))
        // };
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
            <>
            <video></video>
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
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Main>::new().mount_to_body();
    let fc = js_sys::Function::new_no_args("
        const constraints = { video: true }

        navigator.mediaDevices.getUserMedia(constraints)
            .then(function(mediaStream) {
                const video = document.querySelector('video')
                video.srcObject = mediaStream
                video.onloadedmetadata = function(e) {
                    video.play()
                }
            })
            .catch(console.log);
    ");
    fc.call0(&JsValue::NULL).expect("Function is terrable");
}
