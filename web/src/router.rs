use yew::prelude::*;
use yew_router::prelude::*;

use crate::page::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum AppRoute {
    #[not_found]
    #[at("/error")]
    Error,
    #[at("/")]
    Home,
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    #[at("/subscribe")]
    Subscribe,
    #[at("/channel")]
    Channel,
    #[at("/history")]
    History,
    #[at("/like")]
    Like,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Error => html! { <error::Page /> },
        AppRoute::Home => html! { <home::Page /> },
        AppRoute::Register => html! { <register::Page />},
        AppRoute::Login => html! { <login::Page />},
        AppRoute::Subscribe => html! { <subscribe::Page />},
        AppRoute::Channel => html! { <channel::Page />},
        AppRoute::History => html! { <history::Page />},
        AppRoute::Like => html! { <like::Page />},
    }
}
