use icons::common::IconType;
use icons::icon_component::LeptosIcon;
use leptos::prelude::*;

#[component]
pub fn AppCard(
    #[prop(into)] app_title: String,
    #[prop(into)] platform_title: String,
    #[prop(into)] icon: IconType,
) -> impl IntoView {
    view! {
        <div class="flex gap-2 items-center">
            <div class="flex justify-center items-center rounded-lg bg-primary text-primary-foreground aspect-square size-8">
                <LeptosIcon icon={icon} />
            </div>
            <div class="grid flex-1 text-sm leading-tight text-left">
                <span class="font-medium">{app_title}</span>
                <span class="text-xs">{platform_title}</span>
            </div>
        </div>
    }
}
