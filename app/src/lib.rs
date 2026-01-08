mod components;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <MyComponent/>
        <button class="text-white px-4 sm:px-8 py-2 sm:py-3 bg-sky-700 hover:bg-sky-800"
           on:click=on_click>"Click Me: "
           <span>{move || count.get()}</span>
        </button>
    }
}


use icons::{ChevronRight, Heart, Star, Menu};

#[component]
pub fn MyComponent() -> impl IntoView {
    view! {
        <ChevronRight />
        <Heart class="text-red-500" />
        <Star class="size-6" />
        <Menu class="size-8 text-gray-700" />
        // Size variations
<Star class="size-4" />      // 16px
<Star class="size-6" />      // 24px
<Star class="size-8" />      // 32px

// Colors
<Heart class="text-red-500" />
// <Check class="text-green-600" />

// Custom styling
<Menu class="size-6 text-gray-900 hover:text-blue-500" />
    }
}