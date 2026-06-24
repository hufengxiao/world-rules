//! 健康规则模块

pub mod addiction_recovery;
pub mod anxiety_management;
pub mod bone_health;
pub mod brain_health;
pub mod cancer_prevention;
pub mod children_health;
pub mod chronic_disease;
pub mod dental_health;
pub mod depression_awareness;
pub mod detox_rules;
pub mod diabetes_management;
pub mod digestive_health;
pub mod elderly_health;
pub mod ergonomics;
pub mod exercise;
pub mod eye_health;
pub mod eye_strain_rules;
pub mod first_aid;
pub mod gut_health;
pub mod hair_health;
pub mod hearing_health;
pub mod heart_health;
pub mod hydration_rules;
pub mod hypertension_management;
pub mod immune_health;
pub mod infant_health;
pub mod joint_health;
pub mod kidney_health;
pub mod liver_health;
pub mod men_health;
pub mod mental_health;
pub mod mental_wellness;
pub mod nutrition;
pub mod occupational_health;
pub mod oral_health;
pub mod postpartum_health;
pub mod posture_rules;
pub mod prenatal_health;
pub mod respiratory_health;
pub mod skin_care_rules;
pub mod skin_health;
pub mod sleep;
pub mod sleep_hygiene;
pub mod stress_management;
pub mod teen_health;
pub mod toddler_health;
pub mod travel_health;
pub mod vaccination_rules;
pub mod weight_management;
pub mod women_health;

pub use addiction_recovery::AddictionRecoveryRules;
pub use anxiety_management::AnxietyManagementRules;
pub use bone_health::BoneHealthRules;
pub use brain_health::BrainHealthRules;
pub use cancer_prevention::CancerPreventionRules;
pub use children_health::ChildrenHealthRules;
pub use chronic_disease::ChronicDiseaseRules;
pub use dental_health::DentalHealthRules;
pub use depression_awareness::DepressionAwarenessRules;
pub use detox_rules::DetoxRulesRules;
pub use diabetes_management::DiabetesManagementRules;
pub use digestive_health::DigestiveHealthRules;
pub use elderly_health::ElderlyHealthRules;
pub use ergonomics::ErgonomicsRules;
pub use exercise::ExerciseRules;
pub use eye_health::EyeHealthRules;
pub use eye_strain_rules::EyeStrainRulesRules;
pub use first_aid::FirstAidRules;
pub use gut_health::GutHealthRules;
pub use hair_health::HairHealthRules;
pub use hearing_health::HearingHealthRules;
pub use heart_health::HeartHealthRules;
pub use hydration_rules::HydrationRulesRules;
pub use hypertension_management::HypertensionManagementRules;
pub use immune_health::ImmuneHealthRules;
pub use infant_health::InfantHealthRules;
pub use joint_health::JointHealthRules;
pub use kidney_health::KidneyHealthRules;
pub use liver_health::LiverHealthRules;
pub use men_health::MenHealthRules;
pub use mental_health::MentalHealthRules;
pub use mental_wellness::MentalWellnessRules;
pub use nutrition::NutritionRules;
pub use occupational_health::OccupationalHealthRules;
pub use oral_health::OralHealthRules;
pub use postpartum_health::PostpartumHealthRules;
pub use posture_rules::PostureRulesRules;
pub use prenatal_health::PrenatalHealthRules;
pub use respiratory_health::RespiratoryHealthRules;
pub use skin_care_rules::SkinCareRulesRules;
pub use skin_health::SkinHealthRules;
pub use sleep::SleepRules;
pub use sleep_hygiene::SleepHygieneRules;
pub use stress_management::StressManagementRules;
pub use teen_health::TeenHealthRules;
pub use toddler_health::ToddlerHealthRules;
pub use travel_health::TravelHealthRules;
pub use vaccination_rules::VaccinationRulesRules;
pub use weight_management::WeightManagementRules;
pub use women_health::WomenHealthRules;

pub fn all_rules() -> Vec<(
    &'static str,
    crate::rules::core::RuleMetadata,
    crate::rules::core::RuleCategory,
    String,
)> {
    use crate::rules::core::Rule;
    let mut rules = Vec::new();
    {
        let r = ChildrenHealthRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChronicDiseaseRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElderlyHealthRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ExerciseRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EyeHealthRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MentalHealthRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MentalWellnessRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NutritionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OccupationalHealthRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OralHealthRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SkinHealthRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SleepRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    rules
}
