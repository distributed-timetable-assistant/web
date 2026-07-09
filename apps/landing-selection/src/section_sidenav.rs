use dita_core::common::components::section_sidenav::{
    Section, SectionData, SectionGroup, create_section_sidenav,
};
use icons::common::IconType;
use leptos::prelude::*;

#[component]
pub fn SectionSidenav() -> impl IntoView {
    let section_data: SectionData = SectionData {
        app_title: "Landing Selection".to_string(),
        platform_title: "Dita Platform".to_string(),
        app_icon: IconType::LandPlot,
        section_groups: vec![
            SectionGroup {
                title: "User".to_string(),
                sections: vec![Section {
                    title: "Apps".to_string(),
                    path: "apps".to_string(),
                    icon: IconType::LayoutGrid,
                }],
            },
            SectionGroup {
                title: "Account".to_string(),
                sections: vec![Section {
                    title: "Wallet".to_string(),
                    path: "wallet".to_string(),
                    icon: IconType::Wallet,
                }],
            },
        ],
    };
    create_section_sidenav(section_data)
}
