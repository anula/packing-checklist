use yew::prelude::*;

use crate::types::{UserAuthStatus, UserInfo};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

pub struct UserContextProvider;

impl Component for UserContextProvider {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let user_auth = UserAuthStatus {
            user_info: Some(UserInfo {
                display_name: "FakeLocalUser".to_string(),
                email: "fakeLocal@email.com".to_string(),
                id_token: "fakeLocalIdToken".to_string(),
                refresh_token: "fakeLocalRefreshToken".to_string(),
            }),
        };
        html!{
            <ContextProvider<UserAuthStatus> context={user_auth}>
              { for ctx.props().children.iter() }
            </ContextProvider<UserAuthStatus>>
        }
    }
}
