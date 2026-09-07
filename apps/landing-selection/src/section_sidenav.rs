use dita_core::common::components::section_sidenav::{Section, SectionData, SectionGroup};
use icons::{LandPlot, LayoutGrid, Wallet};
use leptos::prelude::IntoAny;
use leptos::view;

pub fn get_section_data() -> SectionData {
    SectionData {
        app_title: "Landing Selection".to_string(),
        platform_title: "Dita Platform".to_string(),
        app_icon: || view! {<LandPlot/>}.into_any(),
        section_groups: vec![
            SectionGroup {
                title: "User".to_string(),
                sections: vec![Section {
                    title: "Apps".to_string(),
                    path: "apps".to_string(),
                    icon: || view! {<LayoutGrid/>}.into_any(),
                }],
            },
            SectionGroup {
                title: "Account".to_string(),
                sections: vec![Section {
                    title: "Wallet".to_string(),
                    path: "wallet".to_string(),
                    icon: || view! {<Wallet/>}.into_any(),
                }],
            },
        ],
    }
}
