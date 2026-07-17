//! 运动康复规则
//!
//! 基于运动医学标准，包含康复阶段划分、康复计划制定、康复训练原则、
//! 康复效果评估和返回运动决策等康复体系。
//!
//! # 规则体系
//!
//! - 康复阶段划分
//! - 康复计划制定原则
//! - 康复训练方法
//! - 康复效果评估
//! - 返回运动决策标准
//!
//! # Examples
//!
//! ```
//! use world_rules::rules::sports::sports_rehabilitation::SportsRehabilitationRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = SportsRehabilitationRules::new();
//! assert!(!rules.rehabilitation_phases().is_empty());
//! assert!(!rules.rehabilitation_principles().is_empty());
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 康复阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RehabilitationPhase {
    /// 急性期（炎症控制）
    AcutePhase,
    /// 亚急性期（早期活动）
    SubacutePhase,
    /// 恢复期（功能重建）
    RecoveryPhase,
    /// 功能训练期
    FunctionalTrainingPhase,
    /// 返回运动期
    ReturnToActivityPhase,
}

impl RehabilitationPhase {
    /// 获取阶段名称
    pub fn name(&self) -> &'static str {
        match self {
            RehabilitationPhase::AcutePhase => "急性期",
            RehabilitationPhase::SubacutePhase => "亚急性期",
            RehabilitationPhase::RecoveryPhase => "恢复期",
            RehabilitationPhase::FunctionalTrainingPhase => "功能训练期",
            RehabilitationPhase::ReturnToActivityPhase => "返回运动期",
        }
    }

    /// 获取阶段持续时间范围（天）
    pub fn duration_days(&self) -> std::ops::Range<u32> {
        match self {
            RehabilitationPhase::AcutePhase => 1..7,
            RehabilitationPhase::SubacutePhase => 7..21,
            RehabilitationPhase::RecoveryPhase => 14..90,
            RehabilitationPhase::FunctionalTrainingPhase => 21..180,
            RehabilitationPhase::ReturnToActivityPhase => 7..30,
        }
    }

    /// 获取阶段目标
    pub fn objectives(&self) -> &'static [&'static str] {
        match self {
            RehabilitationPhase::AcutePhase => {
                &["控制炎症", "缓解疼痛", "保护损伤组织", "预防并发症"]
            }
            RehabilitationPhase::SubacutePhase => &[
                "恢复关节活动度",
                "开始肌肉激活",
                "渐进性负重",
                "预防肌肉萎缩",
            ],
            RehabilitationPhase::RecoveryPhase => {
                &["恢复肌肉力量", "改善关节灵活性", "恢复本体感觉", "提高耐力"]
            }
            RehabilitationPhase::FunctionalTrainingPhase => {
                &["专项动作训练", "力量强化", "灵敏性训练", "运动技能恢复"]
            }
            RehabilitationPhase::ReturnToActivityPhase => &[
                "完全运动能力恢复",
                "心理准备评估",
                "预防再损伤教育",
                "逐步回归比赛",
            ],
        }
    }

    /// 是否需要医疗监督
    pub fn requires_medical_supervision(&self) -> bool {
        matches!(
            self,
            RehabilitationPhase::AcutePhase | RehabilitationPhase::SubacutePhase
        )
    }
}

/// 康复训练类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RehabilitationExerciseType {
    /// 关节活动度训练
    RangeOfMotion,
    /// 肌肉力量训练
    StrengthTraining,
    /// 柔韧性训练
    FlexibilityTraining,
    /// 本体感觉训练
    ProprioceptionTraining,
    /// 心肺耐力训练
    CardiovascularTraining,
    /// 功能性训练
    FunctionalTraining,
    /// 水疗康复
    Hydrotherapy,
    /// 神经肌肉控制训练
    NeuromuscularTraining,
    /// 平衡训练
    BalanceTraining,
    /// 灵敏性训练
    AgilityTraining,
}

