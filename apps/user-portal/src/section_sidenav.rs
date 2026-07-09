use dita_core::common::components::section_sidenav::{
    Section, SectionData, SectionGroup, create_section_sidenav,
};
use icons::common::IconType;
use leptos::prelude::*;

#[component]
pub fn SectionSidenav() -> impl IntoView {
    let section_data: SectionData = SectionData {
        app_title: "User".to_string(),
        platform_title: "Dita Platform".to_string(),
        app_icon: IconType::User,
        section_groups: vec![
            SectionGroup {
                title: "User".to_string(),
                sections: vec![
                    Section {
                        title: "Calendar".to_string(),
                        path: "calendar".to_string(),
                        icon: IconType::CalendarCheck,
                    },
                    Section {
                        title: "Capabilities".to_string(),
                        path: "capabilities".to_string(),
                        icon: IconType::Key,
                    },
                    Section {
                        title: "Subjects".to_string(),
                        path: "subjects".to_string(),
                        icon: IconType::BookMarked,
                    },
                    Section {
                        title: "Resources".to_string(),
                        path: "resources".to_string(),
                        icon: IconType::BookOpenText,
                    },
                    Section {
                        title: "Courses".to_string(),
                        path: "courses".to_string(),
                        icon: IconType::Presentation,
                    },
                    Section {
                        title: "Qualifications".to_string(),
                        path: "qualifications".to_string(),
                        icon: IconType::Award,
                    },
                    Section {
                        title: "Timetable".to_string(),
                        path: "timetable".to_string(),
                        icon: IconType::Table2,
                    },
                ],
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
