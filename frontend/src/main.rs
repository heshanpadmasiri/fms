use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::*;

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
    mount_to_body(|| view! { <Test/>})
}
