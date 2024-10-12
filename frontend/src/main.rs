use common::CollectionIdentifier;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::*;

static BASE_URL: &str = "http://127.0.0.1:9090";

fn collections_url() -> String {
    format!("{}/collections", BASE_URL)
}

#[component]
fn Collection(name: String, index: usize) -> impl IntoView {
    view! {
        <div>
            <h2>{format!("\u{1F4C1} {name}")}</h2>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (collections, set_collections) = create_signal::<Vec<CollectionIdentifier>>(vec![]);
    create_effect(move |_| {
        spawn_local(async move {
            match get_collections().await {
                Ok(result) => set_collections(result),
                Err(message) => {
                    console_log(&format!("failed to get collections due to: {:?}", message))
                }
            };
        });
    });
    view! {
        <div>
        <h1>Home</h1>
        {
            move || {
                collections.get().iter().map(|collection| {
                    // TODO: pass name name index
                    view! { <Collection name=collection.name.clone() index=collection.index/> }
                }).collect::<Vec<_>>()
            }
        }
        </div>
    }
}

async fn get_collections() -> Result<Vec<CollectionIdentifier>, String> {
    let resp = reqwest::get(collections_url())
        .await
        .map_err(|err| format!("failed to get response due to {:?}", err))?
        .json::<Vec<CollectionIdentifier>>()
        .await
        .map_err(|err| format!("failed to databind message due to {:?}", err))?;
    Ok(resp)
}

#[component]
pub fn Test() -> impl IntoView {
    let (message, set_message) = create_signal(String::from("initial value"));
    create_effect(move |_| {
        spawn_local(async move {
            match get_test_str().await {
                Ok(result) => set_message(result),
                Err(message) => set_message(format!("failed to get message due to: {:?}", message)),
            };
        });
    });
    view! {<p>  {message}</p>}
}

pub async fn get_test_str() -> Result<String, String> {
    console_log("test");
    let resp = reqwest::get("http://127.0.0.1:9090")
        .await
        .map_err(|err| format!("failed to get response due to {:?}", err))?
        .json::<common::TestResponse>()
        .await
        .map_err(|err| format!("failed to databind message due to {:?}", err))?;
    console_log(&format!("{:?}", &resp));
    Ok(resp.message)
}

fn main() {
    mount_to_body(|| view! { <Home/>})
}
