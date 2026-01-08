use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::tw_merge;

clx! {Tooltip, div, "inline-block relative mx-0 whitespace-nowrap transition-all duration-300 ease-in-out group/tooltip my-[5px]"}

#[derive(Default, strum::Display)]
pub enum TooltipPosition {
    #[default]
    Top,
    Left,
    Right,
    Bottom,
}

#[component]
pub fn TooltipContent(
    #[prop(into, optional)] class: String,
    #[prop(default = TooltipPosition::default())] data_position: TooltipPosition,
    children: Children,
) -> impl IntoView {
    const SHARED_TRANSITION_CLASSES: &str = "absolute opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover/tooltip:opacity-100 z-[1000000]";

    let tooltip_class = tw_merge!(
        SHARED_TRANSITION_CLASSES,
        "py-2 px-2.5 text-xs whitespace-nowrap shadow-lg text-background bg-foreground/90",
        class,
        match data_position {
            TooltipPosition::Top => "left-1/2 bottom-full mb-1 -ml-2.5",
            TooltipPosition::Right => "bottom-1/2 left-full ml-2.5 -mb-3.5",
            TooltipPosition::Bottom => "left-1/2 top-full mt-1 -ml-2.5",
            TooltipPosition::Left => "bottom-1/2 right-full mr-2.5 -mb-3.5",
        },
    );

    let arrow_class = tw_merge!(
        "absolute opacity-0 transition-all duration-300 ease-in-out pointer-events-none group-hover/tooltip:opacity-100 z-[1000000]",
        "bg-transparent border-transparent border-6",
        match data_position {
            TooltipPosition::Top => "left-1/2 bottom-full -mb-2 border-t-foreground/90",
            TooltipPosition::Right => "bottom-1/2 left-full -mr-0.5 -mb-1 border-r-foreground/90",
            TooltipPosition::Bottom => "left-1/2 top-full -mt-2 border-b-foreground/90",
            TooltipPosition::Left => "bottom-1/2 right-full -mb-1 -ml-0.5 border-l-foreground/90",
        },
    );

    view! {
        <>
            <div data-name="TooltipArrow" class=arrow_class />
            <div data-name="TooltipContent" class=tooltip_class>
                {children()}
            </div>
        </>
    }
}

#[component]
pub fn TooltipProvider() -> impl IntoView {
    view! {
        <style>
            {"
            /* JS adds this class to enable interaction */
            [data-name=\"TooltipContent\"].tooltip__interactive,
            [data-name=\"TooltipContent\"]:hover {
            pointer-events: auto;
            }
            "}
        </style>

        <div data-name="__TooltipProvider" />

        <script>
            {"
            // Simplified JS - only handles pointer-events for text selection
            const tooltipTriggers = document.querySelectorAll('[data-name=\"Tooltip\"]');
            
            tooltipTriggers.forEach(trigger => {
            const content = trigger.querySelector('[data-name=\"TooltipContent\"]');
            
            trigger.addEventListener('mouseenter', () => {
            content.classList.add('tooltip__interactive');
            });
            
            trigger.addEventListener('mouseleave', () => {
            setTimeout(() => {
            content.classList.remove('tooltip__interactive');
            }, 100);
            });
            });
            "}
        </script>
    }
}