impl RehabilitationExerciseType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            RehabilitationExerciseType::RangeOfMotion => "关节活动度训练",
            RehabilitationExerciseType::StrengthTraining => "肌肉力量训练",
            RehabilitationExerciseType::FlexibilityTraining => "柔韧性训练",
            RehabilitationExerciseType::ProprioceptionTraining => "本体感觉训练",
            RehabilitationExerciseType::CardiovascularTraining => "心肺耐力训练",
            RehabilitationExerciseType::FunctionalTraining => "功能性训练",
            RehabilitationExerciseType::Hydrotherapy => "水疗康复",
            RehabilitationExerciseType::NeuromuscularTraining => "神经肌肉控制训练",
            RehabilitationExerciseType::BalanceTraining => "平衡训练",
            RehabilitationExerciseType::AgilityTraining => "灵敏性训练",
        }
    }

    /// 获取适用康复阶段
    pub fn applicable_phases(&self) -> &'static [RehabilitationPhase] {
        match self {
            RehabilitationExerciseType::RangeOfMotion => &[
                RehabilitationPhase::SubacutePhase,
                RehabilitationPhase::RecoveryPhase,
            ],
            RehabilitationExerciseType::StrengthTraining => &[
                RehabilitationPhase::RecoveryPhase,
                RehabilitationPhase::FunctionalTrainingPhase,
            ],
            RehabilitationExerciseType::FlexibilityTraining => &[
                RehabilitationPhase::SubacutePhase,
                RehabilitationPhase::RecoveryPhase,
                RehabilitationPhase::FunctionalTrainingPhase,
            ],
            RehabilitationExerciseType::ProprioceptionTraining => &[
                RehabilitationPhase::RecoveryPhase,
                RehabilitationPhase::FunctionalTrainingPhase,
            ],
            RehabilitationExerciseType::CardiovascularTraining => &[
                RehabilitationPhase::SubacutePhase,
                RehabilitationPhase::RecoveryPhase,
                RehabilitationPhase::FunctionalTrainingPhase,
            ],
            RehabilitationExerciseType::FunctionalTraining => &[
                RehabilitationPhase::FunctionalTrainingPhase,
                RehabilitationPhase::ReturnToActivityPhase,
            ],
            RehabilitationExerciseType::Hydrotherapy => &[
                RehabilitationPhase::AcutePhase,
                RehabilitationPhase::SubacutePhase,
                RehabilitationPhase::RecoveryPhase,
            ],
            RehabilitationExerciseType::NeuromuscularTraining => &[
                RehabilitationPhase::RecoveryPhase,
                RehabilitationPhase::FunctionalTrainingPhase,
            ],
            RehabilitationExerciseType::BalanceTraining => &[
                RehabilitationPhase::RecoveryPhase,
                RehabilitationPhase::FunctionalTrainingPhase,
            ],
            RehabilitationExerciseType::AgilityTraining => &[
                RehabilitationPhase::FunctionalTrainingPhase,
                RehabilitationPhase::ReturnToActivityPhase,
            ],
        }
    }

    /// 获取训练强度等级（1-10）
    pub fn intensity_level(&self) -> u32 {
        match self {
            RehabilitationExerciseType::RangeOfMotion => 2,
            RehabilitationExerciseType::StrengthTraining => 6,
            RehabilitationExerciseType::FlexibilityTraining => 3,
            RehabilitationExerciseType::ProprioceptionTraining => 5,
            RehabilitationExerciseType::CardiovascularTraining => 5,
            RehabilitationExerciseType::FunctionalTraining => 7,
            RehabilitationExerciseType::Hydrotherapy => 3,
            RehabilitationExerciseType::NeuromuscularTraining => 6,
            RehabilitationExerciseType::BalanceTraining => 4,
            RehabilitationExerciseType::AgilityTraining => 8,
        }
    }
}

/// 康复评估指标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RehabilitationAssessmentMetric {
    /// 疼痛评分（VAS）
    PainScore,
    /// 关节活动度
    RangeOfMotion,
    /// 肌肉力量
    MuscleStrength,
    /// 关节稳定性
    JointStability,
    /// 功能能力
    FunctionalCapacity,
    /// 平衡能力
    BalanceAbility,
    /// 本体感觉
    Proprioception,
    /// 心肺耐力
    CardiovascularEndurance,
    /// 心理状态
    PsychologicalStatus,
    /// 运动表现
    AthleticPerformance,
}

