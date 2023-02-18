mod application;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<application::PeterGow>::new().render();
}
