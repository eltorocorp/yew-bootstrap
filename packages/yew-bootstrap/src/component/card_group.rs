
use yew::prelude::*;

/// # Card group
/// [CardGroup] is used to group several [crate::component::Card] instances together.
/// Cards can be arranged vertically.
/// 
/// See [CardGroupProps] for a listing of properties.
/// 
/// ## Example
/// Example of a simple card group:
/// 
/// ```rust
/// use yew::prelude::*;
/// use yew_bootstrap::component::{Card, CardGroup};
/// use yew_bootstrap::util::Color;
/// fn test() -> Html {
///     html!{
///         <CardGroup class={ "class" }>
///             <Card style={Color::Primary} text={ "First button" }/>
///             <Card style={Color::Secondary} text={ "Second button" }/>
///         </CardGroup>
///     }
/// }
/// ```
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

    /// If true, disposition is vertical (Default horizontal)
    #[prop_or_default]
    pub vertical: bool,
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
        if props.vertical {
            classes.push("btn-group-vertical");
        } else {
            classes.push("btn-group");
        }
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
