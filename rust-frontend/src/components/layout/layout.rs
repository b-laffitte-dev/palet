//! Main layout component

use yew::prelude::*;
use yew_router::prelude::*;

use super::{Footer, Header, MobileNav, StickyHeader};

/// Layout component properties
#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

/// Main layout component
///
/// Provides the overall structure for all pages including header, footer, and navigation
#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    let children = props.children.clone();

    html! {
        <div class="min-h-screen bg-brand-green-900 text-brand-green-100 flex flex-col">
            {/* Sticky header for desktop */}
            <StickyHeader />

            {/* Main content */}
            <main class="flex-1">
                {children}
            </main>

            {/* Footer */}
            <Footer />

            {/* Mobile navigation */}
            <MobileNav />
        </div>
    }
}

/// Link component that works with yew-router
#[derive(Properties, PartialEq)]
pub struct LinkProps {
    #[prop_or_default]
    pub to: Route,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub classes: Option<String>,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let LinkProps {
        to,
        children,
        classes,
    } = props;

    let base_classes =
        "text-brand-green-400 hover:text-brand-green-300 transition-colors duration-200";
    let combined_classes = classes!(
        base_classes,
        classes.as_ref().map(|s| s.as_str()).unwrap_or("")
    );

    html! {
        <yew_router::components::Link<Route> to={to.clone()} classes={combined_classes}>
            {children.clone()}
        </yew_router::components::Link<Route>>
    }
}
