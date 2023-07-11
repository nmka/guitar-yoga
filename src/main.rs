use yew::prelude::*;

use crate::player::video::VideosList;
pub mod player;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>

    <h1>{ "Guitar yoga website"} </h1>
    <div>

     <VideosList videos = {player::fixtures::get_videos()} />
        </div></>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
