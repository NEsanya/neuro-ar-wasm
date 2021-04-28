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

    pub trait JS {
        fn new() -> Self;
        fn run_code(&self) -> ();
    }

    impl<'a> JS for FindJS<'a> {
        fn new() -> Self {
            FindJS {
                code: "
                    const socket = new WebSocket('ws://localhost:8080')
                    let wifi_data = false

                    socket.onopen = () => {
                        console.log('connected')
                    }
                      
                    socket.onmessage = (data) => {
                        wifi_data = data.data
                    }

                    Compass.noSupport(() => {
                        console.log('Nope')
                    })
                    Compass.needGPS(() => {
                        console.log('JPS')         // Step 1: we need GPS signal
                      }).needMove(function () {
                        console.log('move1')
                        console.log('move2') // Step 2: user must go forward
                      }).init(function () {
                        console.log('init') // GPS hack is enabled
                    })
                    Compass.watch(heading => {
                        const img = document.getElementById('img-vec')
                        if(wifi_data) {
                            img.src = './img/success.png'
                        } else {
                            if(heading < 90) {
                                img.src = './img/up-arrow.png'
                            }
                            else if(heaing < 180) {
                                img.src = './img/left-arrow.png'
                            }
                            else if (heading< 270) {
                                img.src = './img/down-arrow.png'
                            } 
                            else if (heading < 360) {
                                img.src = './img/right-arrow.png'
                            }
                            else {
                                alert('something wrong') 
                            }
                        }
                    })         
                "
            }
        }

        fn run_code(&self) -> () {
            code_runner(self.code, "Function (Bluetooth) is terrable")
        }
    }


    impl<'a> JS for StreamJS<'a> {
        fn new() -> Self {
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

        fn run_code(&self) -> () {
            code_runner(self.code, "Function (Stream) is terrable")
        }
    }

    pub fn run<T: JS>() -> () {
        T::new().run_code();
    }
}