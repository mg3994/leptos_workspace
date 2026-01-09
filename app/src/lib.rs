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


use icons::{CircleUser, House, SlidersHorizontal, Wallet};

use strum::{Display, EnumIter, IntoEnumIterator};

use crate::components::ui::bottom_nav::{BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel};

// * See enum below.

#[component]
pub fn MyComponent() -> impl IntoView {
    let active_page_signal = RwSignal::new(NavPage::default());

    view! {
        <div class="flex flex-col my-10 rounded-t-2xl border h-[300px] w-[400px]">
           // --- THE CONTENT AREA ---
            <div class="flex-1 bg-gray-50 rounded-t-2xl p-4">
                {move || match active_page_signal.get() {
                    NavPage::Home => view! { <HomeView /> }.into_any(),
                    NavPage::Wallet => view! { <WalletView /> }.into_any(),
                    NavPage::Settings => view! { <SettingsView /> }.into_any(),
                    NavPage::Profile => view! { <ProfileView /> }.into_any(),
                }}
            </div>
            // ------------------------

            <BottomNav>
                <BottomNavGrid>
                    {NavPage::iter()
                        .map(|page| {
                            view! {
                                <BottomNavButton
                                    on:click=move |_| active_page_signal.set(page)
                                    attr:aria-current=move || {
                                        if active_page_signal.get() == page { "page" } else { "" }
                                    }
                                >

                                    {page.icon()}
                                    <BottomNavLabel>{page.to_string()}</BottomNavLabel>
                                </BottomNavButton>
                            }
                        })
                        .collect_view()}
                </BottomNavGrid>
            </BottomNav>
        </div>
    }
}

/* ========================================================== */
/*                       âœ¨  ENUM  âœ¨                         */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, Default)]
enum NavPage {
    Home,
    #[default]
    Wallet,
    Settings,
    Profile,
}

impl NavPage {
    fn icon(self) -> impl IntoView {
        match self {
            NavPage::Home => view! { <House class="size-5" /> }.into_any(),
            NavPage::Wallet => view! { <Wallet class="size-5" /> }.into_any(),
            NavPage::Settings => view! { <SlidersHorizontal class="size-5" /> }.into_any(),
            NavPage::Profile => view! { <CircleUser class="size-5" /> }.into_any(),
        }
    }
}


#[component]
fn HomeView() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 animate-in fade-in duration-300">
            <h2 class="text-xl font-bold text-gray-800">"Dashboard"</h2>
            <div class="grid grid-cols-2 gap-3">
                <div class="p-4 bg-white rounded-xl border shadow-sm">"Activity"</div>
                <div class="p-4 bg-white rounded-xl border shadow-sm">"Stats"</div>
            </div>
        </div>
    }
}

#[component]
fn WalletView() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 animate-in slide-in-from-bottom-2 duration-300">
            <div class="bg-indigo-600 p-6 rounded-2xl text-white shadow-lg">
                <p class="text-sm opacity-80">"Total Balance"</p>
                <h2 class="text-3xl font-mono font-bold">"$4,250.00"</h2>
            </div>
            <div class="space-y-2">
                <div class="flex justify-between p-3 bg-white rounded-lg border">
                    <span>"Bitcoin"</span>
                    <span class="font-bold text-green-600 font-mono text-sm">"+2.4%"</span>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SettingsView() -> impl IntoView {
    view! {
        <div class="space-y-4 animate-in fade-in duration-300">
            <h2 class="text-xl font-bold">"Settings"</h2>
            <div class="flex flex-col gap-2">
                <button class="w-full text-left p-3 bg-white border rounded-lg">"Dark Mode"</button>
                <button class="w-full text-left p-3 bg-white border rounded-lg">"Notifications"</button>
            </div>
        </div>
    }
}

#[component]
fn ProfileView() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-4 py-6 animate-in zoom-in-95 duration-300">
            <div class="size-20 bg-gray-300 rounded-full border-4 border-white shadow-md" />
            <div class="text-center">
                <h2 class="text-lg font-bold">"Alex Rustacean"</h2>
                <p class="text-sm text-gray-500">"alex@leptos.dev"</p>
            </div>
        </div>
    }
}