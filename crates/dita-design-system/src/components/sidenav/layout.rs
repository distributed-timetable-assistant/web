use leptos::prelude::*;
use leptos_router::components::A;
use leptos_ui::clx::tw_merge;
use leptos_ui::{clx, variants, void};

mod components {
    use super::*;
    use leptos_ui::tw_merge;

    // clx! {SidenavInset, div, "bg-background relative flex w-full flex-1 flex-col md:peer-data-[variant=Inset]:m-2 md:peer-data-[variant=Inset]:ml-0 md:peer-data-[variant=Inset]:rounded-xl md:peer-data-[variant=Inset]:shadow-sm md:peer-data-[variant=Inset]:peer-data-[state=Collapsed]:ml-2"}
    clx! {SidenavInset, div, "bg-background relative flex w-full flex-1 flex-col data-[variant=Inset]:rounded-lg data-[variant=Inset]:border data-[variant=Inset]:border-sidenav-border data-[variant=Inset]:shadow-sm data-[variant=Inset]:m-2"}
    // * data-[], not group-data-[]
    clx! {SidenavInner, div, "flex flex-col w-full h-full bg-sidenav data-[variant=Floating]:rounded-lg data-[variant=Floating]:border data-[variant=Floating]:border-sidenav-border data-[variant=Floating]:shadow-sm"}
    clx! {SidenavHeader, div, "flex flex-col gap-2 p-2"}
    clx! {SidenavMenu, ul, "flex flex-col gap-1 w-full min-w-0"}
    clx! {SidenavMenuSub, ul, "border-sidenav-border mx-3.5 flex min-w-0 translate-x-px flex-col gap-1 border-l px-2.5 py-0.5 group-data-[collapsible=Icon]:hidden"}
    clx! {SidenavMenuItem, li, "relative group/menu-item"}
    clx! {SidenavMenuSubItem, li, "group/menu-sub-item"}
    clx! {SidenavContent, div, "scrollbar__on_hover", "flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=Icon]:overflow-hidden"}
    clx! {SidenavGroup, div, "flex relative flex-col p-2 w-full min-w-0"}
    clx! {SidenavGroupContent, div, "w-full text-sm"}
    clx! {SidenavGroupLabel, div, "text-foreground/50 ring-ring flex h-8 shrink-0 items-center rounded-md px-2 text-xs font-medium outline-hidden transition-[margin,opacity] ease-linear focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 group-data-[collapsible=Icon]:-mt-8 group-data-[collapsible=Icon]:opacity-0"}
    clx! {SidenavFooter, footer, "flex flex-col gap-2 p-2"}
    // Button "More"
    clx! {DropdownMenuTriggerEllipsis, button, "text-sidenav-foreground ring-sidenav-ring hover:bg-sidenav-accent hover:text-sidenav-accent-foreground peer-hover/menu-button:text-sidenav-accent-foreground absolute top-1.5 right-1 flex aspect-square w-5 items-center justify-center rounded-md p-0 outline-hidden transition-transform focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 after:absolute after:-inset-2 md:after:hidden peer-data-[size=sm]/menu-button:top-1 peer-data-[size=default]/menu-button:top-1.5 peer-data-[size=lg]/menu-button:top-2.5 group-data-[collapsible=Icon]:hidden peer-data-[active=true]/menu-button:text-sidenav-accent-foreground group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 data-[state=open]:opacity-100 md:opacity-0"}

    void! {SidenavInput, input,
        "file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground dark:bg-input/30 border-input flex h-9 w-full min-w-0 rounded-md border bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm",
        "focus-visible:border-ring focus-visible:ring-ring/50",
        "focus-visible:ring-2", // TODO. Port tw_merge to Tailwind V4.
     // "focus-visible:ring-[3px]", // TODO. Port tw_merge to Tailwind V4.
        "aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive",
        "read-only:bg-muted",
        // Specific to Sidenav
        "w-full h-8 shadow-none bg-background"
    }
}

