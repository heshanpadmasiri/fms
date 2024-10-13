use common::{Collection, CollectionIdentifier};
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

#[derive(Params, PartialEq)]
struct CollectionParams {
    id: Option<usize>,
}

#[component]
fn Collection() -> impl IntoView {
    let (collection, set_collection) = create_signal::<Option<Collection>>(None);
    let params = use_params_map();
    create_resource(
        move || params.with(|p| p.get("id").cloned().unwrap_or_default()),
        move |id| async move {
            let index = id.parse::<usize>().unwrap();
            match get_collection(index).await {
                Ok(result) => set_collection(Some(result)),
                Err(message) => {
                    console_log(&format!("failed to get collection due to: {:?}", message))
                }
            };
        },
    );
    view! {
        <div>
            <a href="/">Home</a>
            {
                move || {
                collection.with(|col| {
                 match col {
                    Some(c) => view!{<div> <CollectionWrapper collection=CollectionData::from(c) /></div>},
                    None => view!{<div> <p> "loading" </p></div>}
                }
            })
                }
            }
        </div>
    }
}

#[derive(Debug, Clone)]
struct CollectionData {
    name: String,
    files: Vec<FileData>,
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Image,
    Video,
}

#[derive(Debug, Clone, Copy)]
struct FileData {
    index: usize,
    kind: Kind,
}

impl From<&common::File> for FileData {
    fn from(file: &common::File) -> Self {
        match file.kind {
            common::FileKind::Image => FileData {
                index: file.index,
                kind: Kind::Image,
            },
            common::FileKind::Video => FileData {
                index: file.index,
                kind: Kind::Video,
            },
            _ => panic!("unsupported file type"),
        }
    }
}

impl From<&Collection> for CollectionData {
    fn from(collection: &Collection) -> Self {
        CollectionData {
            name: collection.name.clone(),
            files: collection.files.iter().map(FileData::from).collect(),
        }
    }
}

#[component]
fn CollectionWrapper(collection: CollectionData) -> impl IntoView {
    if collection.files.is_empty() {
        return view! {
            <div>
                <h1>{collection.name}</h1>
                <p> "No files in collection" </p>
            </div>
        };
    }
    let (current_file_index, set_current_file_index) = create_signal(0);
    let last_index = collection.files.len() - 1;
    let next_index_loop = move || {
        let current_index = current_file_index.get();
        if current_index == last_index {
            0
        } else {
            current_index + 1
        }
    };
    let next_index = move || std::cmp::min(last_index, current_file_index.get() + 1);
    let prev_index = move || std::cmp::max(0, current_file_index.get() - 1);
    view! {
        <div>
        <h1>{collection.name}</h1>
            {
                move || {
                    view! {
                        <div>
                        <p> {format!("File {} of {}", current_file_index.get() + 1, last_index + 1)} </p>
                        </div>
                    }
                }
            }
            <div class="flex justify-evenly space-x-4">
                <button class="bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600" on:click=move |_| set_current_file_index(0)> "First" </button>
                <button class="bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600" on:click=move |_| set_current_file_index(prev_index())> "Previous" </button>
                <button class="bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600" on:click=move |_| set_current_file_index(next_index())> "Next" </button>
                <button class="bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600" on:click=move |_| set_current_file_index(last_index)> "Last" </button>
            </div>
            {
                move || {
                    let f = &collection.files[current_file_index.get()];
                    match f.kind {
                        Kind::Image => view! { <div on:click=move |_| set_current_file_index(next_index_loop())> <Image  index=f.index /> </div> },
                        Kind::Video => view! { <div> <Video index=f.index  /> </div>}

                    }
                }
            }
        </div>
    }
}

#[component]
fn Image(index: usize) -> impl IntoView {
    view! {
        <div class="flex items-center justify-center h-screen">
            <img class="max-w-full max-h-full object-contain" src=format!("{}/file/{}", BASE_URL, index) />
        </div>
    }
}

#[component]
fn Video(index: usize) -> impl IntoView {
    view! {
        <div class="flex items-center justify-center h-screen">
            <video class="max-w-full max-h-full object-contain" controls autoplay>
                <source src=format!("{}/file/{}", BASE_URL, index) type="video/mp4" />
            </video>
        </div>
    }
}

async fn get_collection(index: usize) -> Result<common::Collection, String> {
    let resp = reqwest::get(format!("{}/collection/{}", BASE_URL, index))
        .await
        .map_err(|err| format!("failed to get response due to {:?}", err))?
        .json::<common::Collection>()
        .await
        .map_err(|err| format!("failed to databind message due to {:?}", err))?;
    Ok(resp)
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
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/>})
}
