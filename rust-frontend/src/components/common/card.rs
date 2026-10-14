//! Card component

use yew::prelude::*;

/// Card variant
#[derive(Debug, Clone, PartialEq)]
pub enum CardVariant {
    Default,
    Bordered,
    Elevated,
}

/// Card component properties
#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub variant: CardVariant,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

/// Card component
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let CardProps {
        variant,
        children,
        classes,
    } = props;

    let variant_classes = match variant {
        CardVariant::Default => "bg-brand-green-800/30",
        CardVariant::Bordered => "bg-brand-green-800/20 border border-brand-green-700",
        CardVariant::Elevated => "bg-brand-green-800/50 shadow-lg",
    };

    let base_classes = "rounded-xl p-6";

    let combined_classes = classes!(
        base_classes,
        variant_classes,
        classes.as_deref().unwrap_or("")
    );

    html! {
        <div class={combined_classes}>
            {children.clone()}
        </div>
    }
}

/// Card header component
#[derive(Properties, PartialEq)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

#[function_component(CardHeader)]
pub fn card_header(props: &CardHeaderProps) -> Html {
    let CardHeaderProps { children, classes } = props;

    let base_classes = "mb-4";

    let combined_classes = classes!(base_classes, classes.as_deref().unwrap_or(""));

    html! {
        <div class={combined_classes}>
            {children.clone()}
        </div>
    }
}

/// Card body component
#[derive(Properties, PartialEq)]
pub struct CardBodyProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

#[function_component(CardBody)]
pub fn card_body(props: &CardBodyProps) -> Html {
    let CardBodyProps { children, classes } = props;

    let base_classes = "";

    let combined_classes = classes!(base_classes, classes.as_deref().unwrap_or(""));

    html! {
        <div class={combined_classes}>
            {children.clone()}
        </div>
    }
}

/// Card footer component
#[derive(Properties, PartialEq)]
pub struct CardFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

#[function_component(CardFooter)]
pub fn card_footer(props: &CardFooterProps) -> Html {
    let CardFooterProps { children, classes } = props;

    let base_classes = "mt-4 pt-4 border-t border-brand-green-700/30";

    let combined_classes = classes!(base_classes, classes.as_deref().unwrap_or(""));

    html! {
        <div class={combined_classes}>
            {children.clone()}
        </div>
    }
}
