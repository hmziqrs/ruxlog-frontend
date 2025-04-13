use super::{AuthState, AuthStateAction, AuthStateChannel};
use crate::store::StateFrame;
use dioxus::logger::tracing;
use dioxus_radio::hooks::{use_radio, ChannelSelection, DataAsyncReducer, Radio};

impl AuthState {
    pub fn new() -> Self {
        AuthState {
            login_status: StateFrame::<bool>::new(),
        }
    }

    pub async fn login(&mut self, username: String, password: String) {
        self.login_status.set_loading(None);
        tracing::info!("loading: {:?}", self.login_status);

        gloo_timers::future::TimeoutFuture::new(2000).await;

        self.login_status.set_success(Some(true), None);
        tracing::info!("success: {:?}", self.login_status);
    }
}

impl DataAsyncReducer for AuthState {
    type Channel = AuthStateChannel;
    type Action = AuthStateAction;

    async fn async_reduce(
        radio: &mut Radio<AuthState, Self::Channel>,
        action: Self::Action,
    ) -> ChannelSelection<Self::Channel> {
        match action {
            AuthStateAction::Login(username, password) => {
                radio
                    .write_channel(AuthStateChannel::Main)
                    .login_status
                    .set_loading(None);

                gloo_timers::future::TimeoutFuture::new(2000).await;

                let mut r = use_radio::<AuthState, AuthStateChannel>(AuthStateChannel::Main);

                r.write().login_status.set_failed(None);

                gloo_timers::future::TimeoutFuture::new(2000).await;
                radio
                    .write_silently()
                    .login_status
                    .set_success(Some(true), None);

                // radio.async_apply(action);

                ChannelSelection::Select(AuthStateChannel::Main)
            }
        }
    }
}
