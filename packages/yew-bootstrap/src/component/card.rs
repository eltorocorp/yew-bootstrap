use yew::prelude::*;

use crate::util::Color;

pub struct Card {}

#[derive(Properties, Clone, PartialEq)]
pub struct CardProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub text_bg_color: Option<Color>,

    #[prop_or_default]
    pub border_color: Option<Color>,

    /// Children for the group (Header, Footer, and Body instances)
    #[prop_or_default]
    pub children: Children,

    /// Aria label used for assistive technologies
    #[prop_or_default]
    pub label: String,

    /// Role, used for assistive technoligies to describe the purpose of the group.
    #[prop_or_default]
    pub role: String,
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = classes!("card");

        classes.push(&props.class);

        if let Some(text_bg_color) = &props.text_bg_color {
            classes.push(format!("text-bg-{}", text_bg_color));
        }

        if let Some(border_color) = &props.border_color {
            classes.push(format!("border-{}", border_color));
        }

        html! {
            <div
                class={classes}
                role={props.role.clone()}
                aria-label={props.label.clone()}
            >
                { for props.children.iter() }
            </div>
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct CardBody {}

#[derive(Properties, Clone, PartialEq)]
pub struct CardBodyProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    /// Aria label used for assistive technologies
    #[prop_or_default]
    pub label: String,

    /// Role, used for assistive technoligies to describe the purpose of the group.
    #[prop_or_default]
    pub role: String,
}

impl Component for CardBody {
    type Message = ();
    type Properties = CardBodyProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let classes = classes!("card-body", &props.class);
        html! {
            <div
                class={classes}
                role={props.role.clone()}
                aria-label={props.label.clone()}
            >
                { for props.children.iter() }
            </div>
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct CardHeader {}

#[derive(Properties, Clone, PartialEq)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    /// Aria label used for assistive technologies
    #[prop_or_default]
    pub label: String,

    /// Role, used for assistive technoligies to describe the purpose of the group.
    #[prop_or_default]
    pub role: String,
}

impl Component for CardHeader {
    type Message = ();
    type Properties = CardHeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let classes = classes!("card-header", &props.class);
        html! {
            <div
                class={classes}
                role={props.role.clone()}
                aria-label={props.label.clone()}
            >
                { for props.children.iter() }
            </div>
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct CardFooter {}

#[derive(Properties, Clone, PartialEq)]
pub struct CardFooterProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,

    /// Aria label used for assistive technologies
    #[prop_or_default]
    pub label: String,

    /// Role, used for assistive technoligies to describe the purpose of the group.
    #[prop_or_default]
    pub role: String,
}

impl Component for CardFooter {
    type Message = ();
    type Properties = CardFooterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let classes = classes!("card-footer", &props.class);
        html! {
            <div
                class={classes}
                role={props.role.clone()}
                aria-label={props.label.clone()}
            >
                { for props.children.iter() }
            </div>
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct CardTitle {}

#[derive(Properties, Clone, PartialEq)]
pub struct CardTitleProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub size: HeaderSize,

    #[prop_or_default]
    pub children: Children,
}

impl Component for CardTitle {
    type Message = ();
    type Properties = CardTitleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let classes = classes!("card-title", &props.class);
        match props.size {
            HeaderSize::One => {
                html! {
                    <h1 class={classes}>
                        { for props.children.iter() }
                    </h1>
                }
            }
            HeaderSize::Two => {
                html! {
                    <h2 class={classes}>
                        { for props.children.iter() }
                    </h2>
                }
            }
            HeaderSize::Three => {
                html! {
                    <h3 class={classes}>
                        { for props.children.iter() }
                    </h3>
                }
            }
            HeaderSize::Four => {
                html! {
                    <h4 class={classes}>
                        { for props.children.iter() }
                    </h4>
                }
            }
            HeaderSize::Five => {
                html! {
                    <h5 class={classes}>
                        { for props.children.iter() }
                    </h5>
                }
            }
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct CardText {}

#[derive(Properties, Clone, PartialEq)]
pub struct CardTextProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub children: Children,
}

impl Component for CardText {
    type Message = ();
    type Properties = CardTitleProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let classes = classes!("card-text", &props.class);
        html! {
            <p class={classes}>
                { for props.children.iter() }
            </p>
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub enum HeaderSize {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Default, Clone, PartialEq)]
pub struct CardLink {}

#[derive(Properties, Clone, PartialEq)]
pub struct CardLinkProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub href: String,

    #[prop_or_default]
    pub children: Children,
}

impl Component for CardLink {
    type Message = ();
    type Properties = CardLinkProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let classes = classes!("card-link", &props.class);
        html! {
            <a class={classes} href={props.href.clone()}>
                { for props.children.iter() }
            </a>
        }
    }
}
