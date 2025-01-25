use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <main>
      <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
      <h1>{ "Hello World!" }</h1>
      <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
      <div 
        class="p-4 bg-red-500 text-blue-500"
      >
        { "Hello, Tailwind CSS with Yew!fbderggerg" }
      </div>

    </main>
  }
}
