use app_core::{Member, api::UpdateRequest};
use leptos::{*, html::*, leptos_dom::console_log, ev::SubmitEvent};
use reqwest::{StatusCode, Error};


#[component]
pub fn Splash(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>"Welcome to Noble Park Baptist!!"</p> 
        <a href="/login">"Sine In"</a>
    }
}

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let name:        NodeRef<Input> = create_node_ref(cx);
    let email:       NodeRef<Input> = create_node_ref(cx);
    let address:     NodeRef<Input> = create_node_ref(cx);
    let mobile:      NodeRef<Input> = create_node_ref(cx);
    let sub_a:       NodeRef<Input> = create_node_ref(cx);
    let sub_b:       NodeRef<Input> = create_node_ref(cx);

    let submission = create_action(cx, move |member: &Member| {
        let member = member.clone();
        console_log(&format!("Submitting: {:?}", member));
        async move {
            return submit_member(&member).await.unwrap_or(StatusCode::BAD_REQUEST);
        }
    });
    
    let pending = submission.pending();
    let result = submission.value();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let subs: Vec<String> = vec![
            sub_a(),
            sub_b(),
        ].iter()
            .filter_map(|element| 
                element.as_ref().and_then(|ele| {
                    if ele.checked() {
                        Some(ele.name())
                    } else {
                        None
                    }
                })
            ).collect();

        let member = Member { 
            id: None, 
            name: name().expect("No name given").value(),
            email: email().expect("No email given").value(),
            address: address().map(|it| it.value()),
            mobile: mobile().map(|it| it.value_as_number() as u64),
            subscriptions: subs
        };

        submission.dispatch(member);
    };

    view! { cx, 
        <form method="get" action="/" on:submit=on_submit>
            <input node_ref=name    type="text"      />
            <input node_ref=email   type="email"     />
            <input node_ref=mobile  type="number"   placeholder="mobile" />
            <input node_ref=sub_a   type="checkbox" name="sub-a" />
            <input node_ref=sub_b   type="checkbox" name="sub-b" />
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


async fn submit_member(member: &Member) -> Result<StatusCode, Error> {
    console_log(&format!("Member: {:?}", member));
    let req = UpdateRequest {
        values: vec![member.clone()],
    };
    let client = reqwest::Client::new();
    let resp = client.post("https://api.sinln.mdsimmo.com/members-update")
        .json(&req)
        .send().await?;
    let status = resp.status();
    let data = resp.text().await?;
    console_log(&format!("{:?} Data: {:?}", status, data));
    Ok(status)
}