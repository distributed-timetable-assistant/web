use leptos::prelude::*;
use leptos_ui::tw_merge::tw_merge;

#[component]
pub fn SvgDitaLogo(
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] data_name: Option<String>,
) -> impl IntoView {
    view! {
        <svg
            class=tw_merge!("", class)
            data-name=data_name
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 512 512"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
        >
            <mask id="cut">
                <rect width="512" height="512" fill="white"/>
                <line
                        stroke="black"
                        stroke-width="40"
                        x1="96" y1="364" x2="160" y2="364"
                />
            </mask>
            <defs>
                <linearGradient
                    id="rainbow"
                    gradientUnits="userSpaceOnUse"
                    x1="512" y1="0"
                    x2="0" y2="512">

                    <stop offset="0%" stop-color="#E53935"/>
                    <stop offset="20%" stop-color="#F57C00"/>
                    <stop offset="40%" stop-color="#F9A825"/>
                    <stop offset="60%" stop-color="#00897B"/>
                    <stop offset="80%" stop-color="#1565C0"/>
                    <stop offset="100%" stop-color="#6A1B9A"/>
                </linearGradient>
            </defs>
            <path
                    mask="url(#cut)"
                    fill="url(#rainbow)"
                    fill-rule="evenodd"
                    d="
                M96 64
                H224
                C360 64 448 152 448 256
                C448 360 360 448 224 448
                H96
                Z

                M160 128
                V384
                H224
                C320 384 384 330 384 256
                C384 182 320 128 224 128
                Z"/>
            <g
                    fill="none"
                    stroke="url(#rainbow)"
                    stroke-width="20"
                    stroke-linecap="round">

                <line x1="220" y1="180" x2="220" y2="332"/>
                <line x1="292" y1="180" x2="292" y2="332"/>

                <line x1="160" y1="214" x2="330" y2="214"/>
                <line x1="160" y1="298" x2="330" y2="298"/>

            </g>
        </svg>
    }
}
