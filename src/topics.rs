use app_core::{api::{ListRequest, ListResponse, UpdateRequest}, Topic};
use leptos::{*, leptos_dom::{console_error, console_log}, ev::SubmitEvent, html::*};
use reqwest::{Error, StatusCode};

#[component]
pub fn Topics(cx: Scope) -> impl IntoView {
    let topics_list = create_resource(cx, || (), |_| async move {
        fetch_topics().await.map_err(|e| {
            console_error(&e.to_string());
            e.to_string()} )
    });

    view! { cx,
        <table>
            <thead>
                <tr>
                    <th>"Id"</th>
                    <th>"Name"</th>
                    <th>"Endpoint"</th>
                    <th>"Default"</th>
                </tr>
            </thead>
            <Transition fallback=move || view! {cx, <StatusRow>"Loading..."</StatusRow>}>
                { move || topics_list.read(cx).map(|result| {
                    match result {
                        Ok(topics) => topics.into_iter().map(|topic| view!{ cx,
                                <Topic topic/>
                            }).collect::<Vec<_>>().into_view(cx),
                        Err(e) => view!{ cx, <StatusRow>"Error: "{e}</StatusRow> }.into_view(cx)
                    }
                })}
            </Transition>
            <StatusRow>
                <AddTopic/>
            </StatusRow>
        </table>
    }
}

#[component]
fn StatusRow(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <tr>
            <td colspan=4>{children(cx)}</td>
        </tr>
    }
}

#[component]
fn Topic(cx: Scope, topic: Topic) -> impl IntoView {
    view! { cx, 
        <tr>
            <td>{topic.id}</td>
            <td>{topic.name}</td>
            <td>{topic.endpoint}</td>
            <td>{topic.default}</td>
        </tr>
    }
}

#[component]
fn AddTopic(cx: Scope) -> impl IntoView {
    let name_ref: NodeRef<Input> = create_node_ref(cx);
    let endpoint_ref: NodeRef<Input> = create_node_ref(cx);
    let default_ref: NodeRef<Input> = create_node_ref(cx);

    let submission = create_action(cx, move |topic: &Topic| {
        let topic = topic.clone();
        console_log(&format!("Submitting: {:?}", topic));
        async move {
            return submit_topic(&topic).await.unwrap_or(StatusCode::BAD_REQUEST);
        }
    });

    let pending = submission.pending();
    let result = submission.value();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let topic = Topic { 
            id: None, 
            name: name_ref().expect("No name given").value(),
            endpoint: endpoint_ref().expect("No endpoint given").value(),
            default: default_ref().expect("No default given").checked()
        };
        submission.dispatch(topic);
    };

    view! { cx, 
        <form on:submit=on_submit>
            <input node_ref=name_ref type="text"/>
            <input node_ref=endpoint_ref type="email"/>
            <input node_ref=default_ref type="checkbox"/>
            <button disabled={move || pending()}>
                {
                    move || if pending() { 
                        "Sending" 
                    } else if let Some(status) = result() { 
                        console_log(&format!("{}", status));
                        if status.is_success() { 
                            "Sent"
                        } else {
                            "Error"
                        } 
                    } else {
                        "Send"
                    }
                }
            </button>
        </form>
    }
}

async fn fetch_topics() -> Result<Vec<Topic>, Error> {
    let req = ListRequest {};
    let client = reqwest::Client::new();
    let resp = client.post("https://api.sinln.mdsimmo.com/topics-list")
        .json(&req)
        .send().await?
        .json::<ListResponse<Topic>>()
        .await?;
    Ok(resp.items)
}

async fn submit_topic(topic: &Topic) -> Result<StatusCode, Error> {
    console_log(&format!("Topic: {:?}", topic));
    let req = UpdateRequest {
        values: vec![topic.clone()],
    };
    let client = reqwest::Client::new();
    let resp = client.post("https://api.sinln.mdsimmo.com/topics-update")
        .json(&req)
        .send().await?;
    let status = resp.status();
    let data = resp.text().await?;
    console_log(&format!("{:?} Data: {:?}", status, data));
    Ok(status)
}