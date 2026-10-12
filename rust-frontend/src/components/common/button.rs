//! Button component

use yew::prelude::*;

/// Button variant
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Tertiary,
    Danger,
    Ghost,
}

/// Button size
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonSize {
    S,
    M,
    L,
    Xl,
}

/// Button component properties
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub classes: Option<String>,
    #[prop_or_default]
    pub r#type: Option<String>,
}

/// Button component
#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        variant,
        size,
        disabled,
        loading,
        children,
        onclick,
        classes,
        r#type,
    } = props;

    // Determine button classes based on variant and size
    let variant_classes = match variant {
        ButtonVariant::Primary => "bg-brand-green-600 hover:bg-brand-green-500 text-white",
        ButtonVariant::Secondary => "bg-brand-green-800/50 hover:bg-brand-green-700/50 text-white border border-brand-green-700",
        ButtonVariant::Tertiary => "bg-transparent hover:bg-brand-green-800/20 text-brand-green-200 border border-brand-green-700",
        ButtonVariant::Danger => "bg-red-600 hover:bg-red-500 text-white",
        ButtonVariant::Ghost => "bg-transparent hover:bg-brand-green-800/10 text-brand-green-200",
    };

    let size_classes = match size {
        ButtonSize::S => "px-3 py-1.5 text-sm",
        ButtonSize::M => "px-4 py-2 text-body-s",
        ButtonSize::L => "px-6 py-3 text-body-m",
        ButtonSize::Xl => "px-8 py-4 text-body-l",
    };

    let base_classes = "inline-flex items-center justify-center font-medium rounded-lg transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-brand-green-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed";

    let combined_classes = classes!(
        base_classes,
        variant_classes,
        size_classes,
        classes.as_deref().unwrap_or("")
    );

    let button_type = r#type.as_deref().unwrap_or("button");

    html! {
        <button
            type={button_type}
            class={combined_classes}
            disabled={*disabled || *loading}
            onclick={onclick.clone()}
        >
            if *loading {
                {/* Loading spinner */}
                <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-current" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
            }
            {children.clone()}
        </button>
    }
}

/// Link component that looks like a button
#[derive(Properties, PartialEq)]
pub struct LinkButtonProps {
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub to: String,
    #[prop_or_default]
    pub classes: Option<String>,
}

#[function_component(LinkButton)]
pub fn link_button(props: &LinkButtonProps) -> Html {
    let LinkButtonProps {
        variant,
        size,
        children,
        to,
        classes,
    } = props;

    // Determine button classes based on variant and size
    let variant_classes = match variant {
        ButtonVariant::Primary => "bg-brand-green-600 hover:bg-brand-green-500 text-white",
        ButtonVariant::Secondary => "bg-brand-green-800/50 hover:bg-brand-green-700/50 text-white border border-brand-green-700",
        ButtonVariant::Tertiary => "bg-transparent hover:bg-brand-green-800/20 text-brand-green-200 border border-brand-green-700",
        ButtonVariant::Danger => "bg-red-600 hover:bg-red-500 text-white",
        ButtonVariant::Ghost => "bg-transparent hover:bg-brand-green-800/10 text-brand-green-200",
    };

    let size_classes = match size {
        ButtonSize::S => "px-3 py-1.5 text-sm",
        ButtonSize::M => "px-4 py-2 text-body-s",
        ButtonSize::L => "px-6 py-3 text-body-m",
        ButtonSize::Xl => "px-8 py-4 text-body-l",
    };

    let base_classes = "inline-flex items-center justify-center font-medium rounded-lg transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-brand-green-500 focus:ring-offset-2";

    let combined_classes = classes!(
        base_classes,
        variant_classes,
        size_classes,
        classes.as_deref().unwrap_or("")
    );

    html! {
        <a href={to.clone()} class={combined_classes}>
            {children.clone()}
        </a>
    }
}
