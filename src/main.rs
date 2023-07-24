use gloo_net::http::Request;
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

    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    videos.set(fetched_videos);
                })
            },
            (),
        );
    }
    html! {
    <>
    <h1> {  "Guitar yoga website"} </h1>
    <div>
    <h3>{"Lessons to watch"}</h3>
    <VideosList
        videos = {(*videos).clone()}
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
