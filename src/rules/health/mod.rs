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
pub mod allergy_management;
pub mod anemia_nutrition;
pub mod anger_management;
pub mod ankle_sprain_care;
pub mod anxiety_management;
pub mod balanced_diet_plan;
pub mod bleeding_control_bandage;
pub mod blood_lipid_management;
pub mod bone_health;
pub mod brain_health;
pub mod burnout_prevention;
pub mod cancer_prevention;
pub mod children_health;
pub mod choking_airway_relief;
pub mod chronic_disease;
pub mod common_cold_care;
pub mod constipation_relief;
pub mod cpr_and_emergency;
pub mod cut_wound_care;
pub mod dental_health;
pub mod depression_awareness;
pub mod depression_support;
pub mod detox_rules;
pub mod diabetes_management;
pub mod digestive_health;
pub mod drowning_rescue;
pub mod ear_ache_care;
pub mod earthquake_safety;
pub mod elder_fall_prevention;
pub mod elderly_health;
pub mod emotional_regulation;
pub mod ergonomics;
pub mod exercise;
pub mod eye_health;
pub mod eye_health_general;
pub mod eye_strain_rules;
pub mod fever_care;
pub mod first_aid;
pub mod food_poisoning_care;
pub mod food_safety;
pub mod foot_blister_care;
pub mod gout_management;
pub mod gratitude_practice;
pub mod gut_health;
pub mod hair_health;
pub mod hand_washing_hygiene;
pub mod hearing_health;
pub mod heart_health;
pub mod heatstroke_prevention;
pub mod hiccup_relief;
pub mod home_first_aid_kit;
pub mod hydration_rules;
pub mod hydration_strategy;
pub mod hypertension_lifestyle;
pub mod hypertension_management;
pub mod immune_health;
pub mod immune_support_lifestyle;
pub mod infant_care_basics;
pub mod infant_health;
pub mod insect_bite_care;
pub mod joint_health;
pub mod kidney_health;
pub mod knee_care;
pub mod leg_cramp_relief;
pub mod liver_health;
pub mod loneliness_connection;
pub mod men_health;
pub mod menopause_health;
pub mod mental_health;
pub mod mental_wellness;
pub mod migraine_management;
pub mod mindfulness_basics;
pub mod motion_sickness_relief;
pub mod muscle_stretch_routine;
pub mod nasal_congestion_relief;
pub mod nutrition;
pub mod nutrition_basics;
pub mod occupational_health;
pub mod office_ergonomics;
pub mod oral_health;
pub mod oral_ulcer_care;
pub mod postpartum_health;
pub mod postpartum_recovery;
pub mod posture_rules;
pub mod pregnancy_prenatal;
pub mod premenstrual_syndrome_care;
pub mod prenatal_health;
pub mod procrastination_manage;
pub mod prolonged_sitting_safety;
pub mod respiratory_health;
pub mod scalds_burns;
pub mod screen_eye_strain_care;
pub mod seasonal_health;
pub mod seizure_response;
pub mod self_compassion;
pub mod skin_care_rules;
pub mod skin_health;
pub mod skin_itch_soothing;
pub mod sleep;
pub mod sleep_apnea;
pub mod sleep_hygiene;
pub mod sleep_quality;
pub mod smoking_cessation;
pub mod sore_throat_relief;
pub mod stress_management;
pub mod stress_relief_life;
pub mod sun_protection;
pub mod teen_health;
pub mod thyroid_care;
pub mod tinnitus_ringing_relief;
pub mod toddler_health;
pub mod travel_health;
pub mod vaccination_rules;
pub mod vegetarian_balanced_nutrition;
pub mod vertigo_balance_care;
pub mod vision_care;
pub mod weight_management;
pub mod women_health;

