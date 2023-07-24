use player::{video::Video, video_details::VideoDetails};
use yew::prelude::*;

use crate::player::video_list::VideosList;
pub mod player;
#[function_component(App)]
fn app() -> Html {
    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails
                video ={video.clone()}
            />
        }
    });
    html! {
    <>
    <h1> {  "Guitar yoga website"} </h1>
    <div>
    <h3>{"Lessons to watch"}</h3>
    <VideosList
        videos = {player::fixtures::get_videos()}
        on_click={on_video_select.clone()}
        />
    </div>
    {for details}
    <div>
        <h3> {"footer"}</h3>
    </div>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
