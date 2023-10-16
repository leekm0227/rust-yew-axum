use yew::prelude::*;

#[function_component(CardVideo)]
pub fn html() -> Html {
    html! {
        <div class="card card-compact m-4">
          <figure class="rounded-lg">
            <img src="https://daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg" alt="Shoes" />
          </figure>
          <div class="px-2 pt-1">
            <p class="mb-1 text-base font-medium">{"title title title"}</p>
            <div class="text-sm font-light">
              <p class="-mb-1">{"nickname"}</p>
              <p>{"조회수 3.1만회 • 오늘"}</p>
            </div>
        </div>
      </div>
    }
}
