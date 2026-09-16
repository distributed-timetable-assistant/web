use dita_auth::{AccountState, AuthState};
use dita_design_system::components::core::avatar::{Avatar, AvatarFallback, AvatarImage};
use dita_design_system::components::core::button::{Button, ButtonSize, ButtonVariant};
use dita_design_system::components::core::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuActionVariant, DropdownMenuAlign,
    DropdownMenuContent, DropdownMenuSeparator, DropdownMenuTrigger,
};
use icons::{LogOut, Settings};
use leptos::prelude::*;

#[component]
pub fn AccountMenu() -> impl IntoView {
    let account_state = AccountState::new();
    let auth_state = AuthState::new();
    let user_resource = account_state.user();

    view! {
        <DropdownMenu align=DropdownMenuAlign::End>
            <DropdownMenuTrigger class="p-0 bg-transparent border-0 rounded-full">
                <Button variant=ButtonVariant::Ghost size=ButtonSize::Icon class="rounded-full p-0">
                    <Avatar>
                        <Suspense fallback=move || view! {
                            <AvatarFallback>"U"</AvatarFallback>
                        }>
                            {move || {
                                user_resource.get().flatten().map(|user| {
                                    let initials = user.initials();
                                    let avatar_url = user.avatar_url.clone();
                                    view! {
                                        {avatar_url.map(|url| view! {
                                            <AvatarImage attr:src=url attr:alt="Avatar" />
                                        })}
                                        <AvatarFallback>{initials}</AvatarFallback>
                                    }
                                })
                            }}
                        </Suspense>
                    </Avatar>
                </Button>
            </DropdownMenuTrigger>

            <DropdownMenuContent class="w-64 p-2">
                <div class="flex flex-col gap-0.5 px-2 py-1.5 text-left">
                    <Suspense fallback=move || view! {
                        <div class="text-sm font-medium text-foreground">"Loading..."</div>
                    }>
                        {move || {
                            user_resource.get().flatten().map(|user| {
                                let display_name = user.display_name();
                                let username = user.username.clone();
                                let email = user.email.clone();

                                view! {
                                    <div class="text-sm font-semibold text-foreground truncate">
                                        {display_name}
                                    </div>
                                    {username.map(|uname| view! {
                                        <div class="text-xs text-muted-foreground truncate">
                                            {format!("@{uname}")}
                                        </div>
                                    })}
                                    <div class="text-xs text-muted-foreground truncate mt-0.5">
                                        {email}
                                    </div>
                                }
                            })
                        }}
                    </Suspense>
                </div>

                <DropdownMenuSeparator class="my-1" />

                <DropdownMenuAction
                    href="https://identity.outi.ir/settings"
                    class="px-2 py-1.5 rounded-sm cursor-pointer"
                >
                    <Settings class="size-4" />
                    <span>"Account Settings"</span>
                </DropdownMenuAction>

                <DropdownMenuSeparator class="my-1" />

                <DropdownMenuAction
                    variant=DropdownMenuActionVariant::Destructive
                    class="px-2 py-1.5 rounded-sm cursor-pointer"
                    on:click=move |_| auth_state.logout()
                >
                    <LogOut class="size-4" />
                    <span>"Log out"</span>
                </DropdownMenuAction>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
