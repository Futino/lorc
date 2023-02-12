use yew::{function_component, html, Html};

mod props;
use props::Props;

#[derive(PartialEq)]
pub enum IconType {
    LocationIcon,
    LinkedinIcon,
    ExternalLinkIcon,
    ClockIcon,
    MenuIcon,
    LeftArrow,
    RightArrow,
    SeperatorIcon,
    SquareIcon,
    PhoneIcon,
}

#[function_component]
pub fn Icon(props: &Props) -> Html {
    let Props {
        class,
        icon,
        height,
        width,
        color,
    } = props;
    match icon {
        IconType::LocationIcon => {
            html! {
                <div class={class}>
                    <svg width={width.to_string()} height={height.to_string()} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none" stroke={color.to_string()}>
                        <g id="SVGRepo_bgCarrier" stroke-width="0"/>
                        <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"/>
                        <g id="SVGRepo_iconCarrier">
                            <path stroke="currentColor" stroke-linejoin="round" stroke-width="2" d="M13 9a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"/>
                            <path stroke="currentColor" stroke-linejoin="round" stroke-width="2" d="M17.5 9.5c0 3.038-2 6.5-5.5 10.5-3.5-4-5.5-7.462-5.5-10.5a5.5 5.5 0 1 1 11 0Z" />
                        </g>
                    </svg>
                </div>
            }
        }
        IconType::LinkedinIcon => {
            html! {
                <div class={class}>
                    <svg width={width.to_string()} height={height.to_string()} viewBox="0 0 24 24">
                        <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z"/>
                    </svg>
                </div>
            }
        }
        IconType::ExternalLinkIcon => {
            html! {
                <div class={class}>
                    <svg class="ml-2 w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                        <path d="M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z"></path>
                        <path d="M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z"></path>
                    </svg>
                </div>
            }
        }
        IconType::ClockIcon => {
            html! {
            <div class={class}>
                <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <path d="M12 6V12L16 14M22 12C22 17.5228 17.5228 22 12 22C6.47715 22 2 17.5228 2 12C2 6.47715 6.47715 2 12 2C17.5228 2 22 6.47715 22 12Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path> </g></svg>
            </div>}
        }
        IconType::MenuIcon => {
            html! {
            <div class={class}>
                <svg width="30px" height="30px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"> <g id="style=linear"> <g id="menu-hamburger"> <path id="vector" d="M3 6H21" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"></path> <path id="vector_2" d="M3 12H21" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"></path> <path id="vector_3" d="M3 18H21" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"></path> </g> </g> </g></svg>
            </div>}
        }
        IconType::SquareIcon => {
            html! {
                <div class={class}>
                    <svg fill="currentColor" width="12px" height="12px" viewBox="0 0 32 32" version="1.1" xmlns="http://www.w3.org/2000/svg" transform="rotate(45)">
                    <g id="SVGRepo_bgCarrier" stroke-width="0" />
                    <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round" />
                    <g id="SVGRepo_iconCarrier">
                        <path d="M1.25 1.25v29.5h29.5v-29.5zM29.25 29.25h-26.5v-26.5h26.5z" />
                    </g>
                    </svg>
                </div>
            }
        }
        IconType::SeperatorIcon => {
            html! {
                    <div class={class}>
                        <svg width="110" height="70" x="0px" y="0px" viewBox="0 0 97.1 12.7" enable-background="new 0 0 97.1 12.7" space="preserve">
                            <rect x="31.6" y="3" transform="matrix(0.7071 0.7071 -0.7071 0.7071 14.7528 -22.8645)" fill="none" stroke="currentColor" stroke-width="0.75" stroke-miterlimit="10" width="6.8" height="6.8"/>
                            <rect x="58.7" y="3" transform="matrix(0.7071 0.7071 -0.7071 0.7071 22.699 -42.0484)" fill="none" stroke="currentColor" stroke-width="0.75" stroke-miterlimit="10" width="6.8" height="6.8"/>
                            <polygon fill="none" stroke="currentColor" stroke-width="0.75" stroke-miterlimit="10" points="48.6,12.1 39.8,6.4 48.6,0.6 57.3,6.4
                                "/>
                            <line fill="none" stroke="currentColor" stroke-width="0.75" stroke-miterlimit="10" x1="3.2" y1="6.4" x2="30.2" y2="6.4"/>
                            <circle fill="currentColor" cx="2.6" cy="6.4" r="2.5"/>
                            <line fill="none" stroke="currentColor" stroke-width="0.75" stroke-miterlimit="10" x1="93.9" y1="6.4" x2="66.9" y2="6.4"/>
                            <circle fill="currentColor" cx="94.4" cy="6.4" r="2.5"/>
                        </svg>
                    </div>
            }
        }
        IconType::RightArrow => {
            html! {
                <div class={class}>
                    <svg aria-hidden="true" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7">
                    </path>
                                </svg>
                </div>
            }
        }
        IconType::LeftArrow => {
            html! {
                <div class={class}>
                    <svg aria-hidden="true" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7">
                        </path>
                    </svg>
                </div>
            }
        }
        IconType::PhoneIcon => {
            html! {
                <div class={class}>
                <svg width={width.to_string()} height={height.to_string()} version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 202.592 202.592" fill="#000000">
                    <g id="SVGRepo_bgCarrier" stroke-width="0"/>
                    <g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"/>
                    <g id="SVGRepo_iconCarrier">
                        <g>
                            <g>
                                <path style="fill:#000000;" d="M198.048,160.105l-31.286-31.29c-6.231-6.206-16.552-6.016-23.001,0.433l-15.761,15.761 c-0.995-0.551-2.026-1.124-3.11-1.732c-9.953-5.515-23.577-13.074-37.914-27.421C72.599,101.48,65.03,87.834,59.5,77.874 c-0.587-1.056-1.145-2.072-1.696-3.038l10.579-10.565l5.2-5.207c6.46-6.46,6.639-16.778,0.419-23.001L42.715,4.769 c-6.216-6.216-16.541-6.027-23.001,0.433l-8.818,8.868l0.243,0.24c-2.956,3.772-5.429,8.124-7.265,12.816 c-1.696,4.466-2.752,8.729-3.235,12.998c-4.13,34.25,11.52,65.55,53.994,108.028c58.711,58.707,106.027,54.273,108.067,54.055 c4.449-0.53,8.707-1.593,13.038-3.275c4.652-1.818,9.001-4.284,12.769-7.233l0.193,0.168l8.933-8.747 C204.079,176.661,204.265,166.343,198.048,160.105z M190.683,176.164l-3.937,3.93l-1.568,1.507 c-2.469,2.387-6.743,5.74-12.984,8.181c-3.543,1.364-7.036,2.24-10.59,2.663c-0.447,0.043-44.95,3.84-100.029-51.235 C14.743,94.38,7.238,67.395,10.384,41.259c0.394-3.464,1.263-6.95,2.652-10.593c2.462-6.277,5.812-10.547,8.181-13.02l5.443-5.497 c2.623-2.63,6.714-2.831,9.112-0.433l31.286,31.286c2.394,2.401,2.205,6.492-0.422,9.13L45.507,73.24l1.95,3.282 c1.084,1.829,2.23,3.879,3.454,6.106c5.812,10.482,13.764,24.83,29.121,40.173c15.317,15.325,29.644,23.27,40.094,29.067 c2.258,1.249,4.32,2.398,6.17,3.5l3.289,1.95l21.115-21.122c2.634-2.623,6.739-2.817,9.137-0.426l31.272,31.279 C193.5,169.446,193.31,173.537,190.683,176.164z"/>
                            </g>
                        </g>
                    </g>
                </svg>
                </div>
            }
        }
    }
}
