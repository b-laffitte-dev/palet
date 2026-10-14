//! Clubs page component

use palet_shared::{Club, PaginatedResponse};
use yew::prelude::*;

use crate::{
    components::{common::*, layout::Layout},
    services::api::UseApi,
};

/// Clubs page component
#[function_component(Clubs)]
pub fn clubs() -> Html {
    let api = UseApi();

    // State for clubs and loading
    let clubs = use_state(|| Vec::<Club>::new());
    let pagination = use_state(|| PaginatedResponse::<Club>::default());
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    // Clone API for async callbacks
    let api_clone = api.clone();
    let clubs_clone = clubs.clone();
    let pagination_clone = pagination.clone();
    let loading_clone = loading.clone();
    let error_clone = error.clone();

    // Fetch data on component mount
    use_effect(move || {
        let api = api_clone.clone();
        let clubs = clubs_clone.clone();
        let pagination = pagination_clone.clone();
        let loading = loading_clone.clone();
        let error = error_clone.clone();

        wasm_bindgen_futures::spawn_local(async move {
            match api.get_clubs().await {
                Ok(response) => {
                    clubs.set(response.data.clone());
                    pagination.set(response);
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
                    {/* Page header */}
                    <div class="mb-8">
                        <h1 class="text-heading-xl font-bold text-white uppercase tracking-wide mb-2">
                            {"Clubs"}
                        </h1>
                        <p class="text-body-m text-brand-green-200">
                            {"Liste de tous les clubs de palet vendéen"}
                        </p>
                    </div>

                    {/* Clubs grid */}
                    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
                        { for clubs.iter().map(|club| html! {
                            <ClubCard key={club.id.to_string()} club={club.clone()} show_details={true} />
                        }) }
                    </div>

                    {/* Empty state */}
                    if clubs.is_empty() {
                        <div class="text-center py-12">
                            <p class="text-body-m text-brand-green-300">
                                {"Aucun club trouvé"}
                            </p>
                        </div>
                    }
                </div>
            </main>
        </Layout>
    }
}
