//! Button component for Palet Vendéen
#![allow(dead_code)]

use dioxus::prelude::*;

/// Button variant
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Tertiary,
    Danger,
    Ghost,
}

/// Button size
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ButtonSize {
    #[default]
    M,
    S,
    L,
    Xl,
}

/// Button component properties
#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    #[props(default)]
    variant: ButtonVariant,
    #[props(default)]
    size: ButtonSize,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    loading: bool,
    children: Element,
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    #[props(default)]
    classes: Option<String>,
    #[props(default)]
    r#type: Option<String>,
}

/// Button component
#[component]
pub fn Button(props: ButtonProps) -> Element {
    let variant_classes = match props.variant {
        ButtonVariant::Primary => "bg-brand-green-600 hover:bg-brand-green-500 text-white",
        ButtonVariant::Secondary => "bg-brand-green-800/50 hover:bg-brand-green-700/50 text-white border border-brand-green-700",
        ButtonVariant::Tertiary => "bg-transparent hover:bg-brand-green-800/20 text-brand-green-200 border border-brand-green-700",
        ButtonVariant::Danger => "bg-red-600 hover:bg-red-500 text-white",
        ButtonVariant::Ghost => "bg-transparent hover:bg-brand-green-800/10 text-brand-green-200",
    };

    let size_classes = match props.size {
        ButtonSize::S => "px-3 py-1.5 text-sm",
        ButtonSize::M => "px-4 py-2 text-body-s",
        ButtonSize::L => "px-6 py-3 text-body-m",
        ButtonSize::Xl => "px-8 py-4 text-body-l",
    };

    let base_classes = "inline-flex items-center justify-center font-medium rounded-lg transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-brand-green-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed";

    let combined_classes = format!(
        "{} {} {} {}",
        base_classes,
        variant_classes,
        size_classes,
        props.classes.as_deref().unwrap_or("")
    );

    let button_type = props.r#type.as_deref().unwrap_or("button");

    rsx! {
        button {
            r#type: "{button_type}",
            class: "{combined_classes}",
            disabled: props.disabled || props.loading,
            onclick: move |_| {},
        }
    }
}
