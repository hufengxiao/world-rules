//! 健康规则模块 - 涵盖健康管理、疾病预防和生活方式的规则
//!
//! 本模块包含健康和医疗相关的规则，覆盖：
//! - **营养健康**: 饮食规则、营养搭配、膳食指南
//! - **运动健康**: 运动规则、体能训练、康复指导
//! - **睡眠健康**: 睡眠规则、作息调整、失眠预防
//! - **心理健康**: 心理调适、压力管理、情绪控制
//! - **疾病预防**: 常见疾病预防规则、慢性病管理
//! - **特殊人群**: 儿童、老年人、孕妇健康规则
//!
//! # 模块结构
//!
//! ```text
//! health/
//! ├── nutrition         # 营养健康
//! ├── exercise          # 运动健康
//! ├── sleep             # 睡眠健康
//! ├── mental_health     # 心理健康
//! ├── first_aid         # 急救规则
//! ├── children_health   # 儿童健康
//! ├── elderly_health    # 老年人健康
//! ├── chronic_disease   # 慢性病管理
//! └── weight_management # 体重管理
//! ```
//!
//! # Examples
//!
//! 使用规则示例：
//!
//! ```rust
//! use world_rules::rules::health::{NutritionRules, ExerciseRules};
//! use world_rules::rules::core::Rule;
//!
//! // 营养规则
//! let nutrition = NutritionRules::new();
//! println!("规则: {}", nutrition.metadata().name);
//! println!("分类: {:?}", nutrition.category());
//!
//! // 运动规则
//! let exercise = ExerciseRules::new();
//! let explanation = exercise.explain();
//! assert!(!explanation.is_empty());
//! ```
//!
//! # 规则统计
//!
//! 当前包含数十条健康规则，覆盖：
//! - 10+ 条营养健康规则
//! - 10+ 条运动健康规则
//! - 10+ 条睡眠健康规则
//! - 10+ 条心理健康规则
//! - 15+ 条疾病预防规则
//! - 10+ 条特殊人群健康规则

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
