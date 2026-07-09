use dita_design_system::components::core::button::{Button, ButtonSize, ButtonVariant};
use dita_design_system::svg_icons::dita_logo::SvgDitaLogo;
use icons::{Ellipsis, Layers, University, UserCog, UserRound};
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_ui::clx;
use leptos_ui::tw_merge;

struct App {
    title: &'static str,
    component: Box<dyn Fn() -> AnyView + 'static + Send + Sync>,
    href: &'static str,
    disabled: bool,
}

#[component]
pub fn Sidebar() -> impl IntoView {
    const SHARED_TOOLTIP_CLASS: &str = r#"
        relative tooltip__shortfix__display group after:content-[attr(data-tooltip)] after:fixed
        after:left-12 after:py-1 after:px-2 after:text-xs after:whitespace-nowrap after:rounded
        after:border after:shadow-md after:opacity-0 after:transition-opacity after:duration-200
        after:pointer-events-none after:z-[1000000] after:bg-popover after:text-popover-foreground
        hover:after:opacity-100 focus-visible:after:opacity-100 flex items-center justify-center
        "#;

    let apps = vec![
        App {
            title: "User Portal",
            component: Box::new(|| view! { <UserRound /> }.into_any()),
            href: "/user",
            disabled: false,
        },
        App {
            title: "Institution Portal",
            component: Box::new(|| view! { <University /> }.into_any()),
            href: "/institution",
            disabled: false,
        },
        App {
            title: "Service Portal",
            component: Box::new(|| view! { <Layers /> }.into_any()),
            href: "/service",
            disabled: true,
        },
        App {
            title: "Admin Portal",
            component: Box::new(|| view! { <UserCog /> }.into_any()),
            href: "/admin",
            disabled: true,
        },
    ]
    .into_iter()
    .map(|app| {
        view! {
            <SidebarMenuItem attr:data-sidenav="menu-item">
                <SidebarMenuButton
                    variant=SidebarMenuButtonVariant::Outline
                    attr:data-sidenav="menu-button"
                    class=SHARED_TOOLTIP_CLASS
                    attr:data-tooltip={app.title.to_string()}
                    href={app.href.to_string()}
                    attr:rel="external"
                    attr:aria-disabled={app.disabled.to_string()}
                >
                    {(app.component)()}
                </SidebarMenuButton>
            </SidebarMenuItem>
        }
    })
    .collect_view();

    view! {
        <style>
            {r#"
            /* CSS Counter-based Tooltip Positioning System */
            :root {
                --tooltip-header-offset: 32px;
                --tooltip-gap-after-header: 52px;
                --tooltip-item-height: 36px;
            }
            .tooltip___base {
                counter-reset: counter-base;
            }
            .tooltip__shortfix__display {
                counter-increment: counter-base;
            }
            .tooltip__shortfix__display::after {
                top: calc(var(--tooltip-header-offset) + var(--tooltip-gap-after-header) + (counter(counter-base) - 1) * var(--tooltip-item-height)) !important;
            }
            "#}
        </style>

        <div class="border-r border-border h-full flex flex-col">
            <SidebarHeader>
                <SidebarMenu attr:data-sidenav="menu">
                    <SidebarMenuItem attr:data-sidenav="menu-item">
                        <A
                            attr:data-sidenav="menu-button"
                            attr:class=format!("md:p-0 {}", SHARED_TOOLTIP_CLASS)
                            href="/"
                            attr:rel="external"
                            attr:data-tooltip="DiTA"
                        >
                            <div class="flex justify-center items-center rounded-lg bg-sidenav-primary text-sidenav-primary-foreground aspect-square size-8 [&_svg:not([class*='size-'])]:size-4">
                                <SvgDitaLogo class="size-6" />
                            </div>
                        </A>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarHeader>
            <SidebarContent attr:data-sidenav="content">
                <SidebarGroup attr:data-sidenav="group">
                    <SidebarGroupContent attr:data-sidenav="group-content">
                        <SidebarMenu attr:data-sidenav="menu" class="tooltip___base">
                            {apps}
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
            <SidebarFooter attr:data-sidenav="footer">
                <SidebarMenu attr:data-sidenav="menu">
                    <SidebarMenuItem attr:data-sidenav="menu-item">
                        <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon>
                            <Ellipsis />
                        </Button>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarFooter>
        </div>
    }
}

clx! {SidebarHeader, div, "flex flex-col gap-1 p-1"}
clx! {SidebarMenu, ul, "flex flex-col gap-1 w-full min-w-0"}
clx! {SidebarMenuItem, li, "relative group/menu-item"}
clx! {SidebarGroup, div, "flex relative flex-col w-full min-w-0"}
clx! {SidebarGroupContent, div, "w-full text-sm"}
clx! {SidebarContent, div, "scrollbar__on_hover", "flex min-h-0 flex-1 flex-col gap-2 group-data-[collapsible=Icon]:overflow-hidden overflow-y-auto"}
clx! {SidebarFooter, footer, "flex flex-col gap-1 p-1"}

leptos_ui::variants! {
    SidebarMenuButton {
        base: r#"
        peer/menu-button flex w-full items-center gap-1 overflow-hidden p-1
        text-left text-sm outline-hidden ring-sidenav-ring transition-[width,height,padding]
        hover:bg-sidenav-accent hover:text-sidenav-accent-foreground focus-visible:ring-2
        active:bg-sidenav-accent active:text-sidenav-accent-foreground disabled:pointer-events-none
        disabled:opacity-50 group-has-data-[sidenav=menu-action]/menu-item:pr-8
        aria-disabled:pointer-events-none aria-disabled:opacity-50
        aria-[current=page]:bg-sidenav-accent aria-[current=page]:font-medium
        aria-[current=page]:text-sidenav-accent-foreground data-[state=open]:hover:bg-sidenav-accent
        data-[state=open]:hover:text-sidenav-accent-foreground [&>span:last-child]:truncate
        [&>svg]:size-4 [&>svg]:shrink-0 group-data-[collapsible=Icon]:size-8!
        group-data-[collapsible=Icon]:p-0! [&>svg]:stroke-[1.5] aria-[current=page]:[&>svg]:stroke-[2.5]
        aria-[current=page]:bg-primary
        "#,
        variants: {
            variant: {
                Default: "sidebar-primary hover:bg-sidenav-accent hover:text-sidenav-accent-foreground", // Already in base
                Outline: "sidebar-primary bg-background shadow-[0_0_0_1px_hsl(var(--sidenav-border))] hover:bg-sidenav-accent hover:text-sidenav-accent-foreground hover:shadow-[0_0_0_1px_hsl(var(--sidenav-accent))]",
            },
            size: {
                Default: "h-8 text-lg",
                Sm: "h-7 text-xs",
                Lg: "h-12",
            }
        },
        component: {
            element: button,
            support_href: true,
            support_aria_current: true
        }
    }
}
