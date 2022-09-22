use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IncorrectConfigProp {
    pub location: String
}

#[function_component(IncorrectConfig)]
pub fn incorrect_config(props: &IncorrectConfigProp) -> Html {
    let error_string = format!("Can't access config from '{}'. Make sure that you provide the \
        full config in either JSON format (in .env file) or the environemnt variables. \
        Note that all fields from crate::config::Config have to be present for the config to \
        correctly parse.", props.location);
    html! {
        <p class="container"> {error_string} </p>
    }
}
