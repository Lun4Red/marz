use leptos::*;
use leptos_router::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,

        <div class="navbar">
        <ul>
            <li><A href="/home">"HOME"</A></li>
            <li><A href="#">"ABOUT"</A></li>
            <li><A href="/app">"APP"</A></li>
        </ul>
        </div>


        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
