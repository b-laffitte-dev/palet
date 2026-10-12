//! Home page component

use palet_shared::{Club, Match, Player, Tournament, Championship};
use yew::prelude::*;

use crate::{
    components::{
        common::*,
        layout::Layout,
    },
    services::api::UseApi,
};

/// Home page component
#[function_component(Home)]
pub fn home() -> Html {
    let api = UseApi();
    
    // State for loading and data
    let clubs = use_state(|| Vec::<Club>::new());
    let tournaments = use_state(|| Vec::<Tournament>::new());
    let championships = use_state(|| Vec::<Championship>::new());
    let featured_matches = use_state(|| Vec::<Match>::new());
    let featured_players = use_state(|| Vec::<Player>::new());
    
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    // Clone API for async callbacks
    let api_clone = api.clone();
    let clubs_clone = clubs.clone();
    let loading_clone = loading.clone();
    let error_clone = error.clone();

    // Fetch data on component mount
    use_effect(move || {
        let api = api_clone.clone();
        let clubs = clubs_clone.clone();
        let loading = loading_clone.clone();
        let error = error_clone.clone();

        wasm_bindgen_futures::spawn_local(async move {
            match api.get_clubs().await {
                Ok(response) => {
                    clubs.set(response.data);
                    loading.set(false);
                }
                Err(e) => {
                    error.set(Some(e.message));
                    loading.set(false);
                }
            }
        });

        || ()
    });

    if *loading {
        return html! {
            <Layout>
                <main class="flex-1 px-4 py-6 lg:px-8 lg:py-8">
                    <div class="max-w-7xl mx-auto">
                        <div class="text-center py-12">
                            <p class="text-body-m text-brand-green-200">{"Chargement..."}</p>
                        </div>
                    </div>
                </main>
            </Layout>
        };
    }

    if let Some(err) = &*error {
        return html! {
            <Layout>
                <main class="flex-1 px-4 py-6 lg:px-8 lg:py-8">
                    <div class="max-w-7xl mx-auto">
                        <div class="text-center py-12">
                            <p class="text-body-m text-red-500">{err.clone()}</p>
                        </div>
                    </div>
                </main>
            </Layout>
        };
    }

    html! {
        <Layout>
            <main class="flex-1 px-4 py-6 lg:px-8 lg:py-8">
                <div class="max-w-7xl mx-auto">
                    {/* Hero section */}
                    <section class="mb-12">
                        <div class="bg-gradient-to-r from-brand-green-800 to-brand-green-600 rounded-2xl p-8 lg:p-12 text-center">
                            <h1 class="text-heading-xl lg:text-heading-2xl font-bold text-white uppercase tracking-wide mb-4">
                                {"Bienvenue sur Palet Vendéen"}
                            </h1>
                            <p class="text-body-l text-brand-green-100 mb-8">
                                {"La plateforme de fédération des joueurs, clubs et fédérations de palet vendéen"}
                            </p>
                            <div class="flex flex-col sm:flex-row gap-4 justify-center">
                                <Button variant="primary" size="l">
                                    {"Voir les clubs"}
                                </Button>
                                <Button variant="secondary" size="l">
                                    {"Voir les tournois"}
                                </Button>
                            </div>
                        </div>
                    </section>

                    {/* Featured sections */}
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-12">
                        {/* Clubs section */}
                        <section>
                            <div class="flex items-center justify-between mb-4">
                                <h2 class="text-heading-m font-bold text-white uppercase tracking-wide">
                                    {"Clubs"}
                                </h2>
                                <Link<Route> to={Route::Clubs} classes="text-brand-green-400 hover:text-brand-green-300">
                                    {"Voir tous"}
                                </Link<Route>>
                            </div>
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                { for clubs.iter().take(4).map(|club| html! {
                                    <ClubCard key={club.id.to_string()} club={club.clone()} />
                                }) }
                            </div>
                        </section>

                        {/* Tournaments section */}
                        <section>
                            <div class="flex items-center justify-between mb-4">
                                <h2 class="text-heading-m font-bold text-white uppercase tracking-wide">
                                    {"Tournois"}
                                </h2>
                                <Link<Route> to={Route::Tournois} classes="text-brand-green-400 hover:text-brand-green-300">
                                    {"Voir tous"}
                                </Link<Route>>
                            </div>
                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                {/* Tournament cards would go here */}
                                <div class="bg-brand-green-800/50 rounded-lg p-6 text-center">
                                    <p class="text-brand-green-200">{"Aucun tournoi disponible"}</p>
                                </div>
                            </div>
                        </section>
                    </div>

                    {/* Featured players section */}
                    <section class="mb-12">
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="text-heading-m font-bold text-white uppercase tracking-wide">
                                {"Joueurs en vedette"}
                            </h2>
                            <Link<Route> to={Route::Joueurs} classes="text-brand-green-400 hover:text-brand-green-300">
                                {"Voir tous"}
                            </Link<Route>>
                        </div>
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                            {/* Player cards would go here */}
                        </div>
                    </section>

                    {/* News section */}
                    <section>
                        <div class="flex items-center justify-between mb-4">
                            <h2 class="text-heading-m font-bold text-white uppercase tracking-wide">
                                {"Actualités"}
                            </h2>
                            <Link<Route> to={Route::Actualites} classes="text-brand-green-400 hover:text-brand-green-300">
                                {"Voir toutes"}
                            </Link<Route>>
                        </div>
                        <div class="bg-brand-green-800/50 rounded-lg p-6">
                            <p class="text-brand-green-200">{"Aucune actualité disponible"}</p>
                        </div>
                    </section>
                </div>
            </main>
        </Layout>
    }
}