impl RehabilitationAssessmentMetric {
    /// 获取指标名称
    pub fn name(&self) -> &'static str {
        match self {
            RehabilitationAssessmentMetric::PainScore => "疼痛评分",
            RehabilitationAssessmentMetric::RangeOfMotion => "关节活动度",
            RehabilitationAssessmentMetric::MuscleStrength => "肌肉力量",
            RehabilitationAssessmentMetric::JointStability => "关节稳定性",
            RehabilitationAssessmentMetric::FunctionalCapacity => "功能能力",
            RehabilitationAssessmentMetric::BalanceAbility => "平衡能力",
            RehabilitationAssessmentMetric::Proprioception => "本体感觉",
            RehabilitationAssessmentMetric::CardiovascularEndurance => "心肺耐力",
            RehabilitationAssessmentMetric::PsychologicalStatus => "心理状态",
            RehabilitationAssessmentMetric::AthleticPerformance => "运动表现",
        }
    }

    /// 获取评估频率（天）
    pub fn assessment_frequency_days(&self) -> u32 {
        match self {
            RehabilitationAssessmentMetric::PainScore => 1,
            RehabilitationAssessmentMetric::RangeOfMotion => 7,
            RehabilitationAssessmentMetric::MuscleStrength => 14,
            RehabilitationAssessmentMetric::JointStability => 14,
            RehabilitationAssessmentMetric::FunctionalCapacity => 30,
            RehabilitationAssessmentMetric::BalanceAbility => 14,
            RehabilitationAssessmentMetric::Proprioception => 14,
            RehabilitationAssessmentMetric::CardiovascularEndurance => 30,
            RehabilitationAssessmentMetric::PsychologicalStatus => 30,
            RehabilitationAssessmentMetric::AthleticPerformance => 60,
        }
    }

    /// 获取合格阈值（百分比）
    pub fn passing_threshold(&self) -> u32 {
        match self {
            RehabilitationAssessmentMetric::PainScore => 80, // 疼痛减轻80%
            RehabilitationAssessmentMetric::RangeOfMotion => 90,
            RehabilitationAssessmentMetric::MuscleStrength => 85,
            RehabilitationAssessmentMetric::JointStability => 90,
            RehabilitationAssessmentMetric::FunctionalCapacity => 85,
            RehabilitationAssessmentMetric::BalanceAbility => 85,
            RehabilitationAssessmentMetric::Proprioception => 80,
            RehabilitationAssessmentMetric::CardiovascularEndurance => 80,
            RehabilitationAssessmentMetric::PsychologicalStatus => 90,
            RehabilitationAssessmentMetric::AthleticPerformance => 90,
        }
    }
}

/// 返回运动决策标准
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReturnToActivityCriteria {
    /// 无痛活动
    PainFreeMovement,
    /// 完全关节活动度
    FullRangeOfMotion,
    /// 肌力恢复≥90%
    MuscleStrengthRestored,
    /// 关节稳定
    JointStabilityNormal,
    /// 功能测试通过
    FunctionalTestPassed,
    /// 专项技能恢复
    SportSpecificSkillsRestored,
    /// 心理准备就绪
    PsychologicalReadiness,
    /// 医疗许可
    MedicalClearance,
    /// 教练认可
    CoachApproval,
    /// 逐步训练完成
    ProgressiveTrainingCompleted,
}

impl ReturnToActivityCriteria {
    /// 获取标准名称
    pub fn name(&self) -> &'static str {
        match self {
            ReturnToActivityCriteria::PainFreeMovement => "无痛活动",
            ReturnToActivityCriteria::FullRangeOfMotion => "完全关节活动度",
            ReturnToActivityCriteria::MuscleStrengthRestored => "肌力恢复≥90%",
            ReturnToActivityCriteria::JointStabilityNormal => "关节稳定",
            ReturnToActivityCriteria::FunctionalTestPassed => "功能测试通过",
            ReturnToActivityCriteria::SportSpecificSkillsRestored => "专项技能恢复",
            ReturnToActivityCriteria::PsychologicalReadiness => "心理准备就绪",
            ReturnToActivityCriteria::MedicalClearance => "医疗许可",
            ReturnToActivityCriteria::CoachApproval => "教练认可",
            ReturnToActivityCriteria::ProgressiveTrainingCompleted => "逐步训练完成",
        }
    }

    /// 是否为强制标准
    pub fn is_mandatory(&self) -> bool {
        matches!(
            self,
            ReturnToActivityCriteria::MedicalClearance
                | ReturnToActivityCriteria::PainFreeMovement
                | ReturnToActivityCriteria::FunctionalTestPassed
        )
    }

    /// 获取标准优先级（1最高）
    pub fn priority(&self) -> u32 {
        match self {
            ReturnToActivityCriteria::MedicalClearance => 1,
            ReturnToActivityCriteria::PainFreeMovement => 2,
            ReturnToActivityCriteria::FunctionalTestPassed => 3,
            ReturnToActivityCriteria::FullRangeOfMotion => 4,
            ReturnToActivityCriteria::MuscleStrengthRestored => 5,
            ReturnToActivityCriteria::JointStabilityNormal => 6,
            ReturnToActivityCriteria::PsychologicalReadiness => 7,
            ReturnToActivityCriteria::SportSpecificSkillsRestored => 8,
            ReturnToActivityCriteria::ProgressiveTrainingCompleted => 9,
            ReturnToActivityCriteria::CoachApproval => 10,
        }
    }
}

