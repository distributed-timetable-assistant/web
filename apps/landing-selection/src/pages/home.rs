use dita_design_system::components::core::button::{Button, ButtonVariant};
use dita_design_system::components::core::card::components::{CardGroup, IconWrapper};
use dita_design_system::hooks::header::HeaderMode;
use icons::common::IconType;
use icons::icon_component::LeptosIcon;
use leptos::prelude::*;

#[derive(Clone)]
struct Portal {
    title: &'static str,
    description: &'static str,
    icons: [IconType; 2],
    href: &'static str,
    disabled: bool,
}

#[component]
pub fn HomePage() -> impl IntoView {
    let header_ctx = HeaderMode::new();
    header_ctx.set_page_title("Apps".into());

    let portals = vec![
        Portal {
            title: "User Portal",
            description: "The central hub for our human ecosystem",
            icons: [IconType::User, IconType::GraduationCap],
            href: "/user",
            disabled: false,
        },
        Portal {
            title: "Institution Portal",
            description: "A powerful, centralized environment engineered to manage infrastructure, scale programs, and optimize resources",
            icons: [IconType::University, IconType::Building2],
            href: "/institution",
            disabled: false,
        },
        Portal {
            title: "Service Portal",
            description: "Connects and orchestrates our distributed microservices",
            icons: [IconType::Layers3, IconType::Database],
            href: "/service",
            disabled: true,
        },
        Portal {
            title: "Admin Portal",
            description: "The ultimate command center",
            icons: [IconType::UserCog, IconType::ChartSpline],
            href: "/admin",
            disabled: true,
        },
    ];

    view! {
        <div class="container mx-auto px-4 py-12 w-full">

            <div class="text-center mb-12">
                <h1 class="text-6xl font-bold mb-4">"Welcome to DiTA"</h1>
                <p class="text-xl text-muted-foreground">"Select your landing to continue"</p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <For
                    each=move || portals.clone()
                    key=|portal| portal.title
                    children=move |portal| {
                        view! {
                            <CardGroup class="shadow-lg max-w-lg @lg:max-w-2xl">

                                <div class="flex justify-center isolate">
                                    <IconWrapper class="relative top-1.5 left-2.8 -rotate-6 group-hover:-rotate-12 group-hover:-translate-x-5">
                                        <LeptosIcon icon={portal.icons[0]} class="size-4" />
                                    </IconWrapper>
                                    <IconWrapper class="relative top-1.5 right-2.8 rotate-6 group-hover:rotate-12 group-hover:translate-x-5">
                                        <LeptosIcon icon={portal.icons[1]} class="size-4" />
                                    </IconWrapper>
                                </div>

                                <h2 class="mt-6 text-base font-medium">{portal.title.to_string()}</h2>
                                <p class="mx-auto mt-1 text-sm text-muted-foreground max-w-[300px]">
                                    {portal.description.to_string()}
                                </p>

                                <Button
                                    variant=ButtonVariant::Outline
                                    class="mt-4"
                                    href={portal.href.to_string()}
                                    attr:rel="external"
                                    attr:aria-disabled={portal.disabled.to_string()}
                                >
                                    "Enter Portal"
                                </Button>
                            </CardGroup>
                        }
                    }
                />
            </div>
        </div>
    }
}
