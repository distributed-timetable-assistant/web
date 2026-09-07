use dita_core::common::components::section_sidenav::{Section, SectionData, SectionGroup};
use icons::{
    BookOpen, BookOpenText, Building2, CalendarCheck, DoorClosed, GraduationCap, Package,
    Presentation, Table2, UserRound, Users, Wallet,
};
use leptos::prelude::*;

pub fn get_section_data() -> SectionData {
    SectionData {
        app_title: "Institution".to_string(),
        platform_title: "Dita Platform".to_string(),
        app_icon: || view! {<Building2/>}.into_any(),
        section_groups: vec![
            SectionGroup {
                title: "Institution".to_string(),
                sections: vec![
                    Section {
                        title: "Calendar".to_string(),
                        path: "calendar".to_string(),
                        icon: || view! {<CalendarCheck/>}.into_any(),
                    },
                    Section {
                        title: "Units".to_string(),
                        path: "units".to_string(),
                        icon: || view! {<Building2/>}.into_any(),
                    },
                    Section {
                        title: "Facilities".to_string(),
                        path: "facilities".to_string(),
                        icon: || view! {<DoorClosed/>}.into_any(),
                    },
                    Section {
                        title: "Groups".to_string(),
                        path: "groups".to_string(),
                        icon: || view! {<Users/>}.into_any(),
                    },
                    Section {
                        title: "Learners".to_string(),
                        path: "learners".to_string(),
                        icon: || view! {<GraduationCap/>}.into_any(),
                    },
                    Section {
                        title: "Subjects".to_string(),
                        path: "subjects".to_string(),
                        icon: || view! {<BookOpen/>}.into_any(),
                    },
                    Section {
                        title: "Resources".to_string(),
                        path: "resources".to_string(),
                        icon: || view! {<BookOpenText/>}.into_any(),
                    },
                    Section {
                        title: "Actors".to_string(),
                        path: "actors".to_string(),
                        icon: || view! {<UserRound/>}.into_any(),
                    },
                    Section {
                        title: "Courses".to_string(),
                        path: "courses".to_string(),
                        icon: || view! {<Presentation/>}.into_any(),
                    },
                    Section {
                        title: "Packets".to_string(),
                        path: "packets".to_string(),
                        icon: || view! {<Package/>}.into_any(),
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
