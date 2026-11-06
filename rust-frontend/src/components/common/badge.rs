//! Badge component for Palet Vendéen
#![allow(dead_code)]

use dioxus::prelude::*;

/// Badge variant
#[derive(Debug, Clone, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Info,
}

/// Badge component properties
#[derive(Props, PartialEq, Clone)]
pub struct BadgeProps {
    #[props(default)]
    variant: BadgeVariant,
    children: Element,
    #[props(default)]
    classes: Option<String>,
}

/// Badge component
#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let variant_classes = match props.variant {
        BadgeVariant::Primary => "bg-brand-green-600 text-white",
        BadgeVariant::Secondary => "bg-brand-green-800/50 text-brand-green-200",
        BadgeVariant::Success => "bg-green-500 text-white",
        BadgeVariant::Warning => "bg-yellow-500 text-black",
        BadgeVariant::Danger => "bg-red-500 text-white",
        BadgeVariant::Info => "bg-blue-500 text-white",
    };

    let base_classes = "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium";
    let combined_classes = format!(
        "{} {} {}",
        base_classes,
        variant_classes,
        props.classes.as_deref().unwrap_or("")
    );

    rsx! {
        span { class: "{combined_classes}", {props.children} }
    }
}