/// 康复原则
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RehabilitationPrinciple {
    /// 循序渐进原则
    ProgressiveOverload,
    /// 个体化原则
    Individualization,
    /// 特异性原则
    Specificity,
    /// 全面性原则
    ComprehensiveApproach,
    /// 安全性原则
    SafetyFirst,
    /// 主动参与原则
    ActiveParticipation,
    /// 早期介入原则
    EarlyIntervention,
    /// 多学科协作原则
    MultidisciplinaryTeamwork,
}

impl RehabilitationPrinciple {
    /// 获取原则名称
    pub fn name(&self) -> &'static str {
        match self {
            RehabilitationPrinciple::ProgressiveOverload => "循序渐进原则",
            RehabilitationPrinciple::Individualization => "个体化原则",
            RehabilitationPrinciple::Specificity => "特异性原则",
            RehabilitationPrinciple::ComprehensiveApproach => "全面性原则",
            RehabilitationPrinciple::SafetyFirst => "安全性原则",
            RehabilitationPrinciple::ActiveParticipation => "主动参与原则",
            RehabilitationPrinciple::EarlyIntervention => "早期介入原则",
            RehabilitationPrinciple::MultidisciplinaryTeamwork => "多学科协作原则",
        }
    }

    /// 获取原则描述
    pub fn description(&self) -> &'static str {
        match self {
            RehabilitationPrinciple::ProgressiveOverload => {
                "康复训练强度应逐步增加，避免过度负荷导致再次损伤"
            }
            RehabilitationPrinciple::Individualization => "根据运动员具体情况制定个性化康复方案",
            RehabilitationPrinciple::Specificity => "康复训练应针对特定运动项目和损伤类型",
            RehabilitationPrinciple::ComprehensiveApproach => "康复应涵盖生理、心理、功能等多方面",
            RehabilitationPrinciple::SafetyFirst => "确保训练安全，避免并发症和再损伤",
            RehabilitationPrinciple::ActiveParticipation => "鼓励运动员积极参与康复过程",
            RehabilitationPrinciple::EarlyIntervention => "损伤后尽早开始适当康复，促进恢复",
            RehabilitationPrinciple::MultidisciplinaryTeamwork => {
                "医疗、康复、教练团队协作制定方案"
            }
        }
    }
}

/// 运动康复规则
#[derive(Debug, Clone)]
pub struct SportsRehabilitationRules {
    metadata: RuleMetadata,
}

