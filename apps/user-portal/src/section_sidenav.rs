use dita_core::common::components::section_sidenav::{Section, SectionData, SectionGroup};
use icons::{
    Award, BookMarked, BookOpenText, CalendarCheck, Key, Presentation, Table2, User, Wallet,
};
use leptos::prelude::*;

pub fn get_section_data() -> SectionData {
    SectionData {
        app_title: "User".to_string(),
        platform_title: "Dita Platform".to_string(),
        app_icon: || view! {<User/>}.into_any(),
        section_groups: vec![
            SectionGroup {
                title: "User".to_string(),
                sections: vec![
                    Section {
                        title: "Calendar".to_string(),
                        path: "calendar".to_string(),
                        icon: || view! {<CalendarCheck/>}.into_any(),
                    },
                    Section {
                        title: "Capabilities".to_string(),
                        path: "capabilities".to_string(),
                        icon: || view! {<Key/>}.into_any(),
                    },
                    Section {
                        title: "Subjects".to_string(),
                        path: "subjects".to_string(),
                        icon: || view! {<BookMarked/>}.into_any(),
                    },
                    Section {
                        title: "Resources".to_string(),
                        path: "resources".to_string(),
                        icon: || view! {<BookOpenText/>}.into_any(),
                    },
                    Section {
                        title: "Courses".to_string(),
                        path: "courses".to_string(),
                        icon: || view! {<Presentation/>}.into_any(),
                    },
                    Section {
                        title: "Qualifications".to_string(),
                        path: "qualifications".to_string(),
                        icon: || view! {<Award/>}.into_any(),
                    },
                    Section {
                        title: "Timetable".to_string(),
                        path: "timetable".to_string(),
                        icon: || view! {<Table2/>}.into_any(),
                    },
                ],
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
