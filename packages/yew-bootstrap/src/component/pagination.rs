use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum PaginationSize {
    Large,
    Normal,
    Small,
}

impl Default for PaginationSize {
    fn default() -> Self {
        PaginationSize::Normal
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub on_click_page: Callback<u32>,

    #[prop_or_default]
    pub current_page: u32,

    #[prop_or_default]
    pub size: PaginationSize,

    #[prop_or_default]
    pub total_pages: u32,
}

#[function_component(Pagination)]
pub fn pagination(props: &Props) -> Html {
    // main class(es)
    let mut classes = Classes::new();
    classes.push("pagination");
    match props.size {
        PaginationSize::Large => classes.push("pagination-lg"),
        PaginationSize::Small => classes.push("pagination-sm"),
        _ => (),
    }
    // page-item class(es)
    let mut page_item_classes = Classes::new();
    page_item_classes.push("page-item");

    // page-link class(es)
    let mut page_link_classes = Classes::new();
    page_link_classes.push("page-link");

    html! {
        <nav>
            <ul class={classes}>
            <li class={page_item_classes}>
              <a class={page_link_classes} href="#" aria-label="Previous">
                <span aria-hidden="true">&laquo;</span>
                <span class="sr-only">Previous</span>
              </a>
            </li>
            // child components
            {for page in 1..total_pages {
                if page == &props.current_page {
                    page_item_classes.push("active");
                }

                // callback
                let cb = props.on_click_page.clone();
                let page = *page;
                let onclick = Callback::from(move |e: MouseEvent| {
                    e.prevent_default();
                    cb.emit(page)
                });

                html! {
                    <li
                        class={page_item_classes}
                        onclick={onclick}>
                        <a class={page_link_classes} href="">{page + 1}</a>
                    </li>
                }
            }}
            <li class={page_item_classes}>
              <a class={page_link_classes} href="#" aria-label="Next">
                <span aria-hidden="true">&raquo;</span>
                <span class="sr-only">Next</span>
              </a>
            </li>
            </ul>
        </nav>
    }
}