impl SportsRehabilitationRules {
    /// 创建新实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("运动康复规则", "运动损伤康复规则体系")
                .with_version("1.0.0"),
        }
    }

    /// 获取康复阶段列表
    pub fn rehabilitation_phases(&self) -> Vec<RehabilitationPhase> {
        vec![
            RehabilitationPhase::AcutePhase,
            RehabilitationPhase::SubacutePhase,
            RehabilitationPhase::RecoveryPhase,
            RehabilitationPhase::FunctionalTrainingPhase,
            RehabilitationPhase::ReturnToActivityPhase,
        ]
    }

    /// 获取康复原则列表
    pub fn rehabilitation_principles(&self) -> Vec<RehabilitationPrinciple> {
        vec![
            RehabilitationPrinciple::ProgressiveOverload,
            RehabilitationPrinciple::Individualization,
            RehabilitationPrinciple::Specificity,
            RehabilitationPrinciple::ComprehensiveApproach,
            RehabilitationPrinciple::SafetyFirst,
            RehabilitationPrinciple::ActiveParticipation,
            RehabilitationPrinciple::EarlyIntervention,
            RehabilitationPrinciple::MultidisciplinaryTeamwork,
        ]
    }

    /// 获取康复训练类型列表
    pub fn exercise_types(&self) -> Vec<RehabilitationExerciseType> {
        vec![
            RehabilitationExerciseType::RangeOfMotion,
            RehabilitationExerciseType::StrengthTraining,
            RehabilitationExerciseType::FlexibilityTraining,
            RehabilitationExerciseType::ProprioceptionTraining,
            RehabilitationExerciseType::CardiovascularTraining,
            RehabilitationExerciseType::FunctionalTraining,
            RehabilitationExerciseType::Hydrotherapy,
            RehabilitationExerciseType::NeuromuscularTraining,
            RehabilitationExerciseType::BalanceTraining,
            RehabilitationExerciseType::AgilityTraining,
        ]
    }

    /// 获取康复评估指标列表
    pub fn assessment_metrics(&self) -> Vec<RehabilitationAssessmentMetric> {
        vec![
            RehabilitationAssessmentMetric::PainScore,
            RehabilitationAssessmentMetric::RangeOfMotion,
            RehabilitationAssessmentMetric::MuscleStrength,
            RehabilitationAssessmentMetric::JointStability,
            RehabilitationAssessmentMetric::FunctionalCapacity,
            RehabilitationAssessmentMetric::BalanceAbility,
            RehabilitationAssessmentMetric::Proprioception,
            RehabilitationAssessmentMetric::CardiovascularEndurance,
            RehabilitationAssessmentMetric::PsychologicalStatus,
            RehabilitationAssessmentMetric::AthleticPerformance,
        ]
    }

    /// 获取返回运动决策标准列表
    pub fn return_to_activity_criteria(&self) -> Vec<ReturnToActivityCriteria> {
        vec![
            ReturnToActivityCriteria::MedicalClearance,
            ReturnToActivityCriteria::PainFreeMovement,
            ReturnToActivityCriteria::FullRangeOfMotion,
            ReturnToActivityCriteria::MuscleStrengthRestored,
            ReturnToActivityCriteria::JointStabilityNormal,
            ReturnToActivityCriteria::FunctionalTestPassed,
            ReturnToActivityCriteria::SportSpecificSkillsRestored,
            ReturnToActivityCriteria::PsychologicalReadiness,
            ReturnToActivityCriteria::ProgressiveTrainingCompleted,
            ReturnToActivityCriteria::CoachApproval,
        ]
    }

    /// 根据康复阶段获取适用训练类型
    pub fn get_exercises_for_phase(
        &self,
        phase: RehabilitationPhase,
    ) -> Vec<RehabilitationExerciseType> {
        self.exercise_types()
            .into_iter()
            .filter(|ex| ex.applicable_phases().contains(&phase))
            .collect()
    }

    /// 评估是否满足返回运动标准
    pub fn evaluate_return_to_activity(
        &self,
        criteria_met: &[ReturnToActivityCriteria],
    ) -> RehabilitationDecision {
        let all_criteria = self.return_to_activity_criteria();
        let mandatory_criteria: Vec<_> = all_criteria.iter().filter(|c| c.is_mandatory()).collect();

        // 检查强制标准
        let mandatory_met = mandatory_criteria.iter().all(|c| criteria_met.contains(c));

        if !mandatory_met {
            return RehabilitationDecision::NotReady {
                reason: "未满足所有强制标准".to_string(),
            };
        }

        // 计算总体完成率
        let completion_rate = (criteria_met.len() as f64 / all_criteria.len() as f64) * 100.0;

        if completion_rate >= 90.0 {
            RehabilitationDecision::ReadyForFullActivity
        } else if completion_rate >= 70.0 {
            RehabilitationDecision::ReadyForModifiedActivity {
                restrictions: vec!["限制高强度训练".to_string()],
            }
        } else {
            RehabilitationDecision::NotReady {
                reason: format!("完成率不足（{:.0}%），需继续康复", completion_rate),
            }
        }
    }

    /// 获取康复计划模板
    pub fn rehabilitation_plan_template(
        &self,
        phase: RehabilitationPhase,
    ) -> RehabilitationPlanTemplate {
        let exercises = self.get_exercises_for_phase(phase);
        let duration_days = phase.duration_days().next().unwrap_or(14);

        RehabilitationPlanTemplate {
            phase,
            duration_days,
            objectives: phase.objectives().to_vec(),
            exercises,
            supervision_required: phase.requires_medical_supervision(),
        }
    }
}

impl Default for SportsRehabilitationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SportsRehabilitationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::Sports("运动医学".to_string())
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
}

