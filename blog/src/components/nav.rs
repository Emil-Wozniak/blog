use dioxus::{
    prelude::*,
    router::{Link, Route, Router},
};

pub fn navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        section { class: "w-full top-0 overflow-hidden mb-6 py-6 px-6 border-r bg-sky-500 hover:bg-sky-700",
            nav { class: "block h-full ",
                div { class: "float-left text-3xl font-bold font-heading",
                    a { class: "m-8", Link { to: "/", "Home"} }
                    a { class: "m-8", Link { to: "/blog", "Blog"} }
                    a { class: "m-8", Link { to: "/cv", "CV"} }
                    a { class: "m-8", Link { to: "/secret", "Secret"} }
                }
            }
        }
    })
}

mod icons {
    use super::*;

    pub(super) fn icon_0(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "mr-3",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 23 23",
                width: "23",
                height: "23",
                path {
                    stroke_linejoin: "round",
                    d: "M18.1159 8.72461H2.50427C1.99709 8.72461 1.58594 9.12704 1.58594 9.62346V21.3085C1.58594 21.8049 1.99709 22.2074 2.50427 22.2074H18.1159C18.6231 22.2074 19.0342 21.8049 19.0342 21.3085V9.62346C19.0342 9.12704 18.6231 8.72461 18.1159 8.72461Z",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                }
                path {
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    d: "M6.34473 6.34469V4.95676C6.34473 3.85246 6.76252 2.79338 7.5062 2.01252C8.24988 1.23165 9.25852 0.792969 10.3102 0.792969C11.362 0.792969 12.3706 1.23165 13.1143 2.01252C13.858 2.79338 14.2758 3.85246 14.2758 4.95676V6.34469",
                    stroke_width: "1.5",
                    stroke_linejoin: "round",
                }
            }
		))
    }

    pub(super) fn icon_1(cx: Scope) -> Element {
        cx.render(rsx!(
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                height: "20",
                view_box: "0 0 23 20",
                width: "23",
                fill: "none",
                path {
                    d: "M11.4998 19.2061L2.70115 9.92527C1.92859 9.14433 1.41864 8.1374 1.24355 7.04712C1.06847 5.95684 1.23713 4.8385 1.72563 3.85053V3.85053C2.09464 3.10462 2.63366 2.45803 3.29828 1.96406C3.9629 1.47008 4.73408 1.14284 5.5483 1.00931C6.36252 0.875782 7.19647 0.939779 7.98144 1.19603C8.7664 1.45228 9.47991 1.89345 10.0632 2.48319L11.4998 3.93577L12.9364 2.48319C13.5197 1.89345 14.2332 1.45228 15.0182 1.19603C15.8031 0.939779 16.6371 0.875782 17.4513 1.00931C18.2655 1.14284 19.0367 1.47008 19.7013 1.96406C20.3659 2.45803 20.905 3.10462 21.274 3.85053V3.85053C21.7625 4.8385 21.9311 5.95684 21.756 7.04712C21.581 8.1374 21.071 9.14433 20.2984 9.92527L11.4998 19.2061Z",
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    stroke_linejoin: "round",
                    stroke_linecap: "round",
                }
            }
		))
    }

    pub(super) fn icon_2(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "mr-3",
                fill: "none",
                height: "31",
                xmlns: "http://www.w3.org/2000/svg",
                width: "32",
                view_box: "0 0 32 31",
                path {
                    stroke_linejoin: "round",
                    stroke_width: "1.5",
                    d: "M16.0006 16.3154C19.1303 16.3154 21.6673 13.799 21.6673 10.6948C21.6673 7.59064 19.1303 5.07422 16.0006 5.07422C12.871 5.07422 10.334 7.59064 10.334 10.6948C10.334 13.799 12.871 16.3154 16.0006 16.3154Z",
                    stroke_linecap: "round",
                    stroke: "currentColor",
                }
                path {
                    stroke_width: "1.5",
                    d: "M24.4225 23.8963C23.6678 22.3507 22.4756 21.0445 20.9845 20.1298C19.4934 19.2151 17.7647 18.7295 15.9998 18.7295C14.2349 18.7295 12.5063 19.2151 11.0152 20.1298C9.52406 21.0445 8.33179 22.3507 7.57715 23.8963",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
		))
    }

    pub(super) fn icon_3(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "h-2 w-2 text-gray-500 cursor-pointer",
                height: "10",
                width: "10",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 10 10",
                path {
                    stroke_width: "1.5",
                    stroke_linejoin: "round",
                    d: "M9.00002 1L1 9.00002M1.00003 1L9.00005 9.00002",
                    stroke: "black",
                    stroke_linecap: "round",
                }
            }
		))
    }

    pub(super) fn icon_4(cx: Scope) -> Element {
        cx.render(rsx!(
            svg {
                view_box: "0 0 20 12",
                fill: "none",
                width: "20",
                xmlns: "http://www.w3.org/2000/svg",
                height: "12",
                path {
                    d: "M1 2H19C19.2652 2 19.5196 1.89464 19.7071 1.70711C19.8946 1.51957 20 1.26522 20 1C20 0.734784 19.8946 0.48043 19.7071 0.292893C19.5196 0.105357 19.2652 0 19 0H1C0.734784 0 0.48043 0.105357 0.292893 0.292893C0.105357 0.48043 0 0.734784 0 1C0 1.26522 0.105357 1.51957 0.292893 1.70711C0.48043 1.89464 0.734784 2 1 2ZM19 10H1C0.734784 10 0.48043 10.1054 0.292893 10.2929C0.105357 10.4804 0 10.7348 0 11C0 11.2652 0.105357 11.5196 0.292893 11.7071C0.48043 11.8946 0.734784 12 1 12H19C19.2652 12 19.5196 11.8946 19.7071 11.7071C19.8946 11.5196 20 11.2652 20 11C20 10.7348 19.8946 10.4804 19.7071 10.2929C19.5196 10.1054 19.2652 10 19 10ZM19 5H1C0.734784 5 0.48043 5.10536 0.292893 5.29289C0.105357 5.48043 0 5.73478 0 6C0 6.26522 0.105357 6.51957 0.292893 6.70711C0.48043 6.89464 0.734784 7 1 7H19C19.2652 7 19.5196 6.89464 19.7071 6.70711C19.8946 6.51957 20 6.26522 20 6C20 5.73478 19.8946 5.48043 19.7071 5.29289C19.5196 5.10536 19.2652 5 19 5Z",
                    fill: "#8594A5",
                }
            }
		))
    }

    pub(super) fn icon_5(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "mr-2",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                width: "23",
                height: "23",
                view_box: "0 0 23 23",
                path {
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M18.1159 8.72461H2.50427C1.99709 8.72461 1.58594 9.12704 1.58594 9.62346V21.3085C1.58594 21.8049 1.99709 22.2074 2.50427 22.2074H18.1159C18.6231 22.2074 19.0342 21.8049 19.0342 21.3085V9.62346C19.0342 9.12704 18.6231 8.72461 18.1159 8.72461Z",
                    stroke: "currentColor",
                }
                path {
                    d: "M6.34473 6.34469V4.95676C6.34473 3.85246 6.76252 2.79338 7.5062 2.01252C8.24988 1.23165 9.25852 0.792969 10.3102 0.792969C11.362 0.792969 12.3706 1.23165 13.1143 2.01252C13.858 2.79338 14.2758 3.85246 14.2758 4.95676V6.34469",
                    stroke_linejoin: "round",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke: "currentColor",
                }
            }
		))
    }

    pub(super) fn icon_6(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "mr-3",
                height: "31",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 32 31",
                width: "32",
                fill: "none",
                path {
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    d: "M16.0006 16.3154C19.1303 16.3154 21.6673 13.799 21.6673 10.6948C21.6673 7.59064 19.1303 5.07422 16.0006 5.07422C12.871 5.07422 10.334 7.59064 10.334 10.6948C10.334 13.799 12.871 16.3154 16.0006 16.3154Z",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
                path {
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                    stroke: "currentColor",
                    stroke_linejoin: "round",
                    d: "M24.4225 23.8963C23.6678 22.3507 22.4756 21.0445 20.9845 20.1298C19.4934 19.2151 17.7647 18.7295 15.9998 18.7295C14.2349 18.7295 12.5063 19.2151 11.0152 20.1298C9.52406 21.0445 8.33179 22.3507 7.57715 23.8963",
                }
            }
		))
    }

    pub(super) fn icon_7(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "mr-3",
                view_box: "0 0 23 23",
                fill: "none",
                height: "23",
                width: "23",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    stroke_linecap: "round",
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    stroke_linejoin: "round",
                    d: "M18.1159 8.72461H2.50427C1.99709 8.72461 1.58594 9.12704 1.58594 9.62346V21.3085C1.58594 21.8049 1.99709 22.2074 2.50427 22.2074H18.1159C18.6231 22.2074 19.0342 21.8049 19.0342 21.3085V9.62346C19.0342 9.12704 18.6231 8.72461 18.1159 8.72461Z",
                }
                path {
                    d: "M6.34473 6.34469V4.95676C6.34473 3.85246 6.76252 2.79338 7.5062 2.01252C8.24988 1.23165 9.25852 0.792969 10.3102 0.792969C11.362 0.792969 12.3706 1.23165 13.1143 2.01252C13.858 2.79338 14.2758 3.85246 14.2758 4.95676V6.34469",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke: "currentColor",
                    stroke_linejoin: "round",
                }
            }
		))
    }

    pub(super) fn icon_8(cx: Scope) -> Element {
        cx.render(rsx!(
            svg {
                height: "20",
                width: "23",
                fill: "none",
                view_box: "0 0 23 20",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M11.4998 19.2061L2.70115 9.92527C1.92859 9.14433 1.41864 8.1374 1.24355 7.04712C1.06847 5.95684 1.23713 4.8385 1.72563 3.85053V3.85053C2.09464 3.10462 2.63366 2.45803 3.29828 1.96406C3.9629 1.47008 4.73408 1.14284 5.5483 1.00931C6.36252 0.875782 7.19647 0.939779 7.98144 1.19603C8.7664 1.45228 9.47991 1.89345 10.0632 2.48319L11.4998 3.93577L12.9364 2.48319C13.5197 1.89345 14.2332 1.45228 15.0182 1.19603C15.8031 0.939779 16.6371 0.875782 17.4513 1.00931C18.2655 1.14284 19.0367 1.47008 19.7013 1.96406C20.3659 2.45803 20.905 3.10462 21.274 3.85053V3.85053C21.7625 4.8385 21.9311 5.95684 21.756 7.04712C21.581 8.1374 21.071 9.14433 20.2984 9.92527L11.4998 19.2061Z",
                    stroke_linejoin: "round",
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                }
            }
		))
    }

    pub(super) fn icon_9(cx: Scope) -> Element {
        cx.render(rsx!(
            svg {
                view_box: "0 0 18 18",
                xmlns: "http://www.w3.org/2000/svg",
                width: "18",
                height: "18",
                fill: "none",
                path {
                    fill: "black",
                    d: "M18 15.4688H0V17.7207H18V15.4688Z",
                }
                path {
                    fill: "black",
                    d: "M11.0226 7.87402H0V10.126H11.0226V7.87402Z",
                }
                path {
                    fill: "black",
                    d: "M18 0.279297H0V2.53127H18V0.279297Z",
                }
            }
		))
    }
}