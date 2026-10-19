//! Card component for Palet Vendéen
#![allow(dead_code)]

use dioxus::prelude::*;

/// Card variant
#[derive(Debug, Clone, PartialEq, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Bordered,
    Elevated,
}

/// Card component properties
#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    #[props(default)]
    variant: CardVariant,
    children: Element,
    #[props(default)]
    classes: Option<String>,
}

/// Card component
#[component]
pub fn Card(props: CardProps) -> Element {
    let variant_classes = match props.variant {
        CardVariant::Default => "bg-brand-green-800/30",
        CardVariant::Bordered => "bg-brand-green-800/20 border border-brand-green-700",
        CardVariant::Elevated => "bg-brand-green-800/50 shadow-lg",
    };

    let base_classes = "rounded-xl p-6";
    let combined_classes = format!(
        "{} {} {}",
        base_classes,
        variant_classes,
        props.classes.as_deref().unwrap_or("")
    );

    rsx! {
        div { class: "{combined_classes}", {props.children} }
    }
}
