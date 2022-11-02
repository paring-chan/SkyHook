use cancellation::CancellationToken;
use winsafe::{DispatchMessage, GetMessage, TranslateMessage, MSG};

pub fn process_message(cancellation_token: &CancellationToken) {
    let mut msg: MSG = Default::default();

    while GetMessage(&mut msg, None, 0, 0).expect("Failed to get message") {
        if cancellation_token.is_canceled() {
            break;
        }

        TranslateMessage(&msg);
        unsafe {
            DispatchMessage(&msg);
        }
    }
}
