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

pub mod acid_reflux_care;
pub mod acne_skin_care;
pub mod addiction_recovery;
pub mod allergy_management;
pub mod anemia_nutrition;
pub mod anger_management;
pub mod ankle_sprain_care;
pub mod anxiety_management;
pub mod asthma_management_breathing;
pub mod baby_sleep_safety;
pub mod balanced_breakfast;
pub mod balanced_diet_plan;
pub mod balanced_diet_plate;
pub mod bedtime_ritual;
pub mod bleeding_control_bandage;
pub mod blister_care;
pub mod blood_lipid_management;
pub mod bone_health;
pub mod brain_health;
pub mod brushing_technique_bass;
pub mod burn_scalding_aid;
pub mod burnout_prevention;
pub mod calcium_bone_health;
pub mod callus_corn_foot;
pub mod cancer_prevention;
pub mod caregiver_stress_manage;
pub mod chapped_lip_care;
pub mod child_vaccination_schedule;
pub mod children_health;
pub mod choking_airway_relief;
pub mod choking_relief;
pub mod cholesterol_exercise_plan;
pub mod chronic_disease;
pub mod cold_hands_feet_warm;
pub mod common_cold_care;
pub mod constipation_relief;
pub mod correct_sitting_posture;
pub mod cpr_and_emergency;
pub mod cut_wound_care;
pub mod daily_hydration;
pub mod dementia_care_basic;
pub mod dental_health;
pub mod denture_care_cleaning;
pub mod depression_awareness;
pub mod depression_support;
pub mod detox_rules;
pub mod diabetes_exercise_safety;
pub mod diabetes_management;
pub mod dietary_fiber_vegetable;
pub mod digestive_health;
pub mod digital_eye_rest;
pub mod drowning_rescue;
pub mod dry_cough_soothes;
pub mod dry_cracked_heel;
pub mod ear_ache_care;
pub mod early_morning_routine;
pub mod earthquake_safety;
pub mod elder_fall_prevention;
pub mod elder_medication_remind;
pub mod elderly_fall_prevention;
pub mod elderly_health;
pub mod emotional_regulation;
pub mod ergonomics;
pub mod exercise;
pub mod eye_dryness_comfort;
pub mod eye_health;
pub mod eye_health_general;
pub mod eye_strain_rules;
pub mod fever_care;
pub mod fever_management_home;
pub mod first_aid;
pub mod food_poisoning_care;
pub mod food_safety;
pub mod foot_blister_care;
pub mod foot_care_basics;
pub mod foot_soak_wellness;
pub mod fresh_breath_habit;
pub mod gout_management;
pub mod gratitude_practice;
pub mod gum_bleeding_concern;
pub mod gut_health;
pub mod hair_fall_prevention;
pub mod hair_health;
pub mod hand_hygiene_wash;
pub mod hand_washing_hygiene;
pub mod hearing_health;
pub mod heart_health;
pub mod heatstroke_prevention;
pub mod heatstroke_response;
pub mod hemorrhoid_care;
pub mod hiccup_relief;
pub mod home_first_aid_kit;
pub mod home_medical_alert_senior;
pub mod home_medicine_kit;
pub mod hydration_rules;
pub mod hydration_strategy;
pub mod hypertension_lifestyle;
pub mod hypertension_management;
pub mod hypoglycemia_response;
pub mod immune_health;
pub mod immune_support_lifestyle;
pub mod infant_care_basics;
pub mod infant_feeding_breast;
pub mod infant_health;
pub mod insect_bite_care;
pub mod interdental_floss_tips;
pub mod joint_health;
pub mod kidney_health;
pub mod knee_care;
pub mod leg_cramp_relief;
pub mod liver_health;
pub mod loneliness_connection;
pub mod lumbar_spine_protection;
pub mod memory_brain_training;
pub mod men_health;
pub mod menopause_health;
pub mod mental_health;
pub mod mental_wellness;
pub mod migraine_management;
pub mod mindful_eating;
pub mod mindfulness_basics;
pub mod minor_cut_disinfection;
pub mod mobility_aid_walker;
pub mod mood_diary;
pub mod motion_sickness_prevent;
pub mod motion_sickness_relief;
pub mod muscle_stretch_routine;
pub mod nail_trim_care;
pub mod nasal_congestion_relief;
pub mod neck_shoulder_relief;
pub mod neck_shoulder_stretch;
pub mod nightly_oral_care;
pub mod nocturia_frequent_urination;
pub mod nosebleed_management;
pub mod nutrition;
pub mod nutrition_basics;
pub mod occupational_health;
pub mod office_ergonomics;
pub mod oral_health;
pub mod oral_ulcer_care;
pub mod osteoporosis_walking_safe;
pub mod outdoor_time_eye_health;
pub mod postpartum_health;
pub mod postpartum_mother_care;
pub mod postpartum_recovery;
pub mod posture_rules;
pub mod pregnancy_prenatal;
pub mod premenstrual_syndrome_care;
pub mod prenatal_health;
pub mod procrastination_manage;
pub mod progressive_relaxation;
pub mod prolonged_sitting_safety;
pub mod protein_intake_estimation;
pub mod read_dim_light_avoid;
pub mod respiratory_health;
pub mod salt_control_daily;
pub mod scalds_burns;
pub mod screen_eye_distance_rule;
pub mod screen_eye_strain_care;
pub mod seasonal_allergy_relief;
pub mod seasonal_health;
pub mod seizure_response;
pub mod self_compassion;
pub mod sit_briefly_daily;
pub mod skin_care_rules;
pub mod skin_health;
pub mod skin_itch_soothing;
pub mod sleep;
pub mod sleep_apnea;
pub mod sleep_hygiene;
pub mod sleep_quality;
pub mod smoking_cessation;
pub mod snoring_sleep_basic;
pub mod sore_throat_relief;
pub mod sore_throat_soothe;
pub mod stress_management;
pub mod stress_relief_life;
pub mod sun_protection;
pub mod teen_health;
pub mod teenage_myopia_prevent;
pub mod thyroid_care;
pub mod tinnitus_ringing_relief;
pub mod toddler_health;
pub mod toddler_introducing_solids;
pub mod tooth_ache_care;
pub mod travel_health;
pub mod vaccination_rules;
pub mod varicose_vein_relief;
pub mod vegetarian_balanced_nutrition;
pub mod vertigo_balance_care;
pub mod vision_care;
pub mod weight_management;
pub mod women_health;