pub use addiction_recovery::AddictionRecoveryRules;
pub use allergy_management::AllergyManagementRules;
pub use anemia_nutrition::AnemiaNutritionRules;
pub use anger_management::AngerManagementRules;
pub use ankle_sprain_care::AnkleSprainCareRules;
pub use anxiety_management::AnxietyManagementRules;
pub use balanced_diet_plan::BalancedDietPlanRules;
pub use bleeding_control_bandage::BleedingControlBandageRules;
pub use blood_lipid_management::BloodLipidManagementRules;
pub use bone_health::BoneHealthRules;
pub use brain_health::BrainHealthRules;
pub use burnout_prevention::BurnoutPreventionRules;
pub use cancer_prevention::CancerPreventionRules;
pub use children_health::ChildrenHealthRules;
pub use choking_airway_relief::ChokingAirwayReliefRules;
pub use chronic_disease::ChronicDiseaseRules;
pub use common_cold_care::CommonColdCareRules;
pub use constipation_relief::ConstipationReliefRules;
pub use cpr_and_emergency::CprEmergencyRules;
pub use cut_wound_care::CutWoundCareRules;
pub use dental_health::DentalHealthRules;
pub use depression_awareness::DepressionAwarenessRules;
pub use depression_support::DepressionSupportRules;
pub use detox_rules::DetoxRulesRules;
pub use diabetes_management::DiabetesManagementRules;
pub use digestive_health::DigestiveHealthRules;
pub use drowning_rescue::DrowningRescueRules;
pub use ear_ache_care::EarAcheCareRules;
pub use earthquake_safety::EarthquakeSafetyRules;
pub use elder_fall_prevention::ElderFallPreventionRules;
pub use elderly_health::ElderlyHealthRules;
pub use emotional_regulation::EmotionalRegulationRules;
pub use ergonomics::ErgonomicsRules;
pub use exercise::ExerciseRules;
pub use eye_health::EyeHealthRules;
pub use eye_health_general::EyeHealthGeneralRules;
pub use eye_strain_rules::EyeStrainRulesRules;
pub use fever_care::FeverCareRules;
pub use first_aid::FirstAidRules;
pub use food_poisoning_care::FoodPoisoningCareRules;
pub use food_safety::FoodSafetyRules;
pub use foot_blister_care::FootBlisterCareRules;
pub use gout_management::GoutManagementRules;
pub use gratitude_practice::GratitudePracticeRules;
pub use gut_health::GutHealthRules;
pub use hair_health::HairHealthRules;
pub use hand_washing_hygiene::HandWashingHygieneRules;
pub use hearing_health::HearingHealthRules;
pub use heart_health::HeartHealthRules;
pub use heatstroke_prevention::HeatstrokePreventionRules;
pub use hiccup_relief::HiccupReliefRules;
pub use home_first_aid_kit::HomeFirstAidKitRules;
pub use hydration_rules::HydrationRulesRules;
pub use hydration_strategy::HydrationStrategyRules;
pub use hypertension_lifestyle::HypertensionLifestyleRules;
pub use hypertension_management::HypertensionManagementRules;
pub use immune_health::ImmuneHealthRules;
pub use immune_support_lifestyle::ImmuneSupportLifestyleRules;
pub use infant_care_basics::InfantCareBasicsRules;
pub use infant_health::InfantHealthRules;
pub use insect_bite_care::InsectBiteCareRules;
pub use joint_health::JointHealthRules;
pub use kidney_health::KidneyHealthRules;
pub use knee_care::KneeCareRules;
pub use leg_cramp_relief::LegCrampReliefRules;
pub use liver_health::LiverHealthRules;
pub use loneliness_connection::LonelinessConnectionRules;
pub use men_health::MenHealthRules;
pub use menopause_health::MenopauseHealthRules;
pub use mental_health::MentalHealthRules;
pub use mental_wellness::MentalWellnessRules;
pub use migraine_management::MigraineManagementRules;
pub use mindfulness_basics::MindfulnessBasicsRules;
pub use motion_sickness_relief::MotionSicknessReliefRules;
pub use muscle_stretch_routine::MuscleStretchRoutineRules;
pub use nasal_congestion_relief::NasalCongestionReliefRules;
pub use nutrition::NutritionRules;
pub use nutrition_basics::NutritionBasicsRules;
pub use occupational_health::OccupationalHealthRules;
pub use office_ergonomics::OfficeErgonomicsRules;
pub use oral_health::OralHealthRules;
pub use oral_ulcer_care::OralUlcerCareRules;
pub use postpartum_health::PostpartumHealthRules;
pub use postpartum_recovery::PostpartumRecoveryRules;
pub use posture_rules::PostureRulesRules;
pub use pregnancy_prenatal::PregnancyPrenatalRules;
pub use premenstrual_syndrome_care::PremenstrualSyndromeCareRules;
pub use prenatal_health::PrenatalHealthRules;
pub use procrastination_manage::ProcrastinationManageRules;
pub use prolonged_sitting_safety::ProlongedSittingSafetyRules;
pub use respiratory_health::RespiratoryHealthRules;
pub use scalds_burns::ScaldBurnCareRules;
pub use screen_eye_strain_care::ScreenEyeStrainCareRules;
pub use seasonal_health::SeasonalHealthRules;
pub use seizure_response::SeizureResponseRules;
pub use self_compassion::SelfCompassionRules;
pub use skin_care_rules::SkinCareRulesRules;
pub use skin_health::SkinHealthRules;
pub use skin_itch_soothing::SkinItchSoothingRules;
pub use sleep::SleepRules;
pub use sleep_apnea::SleepApneaRules;
pub use sleep_hygiene::SleepHygieneRules;
pub use sleep_quality::SleepQualityRules;
pub use smoking_cessation::SmokingCessationRules;
pub use sore_throat_relief::SoreThroatReliefRules;
pub use stress_management::StressManagementRules;
pub use stress_relief_life::StressReliefRules;
pub use sun_protection::SunProtectionRules;
pub use teen_health::TeenHealthRules;
pub use thyroid_care::ThyroidCareRules;
pub use tinnitus_ringing_relief::TinnitusRingingReliefRules;
pub use toddler_health::ToddlerHealthRules;
pub use travel_health::TravelHealthRules;
pub use vaccination_rules::VaccinationRulesRules;
pub use vegetarian_balanced_nutrition::VegetarianBalancedNutritionRules;
pub use vertigo_balance_care::VertigoBalanceCareRules;
pub use vision_care::VisionCareRules;
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
        let r = GratitudePracticeRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ProcrastinationManageRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LonelinessConnectionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AngerManagementRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SelfCompassionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TinnitusRingingReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ScreenEyeStrainCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NasalCongestionReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HiccupReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EarAcheCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ProlongedSittingSafetyRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InfantCareBasicsRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PostpartumRecoveryRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MenopauseHealthRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PregnancyPrenatalRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BurnoutPreventionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MindfulnessBasicsRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EmotionalRegulationRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DepressionSupportRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = StressReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BleedingControlBandageRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MotionSicknessReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SkinItchSoothingRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LegCrampReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SoreThroatReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OralUlcerCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PremenstrualSyndromeCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VertigoBalanceCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SmokingCessationRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AnemiaNutritionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GoutManagementRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ConstipationReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EarthquakeSafetyRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HomeFirstAidKitRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DrowningRescueRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SeizureResponseRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChokingAirwayReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CprEmergencyRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ImmuneSupportLifestyleRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HydrationStrategyRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VegetarianBalancedNutritionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NutritionBasicsRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BalancedDietPlanRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EyeHealthGeneralRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SleepApneaRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ThyroidCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = KneeCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BloodLipidManagementRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HandWashingHygieneRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OfficeErgonomicsRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MuscleStretchRoutineRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SleepQualityRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElderFallPreventionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SunProtectionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FootBlisterCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HeatstrokePreventionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VisionCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SeasonalHealthRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InsectBiteCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FoodPoisoningCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AnkleSprainCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CutWoundCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ScaldBurnCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FeverCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CommonColdCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FoodSafetyRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HypertensionLifestyleRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AllergyManagementRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MigraineManagementRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
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