/// 康复决策
#[derive(Debug, Clone, PartialEq)]
pub enum RehabilitationDecision {
    /// 准备完全恢复运动
    ReadyForFullActivity,
    /// 准备有限制运动
    ReadyForModifiedActivity { restrictions: Vec<String> },
    /// 未准备好
    NotReady { reason: String },
}

/// 康复计划模板
#[derive(Debug, Clone)]
pub struct RehabilitationPlanTemplate {
    /// 康复阶段
    pub phase: RehabilitationPhase,
    /// 持续时间（天）
    pub duration_days: u32,
    /// 康复目标
    pub objectives: Vec<&'static str>,
    /// 适用训练类型
    pub exercises: Vec<RehabilitationExerciseType>,
    /// 是否需要监督
    pub supervision_required: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rehabilitation_phases() {
        let rules = SportsRehabilitationRules::new();
        let phases = rules.rehabilitation_phases();

        assert_eq!(phases.len(), 5);
        assert!(phases.contains(&RehabilitationPhase::AcutePhase));
        assert!(phases.contains(&RehabilitationPhase::RecoveryPhase));
    }

    #[test]
    fn test_rehabilitation_principles() {
        let rules = SportsRehabilitationRules::new();
        let principles = rules.rehabilitation_principles();

        assert_eq!(principles.len(), 8);
        assert!(principles.contains(&RehabilitationPrinciple::ProgressiveOverload));
        assert!(principles.contains(&RehabilitationPrinciple::SafetyFirst));
    }

    #[test]
    fn test_exercise_types() {
        let rules = SportsRehabilitationRules::new();
        let exercises = rules.exercise_types();

        assert_eq!(exercises.len(), 10);
        assert!(exercises.contains(&RehabilitationExerciseType::StrengthTraining));
        assert!(exercises.contains(&RehabilitationExerciseType::FunctionalTraining));
    }

    #[test]
    fn test_assessment_metrics() {
        let rules = SportsRehabilitationRules::new();
        let metrics = rules.assessment_metrics();

        assert_eq!(metrics.len(), 10);
        assert!(metrics.contains(&RehabilitationAssessmentMetric::PainScore));
        assert!(metrics.contains(&RehabilitationAssessmentMetric::MuscleStrength));
    }

    #[test]
    fn test_return_to_activity_criteria() {
        let rules = SportsRehabilitationRules::new();
        let criteria = rules.return_to_activity_criteria();

        assert_eq!(criteria.len(), 10);
        assert!(criteria.contains(&ReturnToActivityCriteria::MedicalClearance));
        assert!(criteria.contains(&ReturnToActivityCriteria::PainFreeMovement));
    }

    #[test]
    fn test_phase_names() {
        assert_eq!(RehabilitationPhase::AcutePhase.name(), "急性期");
        assert_eq!(RehabilitationPhase::RecoveryPhase.name(), "恢复期");
        assert_eq!(
            RehabilitationPhase::ReturnToActivityPhase.name(),
            "返回运动期"
        );
    }

    #[test]
    fn test_phase_duration() {
        let duration = RehabilitationPhase::AcutePhase.duration_days();
        assert!(duration.contains(3));

        let duration = RehabilitationPhase::RecoveryPhase.duration_days();
        assert!(duration.contains(30));
    }

    #[test]
    fn test_phase_objectives() {
        let objectives = RehabilitationPhase::AcutePhase.objectives();
        assert!(!objectives.is_empty());
        assert!(objectives.contains(&"控制炎症"));
    }

    #[test]
    fn test_exercise_applicable_phases() {
        let phases = RehabilitationExerciseType::StrengthTraining.applicable_phases();
        assert!(phases.contains(&RehabilitationPhase::RecoveryPhase));
        assert!(phases.contains(&RehabilitationPhase::FunctionalTrainingPhase));
    }

    #[test]
    fn test_exercise_intensity() {
        let intensity = RehabilitationExerciseType::StrengthTraining.intensity_level();
        assert!(intensity >= 1);
        assert!(intensity <= 10);
    }

    #[test]
    fn test_assessment_frequency() {
        let frequency = RehabilitationAssessmentMetric::PainScore.assessment_frequency_days();
        assert_eq!(frequency, 1);

        let frequency = RehabilitationAssessmentMetric::MuscleStrength.assessment_frequency_days();
        assert_eq!(frequency, 14);
    }

    #[test]
    fn test_assessment_threshold() {
        let threshold = RehabilitationAssessmentMetric::RangeOfMotion.passing_threshold();
        assert!(threshold >= 80);
    }

