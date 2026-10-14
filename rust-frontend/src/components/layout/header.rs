//! Header component

use yew::prelude::*;
use yew_router::prelude::*;

use super::Layout;
use crate::components::common::{Button, ButtonVariant, Logo};

/// Header component
#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-brand-green-900/80 backdrop-blur-sm border-b border-brand-green-800">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    {/* Logo and navigation */}
                    <div class="flex items-center space-x-8">
                        <Link<Route> to={Route::Home}>
                            <Logo />
                        </Link<Route>>

                        {/* Desktop navigation */}
                        <nav class="hidden md:flex space-x-6">
                            <Link<Route> to={Route::Clubs} classes="text-brand-green-200 hover:text-white transition-colors">
                                {"Clubs"}
                            </Link<Route>>
                            <Link<Route> to={Route::Joueurs} classes="text-brand-green-200 hover:text-white transition-colors">
                                {"Joueurs"}
                            </Link<Route>>
                            <Link<Route> to={Route::Tournois} classes="text-brand-green-200 hover:text-white transition-colors">
                                {"Tournois"}
                            </Link<Route>>
                            <Link<Route> to={Route::Championnats} classes="text-brand-green-200 hover:text-white transition-colors">
                                {"Championnats"}
                            </Link<Route>>
                            <Link<Route> to={Route::Actualites} classes="text-brand-green-200 hover:text-white transition-colors">
                                {"Actualités"}
                            </Link<Route>>
                            <Link<Route> to={Route::Federation} classes="text-brand-green-200 hover:text-white transition-colors">
                                {"Fédération"}
                            </Link<Route>>
                        </nav>
                    </div>

                    {/* Auth buttons */}
                    <div class="flex items-center space-x-4">
                        <Link<Route> to={Route::Connexion}>
                            <Button variant="ghost" size="s">
                                {"Connexion"}
                            </Button>
                        </Link<Route>>
                        <Link<Route> to={Route::Inscription}>
                            <Button variant="primary" size="s">
                                {"Inscription"}
                            </Button>
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </header>
    }
}
