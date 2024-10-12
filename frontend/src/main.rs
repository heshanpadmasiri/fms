use common::CollectionIdentifier;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::*;
use leptos_router::*;

static BASE_URL: &str = "http://127.0.0.1:9090";

fn collections_url() -> String {
    format!("{}/collections", BASE_URL)
}

#[component]
fn App() -> impl IntoView {
    view! {
    <Router>
        <main>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/collection/:id" view=Collection/>
                <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
        </main>
    </Router>
    }
}

#[component]
fn Collection() -> impl IntoView {
    let cx = use_route();
    let params = cx.params().get_untracked();
    let index = params
        .get("id")
        .expect("id must be passed in as param from router");
    view! {
        <div>
            <h1>{format!("Collection {}", index)}</h1>
            <a href="/">Home</a>
        </div>
    }
}

#[component]
fn CollectionCard(name: String, index: usize) -> impl IntoView {
    view! {
        <div>
            <h2>
            <a href=format!("/collection/{index}")>
                {format!("\u{1F4C1} {name}")}
            </a>
        </h2>
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
                    view! { <CollectionCard name=collection.name.clone() index=collection.index/> }
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
    mount_to_body(|| view! { <App/>})
}
