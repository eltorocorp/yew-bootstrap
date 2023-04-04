use yew::{html, virtual_dom::VNode};

/// Links to the Bootstrap CSS CDN
pub fn include_cdn() -> VNode {
    html! {
        <link
            href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha3/dist/css/bootstrap.min.css"
            rel="stylesheet"
            integrity="sha384-KK94CHFLLe+nY2dmCWGMq91rCGa5gtU4mk92HdvYe+M/SXH301p5ILy+dN9+nJOZ"
            crossorigin="anonymous"
        />
    }
}

/// Links to the Bootstrap JS CDN, including the map file which must be explicitly mentioned for Trunk to copy it
pub fn include_cdn_js() -> VNode {
    html! {
        <>
            <link data-trunk={"true"} rel="copy-file" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha3/dist/js/bootstrap.bundle.min.js.map" />
            <script
                src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha3/dist/js/bootstrap.bundle.min.js"
                data-trunk={"true"}
                integrity="sha384-ENjdO4Dr2bkBIFxQpeoTz1HIcje39Wm4jDKdf19U8gI4ddQ3GYNS7NTKfAdVQSZe"
                crossorigin="anonymous"
            >
            </script>
        </>
    }
}

/// Inserts the bootstrap CSS directly into the content of the page
pub fn include_inline() -> VNode {
    html! {
        <style>
            {include_str!("bootstrap-5.3.0-alpha3.min.css")}
        </style>
    }
}

/// Include the Bootstrap Icons CDN
pub fn include_cdn_icons() -> VNode {
    html! {
        <link data-trunk={"true"} rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons/font/bootstrap-icons.min.css" />
    }
}
