use crate::components::core::avatar::{Avatar, AvatarImage};
use crate::components::core::breadcrumb::{
    Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::components::core::button::{Button, ButtonSize, ButtonVariant};
use crate::components::core::separator::{Separator, SeparatorOrientation};
use crate::hooks::header::HeaderMode;
use crate::hooks::sidenav::SidenavMode;
use crate::hooks::theme::ThemeMode;
use icons::{PanelLeftClose, PanelLeftOpen, SvgIcon};
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {

    let ctx = HeaderMode::new();
    let crumbs = ctx.get_crumbs();
    let title = ctx.get_page_title();

    view! {
        <header class="sticky top-0 z-50 flex h-12 items-center bg-background px-4 w-full">
            <div class="flex items-center gap-2">
                <SidenavToggle />
                <Separator orientation=SeparatorOrientation::Vertical class="-ml-1 h-4" />

                <Breadcrumb>
                    <BreadcrumbList>
                        <For
                            each= move || crumbs.get()
                            key=|item| item.clone()
                            children=move |item| {
                                view! {
                                    <BreadcrumbItem>
                                        <BreadcrumbLink attr:href={item.path}>{item.title}</BreadcrumbLink>
                                    </BreadcrumbItem>
                                    <BreadcrumbSeparator />
                                }
                            }
                        />
                        <BreadcrumbItem>
                            <BreadcrumbPage>{move || title.get()}</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            </div>

            <div class="flex-1 px-4">
                {/* مثلاً portal name فعلی */}
            </div>

            <div class="flex items-center gap-3">
                <ThemeToggle />
                <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon class="rounded-full p-0">
                    <Avatar>
                        <AvatarImage attr:src="https://cdn.jsdelivr.net/gh/alohe/avatars/png/memo_34.png" attr:alt="@rustify" />
                    </Avatar>
                </Button>
            </div>
        </header>
    }
}

#[component]
fn SidenavToggle() -> impl IntoView {
    let ctx = SidenavMode::new();

    view! {
        <Button
            on:click=move |_| ctx.toggle()
            variant=ButtonVariant::Ghost
            size=ButtonSize::IconXs
            class="inline-flex gap-2 justify-center items-center -ml-1 text-sm font-medium whitespace-nowrap rounded-md transition-all outline-none disabled:opacity-50 disabled:pointer-events-none [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 aria-invalid:ring-destructive/20 aria-invalid:border-destructive dark:aria-invalid:ring-destructive/40 dark:hover:bg-accent/50 hover:bg-accent hover:text-accent-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
        >
            <Show
                when=move || ctx.is_open().get()
                fallback=move || view! { <PanelLeftOpen /> }
            >
                <PanelLeftClose />
            </Show>
        </Button>
    }
}

#[component]
fn ThemeToggle() -> impl IntoView {
    let ctx = ThemeMode::new();

    view! {
        <style>
            {"
            .theme__toggle_transition {
                -webkit-tap-highlight-color: transparent;

                svg path {
                    transform-origin: center;
                    transition: all .6s ease;
                    transform: translate3d(0,0,0);
                    backface-visibility: hidden;

                    &.sun {
                        transform: scale(.4) rotate(60deg);
                        opacity: 0;
                    }

                    &.moon {
                        opacity: 1;
                    }
                }

                &.switch {
                    svg path {
                        &.sun {
                            transform: scale(1) rotate(0);
                            opacity: 1;
                        }

                        &.moon {
                            transform: scale(.4) rotate(-60deg);
                            opacity: 0;
                        }
                    }
                }
            }
            "}
        </style>
        <Button
            class=Signal::derive(move || {
                let base_class = "theme__toggle_transition";
                if ctx.is_dark().get() { format!("{base_class} switch") } else { base_class.to_string() }
            })
            on:click=move |_| ctx.toggle()
            variant=ButtonVariant::Ghost
            size=ButtonSize::IconXs
        >
            <SvgIcon class="size-4">
                <path
                    d="M12 1.75V3.25M12 20.75V22.25M1.75 12H3.25M20.75 12H22.25M4.75216 4.75216L5.81282 5.81282M18.1872 18.1872L19.2478 19.2478M4.75216 19.2478L5.81282 18.1872M18.1872 5.81282L19.2478 4.75216M16.25 12C16.25 14.3472 14.3472 16.25 12 16.25C9.65279 16.25 7.75 14.3472 7.75 12C7.75 9.65279 9.65279 7.75 12 7.75C14.3472 7.75 16.25 9.65279 16.25 12Z"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    class="sun text-neutral-300"
                />
                <path
                    d="M2.75 12C2.75 17.1086 6.89137 21.25 12 21.25C16.7154 21.25 20.6068 17.7216 21.1778 13.161C20.1198 13.8498 18.8566 14.25 17.5 14.25C13.7721 14.25 10.75 11.2279 10.75 7.5C10.75 5.66012 11.4861 3.99217 12.6799 2.77461C12.4554 2.7583 12.2287 2.75 12 2.75C6.89137 2.75 2.75 6.89137 2.75 12Z"
                    stroke="currentColor"
                    stroke-width="1.5"
                    stroke-linejoin="round"
                    class="moon text-neutral-700"
                />
            </SvgIcon>
        </Button>
    }
}