pub mod wound_dressing;
pub use acid_reflux_care::AcidRefluxCareRules;
pub use acne_skin_care::AcneSkinCareRules;
pub use addiction_recovery::AddictionRecoveryRules;
pub use allergy_management::AllergyManagementRules;
pub use anemia_nutrition::AnemiaNutritionRules;
pub use anger_management::AngerManagementRules;
pub use ankle_sprain_care::AnkleSprainCareRules;
pub use anxiety_management::AnxietyManagementRules;
pub use asthma_management_breathing::AsthmaManagementBreathingRules;
pub use baby_sleep_safety::BabySleepSafetyRules;
pub use balanced_breakfast::BalancedBreakfastRules;
pub use balanced_diet_plan::BalancedDietPlanRules;
pub use balanced_diet_plate::BalancedDietPlateRules;
pub use bedtime_ritual::BedtimeRitualRules;
pub use bleeding_control_bandage::BleedingControlBandageRules;
pub use blister_care::BlisterCareRules;
pub use blood_lipid_management::BloodLipidManagementRules;
pub use bone_health::BoneHealthRules;
pub use brain_health::BrainHealthRules;
pub use brushing_technique_bass::BrushingTechniqueBassRules;
pub use burn_scalding_aid::BurnScaldingAidRules;
pub use burnout_prevention::BurnoutPreventionRules;
pub use calcium_bone_health::CalciumBoneHealthRules;
pub use callus_corn_foot::CallusCornFootRules;
pub use cancer_prevention::CancerPreventionRules;
pub use caregiver_stress_manage::CaregiverStressManageRules;
pub use chapped_lip_care::ChappedLipCareRules;
pub use child_vaccination_schedule::ChildVaccinationScheduleRules;
pub use children_health::ChildrenHealthRules;
pub use choking_airway_relief::ChokingAirwayReliefRules;
pub use choking_relief::ChokingReliefRules;
pub use cholesterol_exercise_plan::CholesterolExercisePlanRules;
pub use chronic_disease::ChronicDiseaseRules;
pub use cold_hands_feet_warm::ColdHandsFeetWarmRules;
pub use common_cold_care::CommonColdCareRules;
pub use constipation_relief::ConstipationReliefRules;
pub use correct_sitting_posture::CorrectSittingPostureRules;
pub use cpr_and_emergency::CprEmergencyRules;
pub use cut_wound_care::CutWoundCareRules;
pub use daily_hydration::DailyHydrationRules;
pub use dementia_care_basic::DementiaCareBasicRules;
pub use dental_health::DentalHealthRules;
pub use denture_care_cleaning::DentureCareCleaningRules;
pub use depression_awareness::DepressionAwarenessRules;
pub use depression_support::DepressionSupportRules;
pub use detox_rules::DetoxRulesRules;
pub use diabetes_exercise_safety::DiabetesExerciseSafetyRules;
pub use diabetes_management::DiabetesManagementRules;
pub use dietary_fiber_vegetable::DietaryFiberVegetableRules;
pub use digestive_health::DigestiveHealthRules;
pub use digital_eye_rest::DigitalEyeRestRules;
pub use drowning_rescue::DrowningRescueRules;
pub use dry_cough_soothes::DryCoughSoothesRules;
pub use dry_cracked_heel::DryCrackedHeelCareRules;
pub use ear_ache_care::EarAcheCareRules;
pub use early_morning_routine::EarlyMorningRoutineRules;
pub use earthquake_safety::EarthquakeSafetyRules;
pub use elder_fall_prevention::ElderFallPreventionRules;
pub use elder_medication_remind::ElderMedicationRemindRules;
pub use elderly_fall_prevention::ElderlyFallPreventionRules;
pub use elderly_health::ElderlyHealthRules;
pub use emotional_regulation::EmotionalRegulationRules;
pub use ergonomics::ErgonomicsRules;
pub use exercise::ExerciseRules;
pub use eye_dryness_comfort::EyeDrynessComfortRules;
pub use eye_health::EyeHealthRules;
pub use eye_health_general::EyeHealthGeneralRules;
pub use eye_strain_rules::EyeStrainRulesRules;
pub use fever_care::FeverCareRules;
pub use fever_management_home::FeverManagementHomeRules;
pub use first_aid::FirstAidRules;
pub use food_poisoning_care::FoodPoisoningCareRules;
pub use food_safety::FoodSafetyRules;
pub use foot_blister_care::FootBlisterCareRules;
pub use foot_care_basics::FootCareBasicsRules;
pub use foot_soak_wellness::FootSoakWellnessRules;
pub use fresh_breath_habit::FreshBreathHabitRules;
pub use gout_management::GoutManagementRules;
pub use gratitude_practice::GratitudePracticeRules;
pub use gum_bleeding_concern::GumBleedingConcernRules;
pub use gut_health::GutHealthRules;
pub use hair_fall_prevention::HairFallPreventionRules;
pub use hair_health::HairHealthRules;
pub use hand_hygiene_wash::HandHygieneWashRules;
pub use hand_washing_hygiene::HandWashingHygieneRules;
pub use hearing_health::HearingHealthRules;
pub use heart_health::HeartHealthRules;
pub use heatstroke_prevention::HeatstrokePreventionRules;
pub use heatstroke_response::HeatstrokeResponseRules;
pub use hemorrhoid_care::HemorrhoidCareRules;
pub use hiccup_relief::HiccupReliefRules;
pub use home_first_aid_kit::HomeFirstAidKitRules;
pub use home_medical_alert_senior::HomeMedicalAlertSeniorRules;
pub use home_medicine_kit::HomeMedicineKitRules;
pub use hydration_rules::HydrationRulesRules;
pub use hydration_strategy::HydrationStrategyRules;
pub use hypertension_lifestyle::HypertensionLifestyleRules;
pub use hypertension_management::HypertensionManagementRules;
pub use hypoglycemia_response::HypoglycemiaResponseRules;
pub use immune_health::ImmuneHealthRules;
pub use immune_support_lifestyle::ImmuneSupportLifestyleRules;
pub use infant_care_basics::InfantCareBasicsRules;
pub use infant_feeding_breast::InfantFeedingBreastRules;
pub use infant_health::InfantHealthRules;
pub use insect_bite_care::InsectBiteCareRules;
pub use interdental_floss_tips::InterdentalFlossTipsRules;
pub use joint_health::JointHealthRules;
pub use kidney_health::KidneyHealthRules;
pub use knee_care::KneeCareRules;
pub use leg_cramp_relief::LegCrampReliefRules;
pub use liver_health::LiverHealthRules;
pub use loneliness_connection::LonelinessConnectionRules;
pub use lumbar_spine_protection::LumbarSpineProtectionRules;
pub use memory_brain_training::MemoryBrainTrainingRules;
pub use men_health::MenHealthRules;
pub use menopause_health::MenopauseHealthRules;
pub use mental_health::MentalHealthRules;
pub use mental_wellness::MentalWellnessRules;
pub use migraine_management::MigraineManagementRules;
pub use mindful_eating::MindfulEatingRules;
pub use mindfulness_basics::MindfulnessBasicsRules;
pub use minor_cut_disinfection::MinorCutDisinfectionRules;
pub use mobility_aid_walker::MobilityAidWalkerRules;
pub use mood_diary::MoodDiaryRules;
pub use motion_sickness_prevent::MotionSicknessPreventRules;
pub use motion_sickness_relief::MotionSicknessReliefRules;
pub use muscle_stretch_routine::MuscleStretchRoutineRules;
pub use nail_trim_care::NailTrimCareRules;
pub use nasal_congestion_relief::NasalCongestionReliefRules;
pub use neck_shoulder_relief::NeckShoulderReliefRules;
pub use neck_shoulder_stretch::NeckShoulderStretchRules;
pub use nightly_oral_care::NightlyOralCareRules;
pub use nocturia_frequent_urination::NocturiaFrequentUrinationRules;
pub use nosebleed_management::NosebleedManagementRules;
pub use nutrition::NutritionRules;
pub use nutrition_basics::NutritionBasicsRules;
pub use occupational_health::OccupationalHealthRules;
pub use office_ergonomics::OfficeErgonomicsRules;
pub use oral_health::OralHealthRules;
pub use oral_ulcer_care::OralUlcerCareRules;
pub use osteoporosis_walking_safe::OsteoporosisWalkingSafeRules;
pub use outdoor_time_eye_health::OutdoorTimeEyeHealthRules;
pub use postpartum_health::PostpartumHealthRules;
pub use postpartum_mother_care::PostpartumMotherCareRules;
pub use postpartum_recovery::PostpartumRecoveryRules;
pub use posture_rules::PostureRulesRules;
pub use pregnancy_prenatal::PregnancyPrenatalRules;
pub use premenstrual_syndrome_care::PremenstrualSyndromeCareRules;
pub use prenatal_health::PrenatalHealthRules;
pub use procrastination_manage::ProcrastinationManageRules;
pub use progressive_relaxation::ProgressiveRelaxationRules;
pub use prolonged_sitting_safety::ProlongedSittingSafetyRules;
pub use protein_intake_estimation::ProteinIntakeEstimationRules;
pub use read_dim_light_avoid::ReadDimLightAvoidRules;
pub use respiratory_health::RespiratoryHealthRules;
pub use salt_control_daily::SaltControlDailyRules;
pub use scalds_burns::ScaldBurnCareRules;
pub use screen_eye_distance_rule::ScreenEyeDistanceRuleRules;
pub use screen_eye_strain_care::ScreenEyeStrainCareRules;
pub use seasonal_allergy_relief::SeasonalAllergyReliefRules;
pub use seasonal_health::SeasonalHealthRules;
pub use seizure_response::SeizureResponseRules;
pub use self_compassion::SelfCompassionRules;
pub use sit_briefly_daily::SitBreaksDailyRules;
pub use skin_care_rules::SkinCareRulesRules;
pub use skin_health::SkinHealthRules;
pub use skin_itch_soothing::SkinItchSoothingRules;
pub use sleep::SleepRules;
pub use sleep_apnea::SleepApneaRules;
pub use sleep_hygiene::SleepHygieneRules;
pub use sleep_quality::SleepQualityRules;
pub use smoking_cessation::SmokingCessationRules;
pub use snoring_sleep_basic::SnoringSleepBasicRules;
pub use sore_throat_relief::SoreThroatReliefRules;
pub use sore_throat_soothe::SoreThroatSootheRules;
pub use stress_management::StressManagementRules;
pub use stress_relief_life::StressReliefRules;
pub use sun_protection::SunProtectionRules;
pub use teen_health::TeenHealthRules;
pub use teenage_myopia_prevent::TeenageMyopiaPreventRules;
pub use thyroid_care::ThyroidCareRules;
pub use tinnitus_ringing_relief::TinnitusRingingReliefRules;
pub use toddler_health::ToddlerHealthRules;
pub use toddler_introducing_solids::ToddlerIntroducingSolidsRules;
pub use tooth_ache_care::ToothAcheCareRules;
pub use travel_health::TravelHealthRules;
pub use vaccination_rules::VaccinationRulesRules;
pub use varicose_vein_relief::VaricoseVeinReliefRules;
pub use vegetarian_balanced_nutrition::VegetarianBalancedNutritionRules;
pub use vertigo_balance_care::VertigoBalanceCareRules;
pub use vision_care::VisionCareRules;
pub use weight_management::WeightManagementRules;
pub use women_health::WomenHealthRules;
pub use wound_dressing::WoundDressingRules;

