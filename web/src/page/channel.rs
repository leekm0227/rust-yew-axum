use yew::prelude::*;

#[function_component(Page)]
pub fn html() -> Html {
    html! {
        <div>
            <h1>{ "my channel" }</h1>
            <button>{ "Go Home" }</button>
        </div>
    }
}