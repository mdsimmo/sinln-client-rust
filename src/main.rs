mod login;
mod members;
mod topics;

use leptos::*;
use leptos_router::*;

use crate::{login::*, members::*, topics::*};

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav><ul>
            <li><a href="/login">"Login"</a></li>
            <li><a href="/members">"Members"</a></li>
            <li><a href="/topics">"Topics"</a></li>
        </ul></nav>
        <Router>
            <Routes>
                <Route path="/"         view=|cx| view! { cx, <Splash/> } />
                <Route path="/*"        view=|cx| view! { cx, <NotFound/> } />
                <Route path="/login"    view=|cx| view! { cx, <Login/> } />
                <Route path="/members"  view=|cx| view! { cx, <Members/> } />
                <Route path="/topics"   view=|cx| view! { cx, <Topics/> } />
            </Routes>
        </Router>
    }
}

#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    view! { cx,
        <p>"Page not found"</p>
    }
}