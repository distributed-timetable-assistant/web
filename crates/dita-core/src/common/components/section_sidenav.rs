use crate::common::components::app_card::AppCard;
use dita_design_system::components::sidenav::layout::*;
use icons::common::IconType;
use icons::icon_component::LeptosIcon;
use leptos::prelude::*;

#[derive(Debug)]
pub struct SectionData {
    pub app_title: String,
    pub platform_title: String,
    pub app_icon: IconType,
    pub section_groups: Vec<SectionGroup>,
}
#[derive(Debug)]
pub struct SectionGroup {
    pub title: String,
    pub sections: Vec<Section>,
}
#[derive(Debug)]
pub struct Section {
    pub title: String,
    pub path: String,
    pub icon: IconType,
}

pub fn create_section_sidenav(section_data: SectionData) -> impl IntoView {
    view! {
        <Sidenav data_collapsible=SidenavCollapsible::Icon class="flex-1 md:flex">
            <SidenavHeader attr:data-sidenav="header" class="p-3">
                <AppCard app_title={section_data.app_title} platform_title={section_data.platform_title} icon={section_data.app_icon} />
            </SidenavHeader>
            <SidenavContent attr:data-sidenav="content">
                {create_section_groups(section_data.section_groups)}
            </SidenavContent>
        </Sidenav>
    }
}

fn create_sections(sections: Vec<Section>) -> impl IntoView {
    sections
        .into_iter()
        .map(|section| {
            view! {
                <SidenavMenuItem>
                    <SidenavLink href={section.path}>
                        <LeptosIcon icon={section.icon} />
                        <h4 class="pt-1 text-xs font-medium">{section.title}</h4>
                    </SidenavLink>
                </SidenavMenuItem>
            }
        })
        .collect_view()
}

fn create_section_groups(section_groups: Vec<SectionGroup>) -> impl IntoView {
    section_groups
        .into_iter()
        .map(|section_group| {
            view! {
                <SidenavGroup attr:data-sidenav="group">
                    <SidenavGroupContent attr:data-sidenav="group-content">
                        <SidenavGroupLabel>{section_group.title}</SidenavGroupLabel>
                        <SidenavMenu>
                            {create_sections(section_group.sections)}
                        </SidenavMenu>
                    </SidenavGroupContent>
                </SidenavGroup>
            }
        })
        .collect_view()
}
