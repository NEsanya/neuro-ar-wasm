pub mod mainjs {
    fn code_runner(s: &str, f: &str) {
        js_sys::Function::new_no_args(s).call0(&wasm_bindgen::JsValue::NULL).expect(f);
    }

    pub struct FindJS<'a> {
        code: &'a str
    }

    pub struct StreamJS<'a> {
        code: &'a str
    }

    impl<'a> FindJS<'a> {
        pub fn new() -> Self {
            FindJS {
                code: "
                    const socket = new WebSocket('ws://localhost:8080')
                    let wifi_data = []

                    socket.onopen = () => {
                        console.log('connected')
                    }
                      
                    socket.onmessage = (data) => {
                        wifi_data = data.data
                    }

                    Compass.noSupport(() => {
                        console.log('Nope')
                    })
                    Compass.needGPS(function () {
                        console.log('JPS')         // Step 1: we need GPS signal
                      }).needMove(function () {
                        console.log('move1')
                        console.log('move2') // Step 2: user must go forward
                      }).init(function () {
                        console.log('init') // GPS hack is enabled
                    })
                    Compass.watch(function (heading) {
                        console.log(heading)
                    })         
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