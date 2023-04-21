use app_core::{api::{ListRequest, ListResponse}, Member};
use leptos::{*, leptos_dom::console_error};
use reqwest::Error;

#[component]
pub fn Members(cx: Scope) -> impl IntoView {
    let members_list = create_resource(cx, || (), |_| async move {
        fetch_members().await.map_err(|e| {
            console_error(&e.to_string());
            e.to_string()} )
    });

    view! { cx,
        <table>
            <thead>
                <tr>
                    <th>"Id"</th>
                    <th>"Name"</th>
                    <th>"Email"</th>
                    <th>"Address"</th>
                    <th>"Mobile"</th>
                    <th>"Subscriptions"</th>
                </tr>
            </thead>
            <Transition fallback=move || view! {cx, <StatusRow>"Loading..."</StatusRow>}>
                { move || members_list.read(cx).map(|result| {
                    match result {
                        Ok(members) => members.into_iter().map(|member| view!{ cx,
                                <Member member/>
                            }).collect::<Vec<_>>().into_view(cx),
                        Err(e) => view!{ cx, <StatusRow>"Error: "{e}</StatusRow> }.into_view(cx)
                    }
                })}
            </Transition>
        </table>
    }
}

#[component]
fn StatusRow(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <tr>
            <td colspan=6>{children(cx)}</td>
        </tr>
    }
}

#[component]
fn Member(cx: Scope, member: Member) -> impl IntoView {
    view! { cx, 
        <tr>
            <td>{member.id}</td>
            <td>{member.name}</td>
            <td>{member.email}</td>
            <td>{member.address}</td>
            <td>{member.mobile}</td>
            <td>{member.subscriptions}</td>
        </tr>
    }
}

async fn fetch_members() -> Result<Vec<Member>, Error> {
    let req = ListRequest {};
    let client = reqwest::Client::new();
    let resp = client.post("https://api.sinln.mdsimmo.com/members-list")
        .json(&req)
        .send().await?
        .json::<ListResponse<Member>>()
        .await?;
    Ok(resp.items)
}