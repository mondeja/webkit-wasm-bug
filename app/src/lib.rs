use console_error_panic_hook;
use i18n::{gettext, move_gettext, LocaleStateSignal, LANGUAGES};
use leptos::{
    component, create_rw_signal, mount_to_body, provide_context, use_context, view,
    IntoView, RwSignal, Scope, SignalGet,
};
use leptos_router::{
    Route, RouteProps, Router, RouterProps, Routes, RoutesProps,
};
use log::Level;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        view! { cx, <App/> }
    })
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Routes>
                <Route
                    path="/"
                    view=move |cx| {
                        view! { cx, <Index/> }
                    }
                />
            </Routes>
        </Router>
    }
}

#[component]
fn Index(cx: Scope) -> impl IntoView {
    provide_context(cx, LocaleStateSignal(create_rw_signal(cx, LANGUAGES[0])));
    view! { cx, <Header/> }
}

#[derive(Copy, Clone)]
pub struct HeaderStateSignal(pub RwSignal<bool>);

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
    provide_context(cx, HeaderStateSignal(create_rw_signal(cx, false)));

    view! { cx, <HeaderTitle/> }
}

#[component]
pub fn HeaderTitle(cx: Scope) -> impl IntoView {
    let header_state = use_context::<HeaderStateSignal>(cx).unwrap().0;

    view! { cx,
        <div class=move || {
            let mut cls = "flex".to_string();
            if header_state.get() {
                cls.push_str(" hidden");
            }
            cls
        }>
            <h1>"Simple Icons"</h1>
            <p inner_html=move_gettext!(
                cx, "{} free {} icons for popular brands", 2449.to_string().as_str(), &
                format!("<abbr title=\"{}\">{}</abbr>", gettext!(cx, "Scalable Vector Graphic"),
                gettext!(cx, "SVG"),)
            )></p>
        </div>
    }
}
