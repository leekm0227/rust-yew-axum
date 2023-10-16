use serde::Deserialize;
use serde_json::json;
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    component::alert_label::AlertLabel,
    r#const::{BTN_CLASS_REGISTER, INPUT_CLASS_REGISTER},
    router::AppRoute,
    util,
};

#[derive(Deserialize, Clone, Debug)]
struct Result {
    error: Option<String>,
    success: bool,
}

#[function_component(Page)]
pub fn html() -> Html {
    let navigator = use_navigator().unwrap();
    let account = use_state(|| "".to_string());
    let nickname = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let is_confirm_password = use_state(|| false);
    let is_show_alert = use_state(|| false);
    let alert_text = use_state(|| "".to_string());

    let oninput_account = {
        let account = account.clone();

        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
            account.set(input.value());
        }
    };

    let oninput_nickname = {
        let nickname = nickname.clone();

        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
            nickname.set(input.value());
        }
    };

    let oninput_password = {
        let password = password.clone();
        let is_confirm_password = is_confirm_password.clone();

        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
            password.set(input.value());
            is_confirm_password.set(false);
        }
    };

    let oninput_confirm_password = {
        let password = password.clone();
        let is_confirm_password = is_confirm_password.clone();

        move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();

            if !password.deref().eq(&input.value()) {
                input.set_custom_validity("비밀번호가 일치하지 않습니다");
                return;
            }

            is_confirm_password.set(true);
            input.set_custom_validity("");
        }
    };

    let on_register = {
        let navigator = navigator.clone();
        let account = account.clone();
        let nickname = nickname.clone();
        let password = password.clone();
        let is_confirm_password = is_confirm_password.clone();
        let is_show_alert = is_show_alert.clone();
        let alert_text = alert_text.clone();

        move |e: SubmitEvent| {
            e.prevent_default();

            if !is_confirm_password.deref() {
                is_show_alert.set(true);
                alert_text.set("비밀번호가 일치하지 않습니다".to_string());
                return;
            }

            is_show_alert.set(false);
            {
                let navigator = navigator.clone();
                let is_show_alert = is_show_alert.clone();
                let alert_text = alert_text.clone();
                let body = json!({"account":account.deref(), "nickname":nickname.deref(), "password":password.deref()});

                spawn_local(async move {
                    let result = util::request_post::<Result>("auth/register", body).await;

                    if result.success {
                        let window = web_sys::window().unwrap();
                        let _ = window.alert_with_message("회원가입 완료");
                        navigator.replace(&AppRoute::Home);
                    } else {
                        is_show_alert.set(true);
                        alert_text.set(result.error.unwrap_or_default());
                    }
                });
            }
        }
    };

    html! {
        <div class="flex flex-col items-center mt-40 px-6 py-8 mx-auto md:h-screen lg:py-0">
            <div class="w-full bg-white rounded-lg shadow dark:border md:mt-0 sm:max-w-md xl:p-0 dark:bg-gray-800 dark:border-gray-700">
                <div class="p-6 space-y-4 md:space-y-6 sm:p-8">
                    <h1 class="text-xl font-bold leading-tight tracking-tight text-gray-900 md:text-2xl dark:text-white">
                        {"회원가입"}
                    </h1>

                    if *is_show_alert {
                        <AlertLabel text={alert_text.to_string()} />
                    }

                    <form class="space-y-4 md:space-y-6" onsubmit={on_register}>
                        <div>
                            <label for="account" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"아이디"}</label>
                            <input type="text" name="account" id="account" oninput={oninput_account} class={INPUT_CLASS_REGISTER} title="영문/숫자 4~12자" pattern="^[a-z0-9]{4,12}$" placeholder="id" required={true} />
                        </div>
                        <div>
                            <label for="nickname" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"별명"}</label>
                            <input type="text" name="nickname" id="nickname"  oninput={oninput_nickname} class={INPUT_CLASS_REGISTER} title="2~10자" pattern="^.{2,10}$" placeholder="nickname" required={true} />
                        </div>
                        <div>
                            <label for="password" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"비밀번호"}</label>
                            <input type="password" name="password" id="password" placeholder="••••••••"  oninput={oninput_password} title="영문/숫자를 포함한 4~18자" pattern="^[a-z0-9]{4,18}$" class={INPUT_CLASS_REGISTER} required={true} />
                        </div>
                        <div>
                            <label for="confirm-password" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{"비밀번호 확인"}</label>
                            <input type="password" name="confirm-password" id="confirm-password" placeholder="••••••••"  oninput={oninput_confirm_password} title="4~12 글자" class={INPUT_CLASS_REGISTER} required={true} />
                        </div>
                        <button class={BTN_CLASS_REGISTER}>{"등록"}</button>
                    </form>
                </div>
            </div>
        </div>
    }
}
