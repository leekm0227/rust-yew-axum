use serde::Deserialize;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::Dispatch;

use crate::{
    r#const::{BTN_CLASS_REGISTER, INPUT_CLASS_REGISTER},
    router::AppRoute,
    state::AppState,
    util,
};

#[derive(Deserialize, Clone, Debug)]
struct Result {
    error: Option<String>,
    success: bool,
    token: Option<String>,
}

#[function_component(Page)]
pub fn html() -> Html {
    let navigator = use_navigator().unwrap();
    let dispatch = Dispatch::<AppState>::new();
    let account_input_ref = use_node_ref();
    let password_input_ref = use_node_ref();

    let on_login = {
        let account_input_ref = account_input_ref.clone();
        let password_input_ref = password_input_ref.clone();

        move |e: SubmitEvent| {
            e.prevent_default();

            let account_input = account_input_ref.cast::<HtmlInputElement>().unwrap();
            let password_input = password_input_ref.cast::<HtmlInputElement>().unwrap();
            let body = json!({"account":account_input.value(), "password":password_input.value()});
            let navigator = navigator.clone();
            let dispatch = dispatch.clone();

            spawn_local(async move {
                let result = util::request_post::<Result>("auth/login", body).await;

                if result.success {
                    dispatch.reduce_mut(move |s| {
                        s.login(result.token.unwrap());
                        navigator.replace(&AppRoute::Home);
                    });
                } else {
                    let window = web_sys::window().unwrap();
                    let _ = window.alert_with_message(result.error.unwrap().as_str());
                }
            });
        }
    };

    html! {
        <div class="flex flex-col items-center mt-40 px-6 py-8 mx-auto md:h-screen lg:py-0">
            <div class="w-full bg-white rounded-lg shadow dark:border md:mt-0 sm:max-w-md xl:p-0">
                <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
                    <h1 class="text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white">
                        {"로그인"}
                    </h1>
                    <form class="space-y-4 md:space-y-6" onsubmit={on_login}>
                        <div>
                            <label for="account" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"아이디"}</label>
                            <input type="text" name="account" id="account" class={INPUT_CLASS_REGISTER} ref={account_input_ref} placeholder="id" required={true} />
                        </div>
                        <div>
                            <label for="password" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"비밀번호"}</label>
                            <input type="password" name="password" id="password" placeholder="••••••••" title="영문/숫자를 포함한 4~18자" pattern="^[a-z0-9]{4,18}$" class={INPUT_CLASS_REGISTER} ref={password_input_ref} required={true} />
                        </div>
                        <button class={BTN_CLASS_REGISTER}>{"로그인"}</button>
                        <Link<AppRoute> to={AppRoute::Register} classes={BTN_CLASS_REGISTER}>{"회원가입"}</Link<AppRoute>>
                    </form>
                </div>
            </div>
        </div>
    }
}
