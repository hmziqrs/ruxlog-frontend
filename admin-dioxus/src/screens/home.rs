use dioxus::prelude::*;
use dioxus_radio::hooks::{use_radio, RadioAsyncReducer};

use crate::store::{AuthState, AuthStateAction, AuthStateChannel};

#[component]
pub fn HomeScreen() -> Element {
    let mut auth_channel = use_radio::<AuthState, AuthStateChannel>(AuthStateChannel::Main);
    let auth_state = auth_channel.read();

    rsx! {
        div {
            h1 { "Welcome to Dioxus!" }
            p { "This is a simple example of a Dioxus application." }
            p { {format!("{:?}", auth_state.login_status)} }
            button {
                class: "btn btn-primary", onclick: move |_| {
                    auth_channel.async_apply(AuthStateAction::Login("".to_string(), "".to_string()));
                // spawn(async move {
                //     auth_channel.write().login("username".to_string(), "password".to_string()).await;
                // });
            }, "Simulate"
            }
        }
    }
}
