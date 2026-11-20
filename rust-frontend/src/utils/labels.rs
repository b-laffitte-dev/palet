//! Libellés français des enums partagés (affichage uniquement).
//!
//! Pure Rust, donc testable hors navigateur.

use palet_shared::{
    CompetitionCategory, CompetitionFormat, CompetitionMaterial, CompetitionStatus,
    CompetitionType, MatchResult, MatchStatus, RegistrationStatus, TeamClassificationStatus,
};

pub fn competition_status_label(s: &CompetitionStatus) -> &'static str {
    match s {
        CompetitionStatus::Pending => "À venir",
        CompetitionStatus::Registration => "Inscriptions ouvertes",
        CompetitionStatus::InProgress => "En cours",
        CompetitionStatus::Completed => "Terminée",
        CompetitionStatus::Cancelled => "Annulée",
    }
}

pub fn competition_type_label(t: &CompetitionType) -> &'static str {
    match t {
        CompetitionType::Championship => "Championnat",
        CompetitionType::Tournament => "Tournoi",
        CompetitionType::Cup => "Coupe",
        CompetitionType::Friendly => "Amical",
    }
}

pub fn competition_category_label(c: &CompetitionCategory) -> &'static str {
    match c {
        CompetitionCategory::Individual => "Individuel",
        CompetitionCategory::Doublette => "Doublette",
        CompetitionCategory::Triplette => "Triplette",
    }
}

pub fn competition_material_label(m: &CompetitionMaterial) -> &'static str {
    match m {
        CompetitionMaterial::Fonte => "Fonte",
        CompetitionMaterial::Laiton => "Laiton",
        CompetitionMaterial::Bois => "Bois",
        CompetitionMaterial::Mixte => "Mixte",
    }
}

pub fn competition_format_label(f: &CompetitionFormat) -> &'static str {
    match f {
        CompetitionFormat::RoundRobin => "Toutes contre toutes",
        CompetitionFormat::League => "Ligue",
        CompetitionFormat::Groups => "Poules",
        CompetitionFormat::Knockout => "Élimination directe",
    }
}

pub fn match_status_label(s: &MatchStatus) -> &'static str {
    match s {
        MatchStatus::Scheduled => "Programmé",
        MatchStatus::InProgress => "En cours",
        MatchStatus::Completed => "Terminé",
        MatchStatus::Postponed => "Reporté",
        MatchStatus::Cancelled => "Annulé",
        MatchStatus::Disputed => "Contesté",
    }
}

/// Lettre de forme pour le classement : V / N / D.
pub fn match_result_letter(r: &MatchResult) -> char {
    match r {
        MatchResult::Win => 'V',
        MatchResult::Draw => 'N',
        MatchResult::Loss => 'D',
    }
}

pub fn registration_status_label(s: &RegistrationStatus) -> &'static str {
    match s {
        RegistrationStatus::Pending => "En attente",
        RegistrationStatus::Confirmed => "Confirmée",
        RegistrationStatus::Waitlist => "Liste d'attente",
        RegistrationStatus::Cancelled => "Annulée",
    }
}

pub fn classification_status_label(s: &TeamClassificationStatus) -> &'static str {
    match s {
        TeamClassificationStatus::Qualified => "Qualifié",
        TeamClassificationStatus::Barrage => "Barrage",
        TeamClassificationStatus::Relegated => "Relégué",
        TeamClassificationStatus::Safe => "Maintenu",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_variants_have_a_label() {
        let statuses = [
            CompetitionStatus::Pending,
            CompetitionStatus::Registration,
            CompetitionStatus::InProgress,
            CompetitionStatus::Completed,
            CompetitionStatus::Cancelled,
        ];
        for s in statuses {
            assert!(!competition_status_label(&s).is_empty());
        }

        let types = [
            CompetitionType::Championship,
            CompetitionType::Tournament,
            CompetitionType::Cup,
            CompetitionType::Friendly,
        ];
        for t in types {
            assert!(!competition_type_label(&t).is_empty());
        }

        let results = [MatchResult::Win, MatchResult::Draw, MatchResult::Loss];
        assert_eq!(
            results.iter().map(match_result_letter).collect::<Vec<_>>(),
            vec!['V', 'N', 'D']
        );

        let regs = [
            RegistrationStatus::Pending,
            RegistrationStatus::Confirmed,
            RegistrationStatus::Waitlist,
            RegistrationStatus::Cancelled,
        ];
        for r in regs {
            assert!(!registration_status_label(&r).is_empty());
        }
    }
}
