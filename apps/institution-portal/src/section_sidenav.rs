use dita_core::common::components::section_sidenav::{
    Section, SectionData, SectionGroup, create_section_sidenav,
};
use icons::common::IconType;
use leptos::prelude::*;

#[component]
pub fn SectionSidenav() -> impl IntoView {
    let section_data: SectionData = SectionData {
        app_title: "Institution".to_string(),
        platform_title: "Dita Platform".to_string(),
        app_icon: IconType::Building2,
        section_groups: vec![
            SectionGroup {
                title: "Institution".to_string(),
                sections: vec![
                    Section {
                        title: "Calendar".to_string(),
                        path: "calendar".to_string(),
                        icon: IconType::CalendarCheck,
                    },
                    Section {
                        title: "Units".to_string(),
                        path: "units".to_string(),
                        icon: IconType::Building2,
                    },
                    Section {
                        title: "Facilities".to_string(),
                        path: "facilities".to_string(),
                        icon: IconType::DoorClosed,
                    },
                    Section {
                        title: "Groups".to_string(),
                        path: "groups".to_string(),
                        icon: IconType::Users,
                    },
                    Section {
                        title: "Learners".to_string(),
                        path: "learners".to_string(),
                        icon: IconType::GraduationCap,
                    },
                    Section {
                        title: "Subjects".to_string(),
                        path: "subjects".to_string(),
                        icon: IconType::BookOpen,
                    },
                    Section {
                        title: "Resources".to_string(),
                        path: "resources".to_string(),
                        icon: IconType::BookOpenText,
                    },
                    Section {
                        title: "Actors".to_string(),
                        path: "actors".to_string(),
                        icon: IconType::UserRound,
                    },
                    Section {
                        title: "Courses".to_string(),
                        path: "courses".to_string(),
                        icon: IconType::Presentation,
                    },
                    Section {
                        title: "Packets".to_string(),
                        path: "packets".to_string(),
                        icon: IconType::Package,
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
