use std::sync::Mutex;

pub struct Proxy {
    proxy: Mutex<_Proxy>,
}

struct _Proxy {
    xauth: String,

    online: bool,
    closed: bool,
}
