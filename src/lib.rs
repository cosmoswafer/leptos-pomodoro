use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::*;

use wasm_bindgen::prelude::*;

mod components;

use components::pomodoro::PomodoroTimer;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-pomodoro.css"/>
        <Title text="Leptos Pomodoro Timer"/>
        <Meta name="description" content="A beautiful pomodoro timer built with Rust and Leptos"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>

        <Router>
            <main class="app">
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="container">
            <header class="header">
                <h1>"🍅 Leptos Pomodoro Timer"</h1>
                <p>"Stay focused and productive with the Pomodoro Technique"</p>
            </header>

            <PomodoroTimer/>

            <footer class="footer">
                <p>"Built with " <a href="https://leptos.dev" target="_blank">"Leptos"</a> " and Rust 🦀"</p>
            </footer>
        </div>
    }
}

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log!("Starting Leptos Pomodoro Timer app");
    mount_to_body(App);
}
