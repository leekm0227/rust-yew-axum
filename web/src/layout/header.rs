use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{router::AppRoute, state::AppState};

#[function_component(Header)]
pub fn header() -> Html {
    let is_login = use_selector(|s: &AppState| s.is_login());
    let dispatch = Dispatch::<AppState>::new();
    let navigator = use_navigator().unwrap();

    let on_logout = dispatch.reduce_mut_callback(move |s| {
        s.logout();
        navigator.replace(&AppRoute::Home);
    });

    html! {
      <header class="navbar">
          <div class="float-left">
            <Link<AppRoute> to={AppRoute::Home} classes={"btn btn-ghost hover:bg-transparent normal-case text-xl"}>
                <svg xmlns="http://www.w3.org/2000/svg" width="40" height="30">
                    <g>
                        <rect stroke="#ff0000" fill="#ff0000" x="5.74051" y="7.78479" width="28.51898" height="18.78897" id="svg_2" rx="5"/>
                        <path stroke="#ffffff" fill="#ffffff" d="m17.91843,20.16275l-0.02839,-6.37607l5.50765,3.21263l-5.47926,3.16345l0,0z" id="svg_7"/>
                        <path stroke="#ff0000" fill="#ff0000" d="m25.0489,9.19295l3.29526,-5.76671l3.29526,5.76671l-6.59052,0z" id="svg_11"/>
                        <path stroke="#ff0000" fill="#ff0000" d="m8.09787,9.19294l3.29526,-5.7667l3.29526,5.7667l-6.59052,0z" id="svg_12"/>
                    </g>
                </svg>
                {"웁튜브"}
            </Link<AppRoute>>
          </div>
          <div class="flex-1 form-control justify-center flex-row">
              <input type="text" placeholder="검색" class="input input-bordered input-sm w-full max-w-lg" />
              <button class="btn btn-ghost btn-circle -ml-10 hover:bg-transparent">
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                  </svg>
              </button>
          </div>

          if !*is_login {
            <div class="flaot-right mr-2">
                <Link<AppRoute> to={AppRoute::Login} classes={"btn rounded-full"}>
                    <svg class="w-5 h-5 text-gray-500 transition group-hover:text-gray-600" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 18">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="0.5" d="M7 8a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7Zm-2 3h4a4 4 0 0 1 4 4v2H1v-2a4 4 0 0 1 4-4Z"/>
                    </svg>
                    {"로그인"}
                </Link<AppRoute>>
            </div>
          }else{
            <div class="float-right dropdown dropdown-end">
                <label tabindex="0" class="btn btn-ghost btn-circle avatar">
                    <div class="w-10 rounded-full">
                        <img src="https://daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg" />
                    </div>
                </label>
                <ul tabindex="0" class="bg-white p-2 z-10 shadow menu menu-sm dropdown-content rounded-box w-52">
                    <li><a>{"프로필"}</a></li>
                    <li onclick={on_logout}><a>{"로그아웃"}</a></li>
                </ul>
            </div>
          }
      </header>
    }
}