pub fn all_rules() -> Vec<(
    &'static str,
    crate::rules::core::RuleMetadata,
    crate::rules::core::RuleCategory,
    String,
)> {
    use crate::rules::core::Rule;
    let mut rules = Vec::new();
    {
        let r = ChappedLipCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NailTrimCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HairFallPreventionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ColdHandsFeetWarmRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EyeDrynessComfortRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DryCrackedHeelCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FootSoakWellnessRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DentureCareCleaningRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NocturiaFrequentUrinationRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SnoringSleepBasicRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CallusCornFootRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AcneSkinCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HemorrhoidCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = VaricoseVeinReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HypoglycemiaResponseRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DryCoughSoothesRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MinorCutDisinfectionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SoreThroatSootheRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MotionSicknessPreventRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FeverManagementHomeRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ReadDimLightAvoidRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OutdoorTimeEyeHealthRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ScreenEyeDistanceRuleRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CorrectSittingPostureRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = TeenageMyopiaPreventRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NeckShoulderReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = OsteoporosisWalkingSafeRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CholesterolExercisePlanRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AsthmaManagementBreathingRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DiabetesExerciseSafetyRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FreshBreathHabitRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = GumBleedingConcernRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InterdentalFlossTipsRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NightlyOralCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BrushingTechniqueBassRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HomeMedicalAlertSeniorRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DementiaCareBasicRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MobilityAidWalkerRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElderMedicationRemindRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CaregiverStressManageRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ToddlerIntroducingSolidsRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChildVaccinationScheduleRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BabySleepSafetyRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = InfantFeedingBreastRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = PostpartumMotherCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DietaryFiberVegetableRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SaltControlDailyRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = CalciumBoneHealthRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ProteinIntakeEstimationRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BalancedDietPlateRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HomeMedicineKitRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ChokingReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = WoundDressingRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HeatstrokeResponseRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BurnScaldingAidRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DailyHydrationRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BalancedBreakfastRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SitBreaksDailyRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = DigitalEyeRestRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = LumbarSpineProtectionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = FootCareBasicsRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ElderlyFallPreventionRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = HandHygieneWashRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MemoryBrainTrainingRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NeckShoulderStretchRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ProgressiveRelaxationRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = EarlyMorningRoutineRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MoodDiaryRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = MindfulEatingRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = BedtimeRitualRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = NosebleedManagementRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = SeasonalAllergyReliefRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = AcidRefluxCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
    {
        let r = ToothAcheCareRules::new();
        rules.push(("health", r.metadata().clone(), r.category(), r.explain()));
    }
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