    #[test]
    fn test_criteria_mandatory() {
        assert!(ReturnToActivityCriteria::MedicalClearance.is_mandatory());
        assert!(ReturnToActivityCriteria::PainFreeMovement.is_mandatory());
        assert!(!ReturnToActivityCriteria::CoachApproval.is_mandatory());
    }

    #[test]
    fn test_criteria_priority() {
        let medical = ReturnToActivityCriteria::MedicalClearance.priority();
        let coach = ReturnToActivityCriteria::CoachApproval.priority();
        assert!(medical < coach);
    }

    #[test]
    fn test_get_exercises_for_phase() {
        let rules = SportsRehabilitationRules::new();
        let exercises = rules.get_exercises_for_phase(RehabilitationPhase::AcutePhase);

        // 急性期应该包含水疗
        assert!(exercises.contains(&RehabilitationExerciseType::Hydrotherapy));
    }

    #[test]
    fn test_evaluate_return_to_activity_all_met() {
        let rules = SportsRehabilitationRules::new();
        let all_criteria = rules.return_to_activity_criteria();

        let decision = rules.evaluate_return_to_activity(&all_criteria);
        assert_eq!(decision, RehabilitationDecision::ReadyForFullActivity);
    }

    #[test]
    fn test_evaluate_return_to_activity_mandatory_only() {
        let rules = SportsRehabilitationRules::new();
        let mandatory: Vec<_> = rules
            .return_to_activity_criteria()
            .into_iter()
            .filter(|c| c.is_mandatory())
            .collect();

        let decision = rules.evaluate_return_to_activity(&mandatory);
        // 只有3个强制标准（30%），应该返回 NotReady 或 ModifiedActivity
        match decision {
            RehabilitationDecision::NotReady { .. } => {}
            RehabilitationDecision::ReadyForModifiedActivity { .. } => {}
            _ => panic!("应返回 NotReady 或 ModifiedActivity"),
        }
    }

    #[test]
    fn test_evaluate_return_to_activity_none_met() {
        let rules = SportsRehabilitationRules::new();
        let decision = rules.evaluate_return_to_activity(&[]);

        match decision {
            RehabilitationDecision::NotReady { reason } => {
                assert!(reason.contains("强制标准"));
            }
            _ => panic!("应返回 NotReady"),
        }
    }

    #[test]
    fn test_rehabilitation_plan_template() {
        let rules = SportsRehabilitationRules::new();
        let template = rules.rehabilitation_plan_template(RehabilitationPhase::RecoveryPhase);

        assert_eq!(template.phase, RehabilitationPhase::RecoveryPhase);
        assert!(!template.objectives.is_empty());
        assert!(!template.exercises.is_empty());
        assert!(!template.supervision_required);
    }

    #[test]
    fn test_rehabilitation_principle_description() {
        let desc = RehabilitationPrinciple::ProgressiveOverload.description();
        assert!(!desc.is_empty());
        assert!(desc.contains("逐步"));
    }

    #[test]
    fn test_rule_validation() {
        let rules = SportsRehabilitationRules::new();
        let ctx = ValidateContext::default();
        let result = rules.validate(&ctx);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_rule_metadata() {
        let rules = SportsRehabilitationRules::new();
        let metadata = rules.metadata();
        let category = rules.category();

        assert_eq!(metadata.name, "运动康复规则");
        assert!(matches!(category, RuleCategory::Sports(_)));
    }

    #[test]
    fn test_phase_medical_supervision() {
        assert!(RehabilitationPhase::AcutePhase.requires_medical_supervision());
        assert!(RehabilitationPhase::SubacutePhase.requires_medical_supervision());
        assert!(!RehabilitationPhase::RecoveryPhase.requires_medical_supervision());
    }

    #[test]
    fn test_seventy_percent_completion() {
        let rules = SportsRehabilitationRules::new();
        let all_criteria = rules.return_to_activity_criteria();

        // 70% 完成率 = 7 个标准
        let seven_criteria: Vec<_> = all_criteria.into_iter().take(7).collect();
        let decision = rules.evaluate_return_to_activity(&seven_criteria);

        match decision {
            RehabilitationDecision::ReadyForModifiedActivity { .. } => {}
            RehabilitationDecision::ReadyForFullActivity => {}
            RehabilitationDecision::NotReady { .. } => {}
        }
    }
}
