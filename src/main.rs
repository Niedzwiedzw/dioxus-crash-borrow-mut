use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(app);
}

pub fn Counter(cx: Scope) -> Element {
    let counter = use_shared_state::<u32>(cx).expect("counter not initialized");
    cx.render(rsx! { div { class: "Counter", "current: {counter.read()}" } })
}

fn CounterDropdown(cx: Scope) -> Element {
    let counter = use_shared_state::<u32>(cx).expect("counter not initialized");
    let controller_content = |open: bool| {
        let indicator = match open {
            true => "close /\\",
            false => "open \\/",
        };
        cx.render(rsx! {
            Counter {}
            button { "{indicator}" }
        })
    };
    let dropdown_content = || {
        cx.render(rsx! {
            button { onclick: |_| {
                    let old = *counter.read();
                    *counter.write() = old + 1;
                },
                "click to [+1]"
            }
        })
    };
    cx.render(rsx! {
        DropdownMenu {
            dropdown_content: dropdown_content(),
            controller_open: controller_content(true),
            controller_closed: controller_content(false)
        }
    })
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider::<u32>(cx, || 42);
    cx.render(rsx! { CounterDropdown {} })
}

#[derive(Props)]
pub struct DropdownMenuProps<'children> {
    dropdown_content: Element<'children>,
    controller_open: Element<'children>,
    controller_closed: Element<'children>,
}

pub fn DropdownMenu<'children>(cx: Scope<'children, DropdownMenuProps<'children>>) -> Element {
    let open = use_state(cx, || false);
    let onclick = |_| open.set(!open.get());

    cx.render(rsx!(
        div { class: "DropdownMenu",
            div { class: "controller-content", onclick: onclick,
                if *open.get() {
                    &cx.props.controller_open
                } else {
                    &cx.props.controller_closed
                }
            }
            div { class: "dropdown-content", open.get().then(|| cx.props.dropdown_content.as_ref()) }
        }
    ))
}
