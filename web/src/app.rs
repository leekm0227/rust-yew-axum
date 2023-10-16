use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::{
    layout,
    router::{self, AppRoute},
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <body class="">
                <layout::header::Header />
                <layout::nav::Nav />
                <div class="ml-60 p-8">
                    <Switch<AppRoute> render={router::switch} />
                </div>
            </body>
        </BrowserRouter>
    }
}
