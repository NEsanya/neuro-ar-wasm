pub mod mainjs {
    fn code_runner(s: &str, f: &str) {
        js_sys::Function::new_no_args(s).call0(&wasm_bindgen::JsValue::NULL).expect(f);
    }

    pub struct BluethoothJS<'a> {
        code: &'a str
    }

    pub struct StreamJS<'a> {
        code: &'a str
    }

    impl<'a> BluethoothJS<'a> {
        pub fn new() -> Self {
            BluethoothJS {
                code: "
                    const socket = new WebSocket('ws://localhost:8080')

                    socket.onopen = () => {
                        console.log('connected')
                    };
                      
                      socket.onmessage = (data) => {
                        console.log(data.data);
                    };
                "
            }
        }

        pub fn run_code(&self) -> () {
            code_runner(self.code, "Function (Bluetooth) is terrable")
        }
    }


    impl<'a> StreamJS<'a> {
        pub fn new() -> Self {
            StreamJS {
                code: "
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
                "
            }
        }

        pub fn run_code(&self) -> () {
            code_runner(self.code, "Function (Stream) is terrable")
        }
    }
}