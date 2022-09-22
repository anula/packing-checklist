use yew::prelude::*;

use crate::config::{Config};
use crate::components::{IncorrectConfig};

pub struct ConfigProvider;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

impl Component for ConfigProvider {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            if let Ok(config) = Config::init() {
                <ContextProvider<Config> context={config}>
                  { for ctx.props().children.iter() }
                </ContextProvider<Config>>
            } else {
                <IncorrectConfig location={"ConfigProvider".to_string()}/>
            }
        }
    }
}
