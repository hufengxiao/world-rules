//! 健康规则模块

pub mod children_health;
pub mod chronic_disease;
pub mod elderly_health;
pub mod exercise;
pub mod eye_health;
pub mod mental_health;
pub mod mental_wellness;
pub mod nutrition;
pub mod occupational_health;
pub mod oral_health;
pub mod skin_health;
pub mod sleep;

pub use children_health::ChildrenHealthRules;
pub use chronic_disease::ChronicDiseaseRules;
pub use elderly_health::ElderlyHealthRules;
pub use exercise::ExerciseRules;
pub use eye_health::EyeHealthRules;
pub use mental_health::MentalHealthRules;
pub use mental_wellness::MentalWellnessRules;
pub use nutrition::NutritionRules;
pub use occupational_health::OccupationalHealthRules;
pub use oral_health::OralHealthRules;
pub use skin_health::SkinHealthRules;
pub use sleep::SleepRules;

pub fn all_rules() -> Vec<(&'static str, crate::rules::core::RuleMetadata, crate::rules::core::RuleCategory)> {
    use crate::rules::core::Rule;
    let mut rules = Vec::new();
    { let r = ChildrenHealthRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    { let r = ChronicDiseaseRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    { let r = ElderlyHealthRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    { let r = ExerciseRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    { let r = EyeHealthRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    { let r = MentalHealthRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    { let r = MentalWellnessRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    { let r = NutritionRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    { let r = OccupationalHealthRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    { let r = OralHealthRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    { let r = SkinHealthRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    { let r = SleepRules::new(); rules.push(("health", r.metadata().clone(), r.category())); }
    rules
}
