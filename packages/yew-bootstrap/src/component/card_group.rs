use yew::prelude::*;

/// # Card group
/// [CardGroup] is used to group several [crate::component::Card] instances together.
/// Cards can be arranged vertically.
///
/// See [CardGroupProps] for a listing of properties.
///
pub struct CardGroup {}

/// Properties for [CardGroup]
#[derive(Properties, Clone, PartialEq)]
pub struct CardGroupProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Children for the group (Card instances)
    #[prop_or_default]
    pub children: Children,

    /// Aria label used for assistive technologies
    #[prop_or_default]
    pub label: String,

    /// Role, used for assistive technoligies to describe the purpose of the group.
    #[prop_or_default]
    pub role: String,
}

impl Component for CardGroup {
    type Message = ();
    type Properties = CardGroupProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push("card-group");
        classes.push(props.class.clone());

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

pub struct CardDeck {}

/// Properties for [CardDeck]
#[derive(Properties, Clone, PartialEq)]
pub struct CardDeckProps {
    /// CSS class
    #[prop_or_default]
    pub class: String,

    /// Children for the group (Card instances)
    #[prop_or_default]
    pub children: Children,

    /// Aria label used for assistive technologies
    #[prop_or_default]
    pub label: String,

    /// Role, used for assistive technoligies to describe the purpose of the group.
    #[prop_or_default]
    pub role: String,
}

impl Component for CardDeck {
    type Message = ();
    type Properties = CardDeckProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let mut classes = Classes::new();
        classes.push("card-deck");
        classes.push(props.class.clone());

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
