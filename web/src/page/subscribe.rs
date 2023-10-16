use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::AppRoute;

#[function_component(Page)]
pub fn html() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = Callback::from(move |_| navigator.push(&AppRoute::Home));
    
    html! {
        <div>
            <h1>{ "subscribe" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}