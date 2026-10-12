//! Club card component

use palet_shared::Club;
use yew::prelude::*;

use super::{Button, ButtonVariant, Card, CardBody, CardFooter, CardHeader};

/// Club card component properties
#[derive(Properties, PartialEq)]
pub struct ClubCardProps {
    pub club: Club,
    #[prop_or_default]
    pub show_details: bool,
}

/// Club card component
#[function_component(ClubCard)]
pub fn club_card(props: &ClubCardProps) -> Html {
    let ClubCardProps { club, show_details } = props;

    html! {
        <Card variant="bordered" classes="h-full flex flex-col">
            <CardHeader>
                <div class="flex items-center gap-4">
                    if let Some(logo) = &club.logo {
                        <img src={logo.clone()} alt={format!("Logo {}", club.name)} class="w-12 h-12 rounded-lg object-cover" />
                    } else {
                        <div class="w-12 h-12 bg-brand-green-700 rounded-lg flex items-center justify-center">
                            <span class="text-white font-bold text-xl">{&club.name.chars().next().unwrap_or('?')}</span>
                        </div>
                    }
                    <div>
                        <h3 class="text-heading-s font-bold text-white uppercase tracking-wide">
                            {&club.name}
                        </h3>
                        if let Some(nickname) = &club.nickname {
                            <p class="text-body-xs text-brand-green-300">{nickname}</p>
                        }
                    </div>
                </div>
            </CardHeader>
            
            <CardBody>
                <div class="space-y-2">
                    <p class="text-body-s text-brand-green-200">
                        <span class="font-semibold">{"Ville: "}</span>
                        {&club.city}
                    </p>
                    <p class="text-body-s text-brand-green-200">
                        <span class="font-semibold">{"Division: "}</span>
                        {&club.division}
                    </p>
                    <p class="text-body-s text-brand-green-200">
                        <span class="font-semibold">{"Joueurs: "}</span>
                        {club.players_count}
                    </p>
                    <p class="text-body-s text-brand-green-200">
                        <span class="font-semibold">{"Équipes: "}</span>
                        {club.teams_count}
                    </p>
                </div>
            </CardBody>
            
            if *show_details {
                <CardFooter>
                    <Button variant="secondary" size="s">
                        {"Voir les détails"}
                    </Button>
                </CardFooter>
            }
        </Card>
    }
}