use crate::hooks::sidenav::SidenavMode;
pub use components::*;

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display, strum::IntoStaticStr)]
pub enum SidenavVariant {
    #[default]
    Sidenav,
    Floating,
    Inset,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum SidenavState {
    #[default]
    Expanded,
    Collapsed,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum SidenavSide {
    #[default]
    Left,
    Right,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, strum::Display)]
pub enum SidenavCollapsible {
    #[default]
    Offcanvas,
    None,
    Icon,
}

#[component]
pub fn Sidenav(
    #[prop(into, optional)] class: String,
    #[prop(default = SidenavVariant::default())] variant: SidenavVariant,
    #[prop(default = SidenavState::default())] data_state: SidenavState,
    #[prop(default = SidenavSide::default())] data_side: SidenavSide,
    #[prop(default = SidenavCollapsible::default())] data_collapsible: SidenavCollapsible,
    children: Children,
) -> impl IntoView {
    let sidenav_ctx = SidenavMode::new();

    view! {
        {if data_collapsible == SidenavCollapsible::None {
            view! {
                <aside
                    data-name="Sidenav"
                    class=tw_merge!(
                        "flex flex-col bg-sidenav text-sidenav-foreground w-(--sidenav-width)", class
                    )
                >
                    {children()}
                </aside>
            }
                .into_any()
        } else {
            view! {
                <aside
                    data-name="Sidenav"
                    data-sidenav=data_state.to_string()
                    data-state=move || if sidenav_ctx.is_open().get() { "Expanded" } else { "Collapsed" }
                    data-side=data_side.to_string()
                    class="md:block group peer text-sidenav-foreground data-[state=Collapsed]:hidden"
                >
                    <div
                        data-name="SidenavContainer"
                        class=tw_merge!(
                            "inset-y-0 z-10 h-svh w-(--sidenav-width) transition-[left,right,width] ease-linear",
                            class,
                            match data_side {
                                SidenavSide::Left => "group-data-[collapsible=Offcanvas]:left-[calc(var(--sidenav-width)*-1)]",
                                SidenavSide::Right => "group-data-[collapsible=Offcanvas]:right-[calc(var(--sidenav-width)*-1)]"
                            },
                            match variant {
                                 SidenavVariant::Sidenav => "group-data-[collapsible=Icon]:w-(--sidenav-width-icon) group-data-[side=Left]:border-r border-border group-data-[side=Right]:border-l",
                                SidenavVariant::Floating | SidenavVariant::Inset =>
                                    "p-2 group-data-[collapsible=Icon]:w-[calc(var(--sidenav-width-icon)+(--spacing(4))+2px)]",
                            },
                        )
                    >
                        // * Act as a Sidenav for the onclick trigger to work with nested Sidenavs.
                        <SidenavInner attr:data-sidenav="Sidenav" attr:data-variant=variant.to_string()>
                            {children()}
                        </SidenavInner>
                    </div>
                </aside>
            }
                .into_any()
        }}
    }
}

variants! {
    SidenavMenuButton {
        base: r#"
        peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2
        text-left text-sm outline-hidden ring-sidenav-ring transition-[width,height,padding]
        hover:bg-sidenav-accent hover:text-sidenav-accent-foreground focus-visible:ring-2
        active:bg-sidenav-accent active:text-sidenav-accent-foreground disabled:pointer-events-none
        disabled:opacity-50 group-has-data-[sidenav=menu-action]/menu-item:pr-8
        aria-disabled:pointer-events-none aria-disabled:opacity-50
        aria-[current=page]:bg-accent aria-[current=page]:font-medium
        aria-[current=page]:text-accent-foreground data-[state=open]:hover:bg-sidenav-accent
        data-[state=open]:hover:text-sidenav-accent-foreground [&>span:last-child]:truncate
        [&>svg]:size-4 [&>svg]:shrink-0 group-data-[collapsible=Icon]:size-8!
        group-data-[collapsible=Icon]:p-0! [&>svg]:stroke-2 aria-[current=page]:[&>svg]:stroke-[2.7]
        sidebar-primary
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

#[component]
pub fn SidenavLink(
    children: Children,
    #[prop(into)] href: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let merged_class = tw_merge!(
        r#"peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-md p-2 text-left
        outline-hidden ring-ring transition-[width,height,padding] focus-visible:ring-2
        active:bg-accent active:text-accent-foreground disabled:pointer-events-none
        disabled:opacity-50 group-has-data-[sidenav=menu-action]/menu-item:pr-8 aria-disabled:pointer-events-none
        aria-disabled:opacity-50 aria-[current=page]:bg-accent aria-[current=page]:font-semibold
        aria-[current=page]:text-accent-foreground data-[state=open]:hover:bg-accent
        data-[state=open]:hover:text-sidenav-accent-foreground group-data-[collapsible=Icon]:size-8!
        group-data-[collapsible=Icon]:p-2! [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0
        hover:bg-accent hover:text-accent-foreground h-8 text-sm
        [&_svg:not([class*='size-'])]:size-4"#,
        class
    );

    view! {
        <A
            attr:data-name="SidenavLink"
            attr:class=merged_class
            href=href
        >
            {children()}
        </A>
    }
}

#[component]
pub fn SidenavWrapper(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!(
        "group/sidenav-wrapper has-data-[variant=Inset]:bg-sidenav flex h-full w-full",
        class
    );

    view! {
        <div class=merged_class data-name="SidenavWrapper">
            {children()}
        </div>
    }
}
