//! 运动员健康监测规则
//!
//! 基于运动医学标准，包含运动员健康评估、体能监测、训练负荷管理、
//! 过度训练综合征预防等健康监测体系。
//!
//! # 规则体系
//!
//! - 运动员健康评估标准
//! - 体能监测指标
//! - 训练负荷管理
//! - 过度训练综合征预防
//! - 健康档案管理
//!
//! # Examples
//!
//! ```
//! use world_rules::rules::sports::athlete_health_monitoring::AthleteHealthMonitoringRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = AthleteHealthMonitoringRules::new();
//! assert!(!rules.health_indicators().is_empty());
//! assert!(!rules.monitoring_intervals().is_empty());
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 健康指标类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HealthIndicator {
    /// 心率
    HeartRate,
    /// 血压
    BloodPressure,
    /// 体温
    BodyTemperature,
    /// 血氧饱和度
    OxygenSaturation,
    /// 体重
    BodyWeight,
    /// 体脂率
    BodyFatPercentage,
    /// 肌肉质量
    MuscleMass,
    /// 骨密度
    BoneDensity,
    /// 血红蛋白
    Hemoglobin,
    /// 睾酮水平
    TestosteroneLevel,
    /// 皮质醇水平
    CortisolLevel,
    /// 睡眠质量
    SleepQuality,
    /// 晨起心率
    RestingHeartRate,
    /// 心率变异性
    HeartRateVariability,
}

impl HealthIndicator {
    /// 获取指标名称
    pub fn name(&self) -> &'static str {
        match self {
            HealthIndicator::HeartRate => "心率",
            HealthIndicator::BloodPressure => "血压",
            HealthIndicator::BodyTemperature => "体温",
            HealthIndicator::OxygenSaturation => "血氧饱和度",
            HealthIndicator::BodyWeight => "体重",
            HealthIndicator::BodyFatPercentage => "体脂率",
            HealthIndicator::MuscleMass => "肌肉质量",
            HealthIndicator::BoneDensity => "骨密度",
            HealthIndicator::Hemoglobin => "血红蛋白",
            HealthIndicator::TestosteroneLevel => "睾酮水平",
            HealthIndicator::CortisolLevel => "皮质醇水平",
            HealthIndicator::SleepQuality => "睡眠质量",
            HealthIndicator::RestingHeartRate => "晨起心率",
            HealthIndicator::HeartRateVariability => "心率变异性",
        }
    }

    /// 获取监测频率（天）
    pub fn monitoring_frequency_days(&self) -> u32 {
        match self {
            HealthIndicator::HeartRate => 1,        // 每日
            HealthIndicator::BloodPressure => 7,    // 每周
            HealthIndicator::BodyTemperature => 1,  // 每日
            HealthIndicator::OxygenSaturation => 1, // 每日（训练日）
            HealthIndicator::BodyWeight => 1,       // 每日
            HealthIndicator::BodyFatPercentage => 30, // 每月
            HealthIndicator::MuscleMass => 30,      // 每月
            HealthIndicator::BoneDensity => 365,    // 每年
            HealthIndicator::Hemoglobin => 90,      // 每季度
            HealthIndicator::TestosteroneLevel => 90, // 每季度
            HealthIndicator::CortisolLevel => 30,   // 每月
            HealthIndicator::SleepQuality => 1,     // 每日
            HealthIndicator::RestingHeartRate => 1, // 每日
            HealthIndicator::HeartRateVariability => 1, // 每日
        }
    }

    /// 是否为关键健康指标
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            HealthIndicator::HeartRate
                | HealthIndicator::BloodPressure
                | HealthIndicator::BodyTemperature
                | HealthIndicator::OxygenSaturation
        )
    }
}

/// 过度训练综合征阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OvertrainingStage {
    /// 正常状态
    Normal,
    /// 功能性过度训练（早期）
    FunctionalOverreaching,
    /// 非功能性过度训练（中期）
    NonFunctionalOverreaching,
    /// 过度训练综合征（晚期）
    OvertrainingSyndrome,
}

impl OvertrainingStage {
    /// 获取阶段名称
    pub fn name(&self) -> &'static str {
        match self {
            OvertrainingStage::Normal => "正常",
            OvertrainingStage::FunctionalOverreaching => "功能性过度训练",
            OvertrainingStage::NonFunctionalOverreaching => "非功能性过度训练",
            OvertrainingStage::OvertrainingSyndrome => "过度训练综合征",
        }
    }

    /// 获取恢复时间范围（天）
    pub fn recovery_days(&self) -> std::ops::Range<u32> {
        match self {
            OvertrainingStage::Normal => 0..1,
            OvertrainingStage::FunctionalOverreaching => 2..7,
            OvertrainingStage::NonFunctionalOverreaching => 14..60,
            OvertrainingStage::OvertrainingSyndrome => 60..365,
        }
    }

    /// 是否需要医疗干预
    pub fn requires_medical_intervention(&self) -> bool {
        matches!(
            self,
            OvertrainingStage::NonFunctionalOverreaching | OvertrainingStage::OvertrainingSyndrome
        )
    }

    /// 获取预警信号
    pub fn warning_signs(&self) -> Vec<&'static str> {
        match self {
            OvertrainingStage::Normal => vec![],
            OvertrainingStage::FunctionalOverreaching => {
                vec!["疲劳感增加", "肌肉酸痛持续", "训练表现轻微下降"]
            }
            OvertrainingStage::NonFunctionalOverreaching => {
                vec![
                    "持续疲劳",
                    "睡眠障碍",
                    "食欲下降",
                    "情绪波动",
                    "训练表现明显下降",
                ]
            }
            OvertrainingStage::OvertrainingSyndrome => {
                vec![
                    "严重疲劳",
                    "失眠",
                    "抑郁",
                    "免疫力下降",
                    "内分泌失调",
                    "运动表现崩溃",
                ]
            }
        }
    }
}

/// 训练负荷指标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrainingLoadMetric {
    /// 急性训练负荷（1周）
    AcuteLoad,
    /// 慢性训练负荷（4周）
    ChronicLoad,
    /// 急慢性负荷比（ACWR）
    AcuteChronicWorkloadRatio,
    /// 训练单调性
    TrainingMonotony,
    /// 训练压力指数
    TrainingStrain,
}

impl TrainingLoadMetric {
    /// 获取指标名称
    pub fn name(&self) -> &'static str {
        match self {
            TrainingLoadMetric::AcuteLoad => "急性训练负荷",
            TrainingLoadMetric::ChronicLoad => "慢性训练负荷",
            TrainingLoadMetric::AcuteChronicWorkloadRatio => "急慢性负荷比",
            TrainingLoadMetric::TrainingMonotony => "训练单调性",
            TrainingLoadMetric::TrainingStrain => "训练压力指数",
        }
    }

    /// 获取安全阈值范围
    pub fn safe_threshold(&self) -> (f64, f64) {
        match self {
            TrainingLoadMetric::AcuteLoad => (0.0, 1000.0),      // 取决于运动项目
            TrainingLoadMetric::ChronicLoad => (0.0, 800.0),      // 取决于运动项目
            TrainingLoadMetric::AcuteChronicWorkloadRatio => (0.8, 1.3), // 推荐 0.8-1.3
            TrainingLoadMetric::TrainingMonotony => (0.0, 2.0),   // < 2.0 为安全
            TrainingLoadMetric::TrainingStrain => (0.0, 2000.0),  // 取决于运动项目
        }
    }

    /// 检查值是否在安全范围内
    pub fn is_safe(&self, value: f64) -> bool {
        let (min, max) = self.safe_threshold();
        value >= min && value <= max
    }
}

/// 健康评估等级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HealthAssessmentGrade {
    /// 优秀
    Excellent,
    /// 良好
    Good,
    /// 一般
    Fair,
    /// 需关注
    NeedsAttention,
    /// 不适合训练
    Unfit,
}

impl HealthAssessmentGrade {
    /// 获取等级名称
    pub fn name(&self) -> &'static str {
        match self {
            HealthAssessmentGrade::Excellent => "优秀",
            HealthAssessmentGrade::Good => "良好",
            HealthAssessmentGrade::Fair => "一般",
            HealthAssessmentGrade::NeedsAttention => "需关注",
            HealthAssessmentGrade::Unfit => "不适合训练",
        }
    }

    /// 是否适合训练
    pub fn can_train(&self) -> bool {
        matches!(
            self,
            HealthAssessmentGrade::Excellent
                | HealthAssessmentGrade::Good
                | HealthAssessmentGrade::Fair
        )
    }

    /// 是否需要调整训练计划
    pub fn requires_adjustment(&self) -> bool {
        matches!(
            self,
            HealthAssessmentGrade::Fair | HealthAssessmentGrade::NeedsAttention
        )
    }
}

/// 监测周期类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonitoringInterval {
    /// 每日监测
    Daily,
    /// 每周监测
    Weekly,
    /// 每月监测
    Monthly,
    /// 每季度监测
    Quarterly,
    /// 年度体检
    Annual,
}

impl MonitoringInterval {
    /// 获取周期名称
    pub fn name(&self) -> &'static str {
        match self {
            MonitoringInterval::Daily => "每日",
            MonitoringInterval::Weekly => "每周",
            MonitoringInterval::Monthly => "每月",
            MonitoringInterval::Quarterly => "每季度",
            MonitoringInterval::Annual => "年度",
        }
    }

    /// 获取间隔天数
    pub fn days(&self) -> u32 {
        match self {
            MonitoringInterval::Daily => 1,
            MonitoringInterval::Weekly => 7,
            MonitoringInterval::Monthly => 30,
            MonitoringInterval::Quarterly => 90,
            MonitoringInterval::Annual => 365,
        }
    }
}

/// 健康档案记录项
#[derive(Debug, Clone, PartialEq)]
pub struct HealthRecordItem {
    /// 指标类型
    pub indicator: HealthIndicator,
    /// 测量值
    pub value: f64,
    /// 单位
    pub unit: String,
    /// 测量时间戳
    pub timestamp: u64,
    /// 是否异常
    pub is_abnormal: bool,
}

/// 运动员健康监测规则
pub struct AthleteHealthMonitoringRules {
    metadata: RuleMetadata,
}

impl AthleteHealthMonitoringRules {
    /// 创建新的运动员健康监测规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "运动员健康监测规则",
                "运动员健康评估、体能监测、训练负荷管理、过度训练预防等规则体系",
            )
            .with_origin("运动医学标准")
            .with_tags(vec![
                "体育".into(),
                "健康监测".into(),
                "过度训练".into(),
                "训练负荷".into(),
            ]),
        }
    }

    /// 获取健康指标列表
    pub fn health_indicators(&self) -> Vec<HealthIndicator> {
        vec![
            HealthIndicator::HeartRate,
            HealthIndicator::BloodPressure,
            HealthIndicator::BodyTemperature,
            HealthIndicator::OxygenSaturation,
            HealthIndicator::BodyWeight,
            HealthIndicator::BodyFatPercentage,
            HealthIndicator::MuscleMass,
            HealthIndicator::BoneDensity,
            HealthIndicator::Hemoglobin,
            HealthIndicator::TestosteroneLevel,
            HealthIndicator::CortisolLevel,
            HealthIndicator::SleepQuality,
            HealthIndicator::RestingHeartRate,
            HealthIndicator::HeartRateVariability,
        ]
    }

    /// 获取关键健康指标
    pub fn critical_indicators(&self) -> Vec<HealthIndicator> {
        self.health_indicators()
            .into_iter()
            .filter(|i| i.is_critical())
            .collect()
    }

    /// 获取监测周期列表
    pub fn monitoring_intervals(&self) -> Vec<MonitoringInterval> {
        vec![
            MonitoringInterval::Daily,
            MonitoringInterval::Weekly,
            MonitoringInterval::Monthly,
            MonitoringInterval::Quarterly,
            MonitoringInterval::Annual,
        ]
    }

    /// 获取训练负荷指标列表
    pub fn training_load_metrics(&self) -> Vec<TrainingLoadMetric> {
        vec![
            TrainingLoadMetric::AcuteLoad,
            TrainingLoadMetric::ChronicLoad,
            TrainingLoadMetric::AcuteChronicWorkloadRatio,
            TrainingLoadMetric::TrainingMonotony,
            TrainingLoadMetric::TrainingStrain,
        ]
    }

    /// 评估过度训练风险
    ///
    /// # Arguments
    /// * `acwr` - 急慢性负荷比
    /// * `monotony` - 训练单调性
    /// * `resting_hr_change` - 晨起心率变化百分比
    /// * `sleep_quality_score` - 睡眠质量评分（0-100）
    /// * `mood_score` - 情绪评分（0-100）
    pub fn assess_overtraining_risk(
        &self,
        acwr: f64,
        monotony: f64,
        resting_hr_change: f64,
        sleep_quality_score: u32,
        mood_score: u32,
    ) -> OvertrainingStage {
        let mut risk_factors = 0;

        // 检查急慢性负荷比
        if acwr > 1.5 {
            risk_factors += 2;
        } else if acwr > 1.3 {
            risk_factors += 1;
        }

        // 检查训练单调性
        if monotony > 2.5 {
            risk_factors += 2;
        } else if monotony > 2.0 {
            risk_factors += 1;
        }

        // 检查晨起心率变化
        if resting_hr_change > 10.0 {
            risk_factors += 2;
        } else if resting_hr_change > 5.0 {
            risk_factors += 1;
        }

        // 检查睡眠质量
        if sleep_quality_score < 50 {
            risk_factors += 2;
        } else if sleep_quality_score < 70 {
            risk_factors += 1;
        }

        // 检查情绪状态
        if mood_score < 50 {
            risk_factors += 2;
        } else if mood_score < 70 {
            risk_factors += 1;
        }

        match risk_factors {
            0..=1 => OvertrainingStage::Normal,
            2..=3 => OvertrainingStage::FunctionalOverreaching,
            4..=6 => OvertrainingStage::NonFunctionalOverreaching,
            _ => OvertrainingStage::OvertrainingSyndrome,
        }
    }

    /// 计算急慢性负荷比（ACWR）
    ///
    /// # Arguments
    /// * `acute_load` - 急性训练负荷（1周总和）
    /// * `chronic_load` - 慢性训练负荷（4周平均值）
    pub fn calculate_acwr(&self, acute_load: f64, chronic_load: f64) -> f64 {
        if chronic_load > 0.0 {
            acute_load / chronic_load
        } else {
            0.0
        }
    }

    /// 计算训练单调性
    ///
    /// # Arguments
    /// * `weekly_loads` - 一周内每日训练负荷
    pub fn calculate_monotony(&self, weekly_loads: &[f64]) -> f64 {
        if weekly_loads.is_empty() {
            return 0.0;
        }

        let mean = weekly_loads.iter().sum::<f64>() / weekly_loads.len() as f64;
        if mean == 0.0 {
            return 0.0;
        }

        let variance: f64 = weekly_loads
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>()
            / weekly_loads.len() as f64;

        let std_dev = variance.sqrt();
        std_dev / mean
    }

    /// 计算训练压力指数
    ///
    /// # Arguments
    /// * `weekly_load` - 周训练负荷总和
    /// * `monotony` - 训练单调性
    pub fn calculate_strain(&self, weekly_load: f64, monotony: f64) -> f64 {
        weekly_load * monotony
    }

    /// 综合健康评估
    ///
    /// # Arguments
    /// * `hr_status` - 心率状态（正常/偏高/偏低）
    /// * `bp_status` - 血压状态
    /// * `sleep_score` - 睡眠评分（0-100）
    /// * `fatigue_level` - 疲劳程度（0-10）
    /// * `injury_status` - 是否有伤病
    pub fn comprehensive_assessment(
        &self,
        hr_normal: bool,
        bp_normal: bool,
        sleep_score: u32,
        fatigue_level: u32,
        has_injury: bool,
    ) -> HealthAssessmentGrade {
        let mut score = 100;

        if !hr_normal {
            score -= 15;
        }
        if !bp_normal {
            score -= 15;
        }
        if sleep_score < 60 {
            score -= 20;
        } else if sleep_score < 80 {
            score -= 10;
        }
        if fatigue_level > 7 {
            score -= 20;
        } else if fatigue_level > 5 {
            score -= 10;
        }
        if has_injury {
            score -= 25;
        }

        match score {
            90..=100 => HealthAssessmentGrade::Excellent,
            75..=89 => HealthAssessmentGrade::Good,
            60..=74 => HealthAssessmentGrade::Fair,
            40..=59 => HealthAssessmentGrade::NeedsAttention,
            _ => HealthAssessmentGrade::Unfit,
        }
    }

    /// 获取健康监测检查清单
    pub fn daily_checklist(&self) -> Vec<&'static str> {
        vec![
            "晨起心率测量",
            "体重测量",
            "睡眠质量自评",
            "疲劳程度评估",
            "肌肉酸痛评估",
            "情绪状态评估",
            "饮水情况记录",
            "饮食情况记录",
        ]
    }

    /// 获取周度检查清单
    pub fn weekly_checklist(&self) -> Vec<&'static str> {
        vec![
            "训练负荷统计",
            "心率变异性分析",
            "体重变化趋势分析",
            "训练恢复评估",
            "营养补充评估",
            "心理状态评估",
        ]
    }

    /// 获取年度体检项目清单
    pub fn annual_checkup_items(&self) -> Vec<&'static str> {
        vec![
            "全面血液检查",
            "心电图检查",
            "心脏超声",
            "骨密度检测",
            "体成分分析",
            "运动能力测试",
            "营养状况评估",
            "心理健康评估",
        ]
    }
}

impl Default for AthleteHealthMonitoringRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AthleteHealthMonitoringRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("athlete_health_monitoring")
    }

    fn explain(&self) -> String {
        String::from(
            "运动员健康监测规则\n\
            \n\
            本规则体系包含以下核心内容：\n\
            \n\
            1. 健康指标监测\n\
            - 心率、血压、体温等基础生命体征\n\
            - 血氧、血红蛋白等血液指标\n\
            - 睾酮、皮质醇等内分泌指标\n\
            - 睡眠质量、心率变异性等恢复指标\n\
            \n\
            2. 训练负荷管理\n\
            - 急性训练负荷（ATL）：1周训练负荷总和\n\
            - 慢性训练负荷（CTL）：4周训练负荷平均值\n\
            - 急慢性负荷比（ACWR）：推荐 0.8-1.3\n\
            - 训练单调性：建议 < 2.0\n\
            - 训练压力指数：结合负荷与单调性\n\
            \n\
            3. 过度训练预防\n\
            - 功能性过度训练：2-7天可恢复\n\
            - 非功能性过度训练：需14-60天恢复\n\
            - 过度训练综合征：需数月恢复，需医疗干预\n\
            \n\
            4. 健康评估标准\n\
            - 优秀：所有指标正常，可正常训练\n\
            - 良好：大部分指标正常，可正常训练\n\
            - 一般：部分指标异常，需调整训练\n\
            - 需关注：多项指标异常，需减量训练\n\
            - 不适合训练：健康风险较高，需休息\n\
            \n\
            5. 监测周期\n\
            - 每日：晨起心率、体重、睡眠、疲劳自评\n\
            - 每周：训练负荷统计、恢复评估\n\
            - 每月：体成分、内分泌指标\n\
            - 每季度：血液检查\n\
            - 年度：全面体检",
        )
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        // 简单验证实现
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_indicator_names() {
        assert_eq!(HealthIndicator::HeartRate.name(), "心率");
        assert_eq!(HealthIndicator::BloodPressure.name(), "血压");
        assert_eq!(HealthIndicator::CortisolLevel.name(), "皮质醇水平");
    }

    #[test]
    fn test_health_indicator_monitoring_frequency() {
        assert_eq!(HealthIndicator::HeartRate.monitoring_frequency_days(), 1);
        assert_eq!(HealthIndicator::BloodPressure.monitoring_frequency_days(), 7);
        assert_eq!(HealthIndicator::BoneDensity.monitoring_frequency_days(), 365);
    }

    #[test]
    fn test_critical_indicators() {
        assert!(HealthIndicator::HeartRate.is_critical());
        assert!(HealthIndicator::BloodPressure.is_critical());
        assert!(!HealthIndicator::BodyWeight.is_critical());
    }

    #[test]
    fn test_overtraining_stage_names() {
        assert_eq!(OvertrainingStage::Normal.name(), "正常");
        assert_eq!(
            OvertrainingStage::FunctionalOverreaching.name(),
            "功能性过度训练"
        );
        assert_eq!(
            OvertrainingStage::OvertrainingSyndrome.name(),
            "过度训练综合征"
        );
    }

    #[test]
    fn test_overtraining_recovery_days() {
        let normal_range = OvertrainingStage::Normal.recovery_days();
        assert!(normal_range.contains(&0));

        let syndrome_range = OvertrainingStage::OvertrainingSyndrome.recovery_days();
        assert!(syndrome_range.start >= 60);
    }

    #[test]
    fn test_overtraining_warning_signs() {
        let signs = OvertrainingStage::NonFunctionalOverreaching.warning_signs();
        assert!(!signs.is_empty());
        assert!(signs.contains(&"睡眠障碍"));
    }

    #[test]
    fn test_training_load_metric_thresholds() {
        let (min, max) = TrainingLoadMetric::AcuteChronicWorkloadRatio.safe_threshold();
        assert_eq!(min, 0.8);
        assert_eq!(max, 1.3);

        assert!(TrainingLoadMetric::AcuteChronicWorkloadRatio.is_safe(1.0));
        assert!(!TrainingLoadMetric::AcuteChronicWorkloadRatio.is_safe(2.0));
    }

    #[test]
    fn test_health_assessment_grade() {
        assert!(HealthAssessmentGrade::Excellent.can_train());
        assert!(HealthAssessmentGrade::Good.can_train());
        assert!(!HealthAssessmentGrade::Unfit.can_train());

        assert!(HealthAssessmentGrade::Fair.requires_adjustment());
        assert!(!HealthAssessmentGrade::Excellent.requires_adjustment());
    }

    #[test]
    fn test_monitoring_interval_days() {
        assert_eq!(MonitoringInterval::Daily.days(), 1);
        assert_eq!(MonitoringInterval::Weekly.days(), 7);
        assert_eq!(MonitoringInterval::Monthly.days(), 30);
        assert_eq!(MonitoringInterval::Quarterly.days(), 90);
        assert_eq!(MonitoringInterval::Annual.days(), 365);
    }

    #[test]
    fn test_rules_creation() {
        let rules = AthleteHealthMonitoringRules::new();
        assert!(!rules.health_indicators().is_empty());
        assert!(!rules.monitoring_intervals().is_empty());
        assert!(!rules.training_load_metrics().is_empty());
    }

    #[test]
    fn test_critical_indicators_list() {
        let rules = AthleteHealthMonitoringRules::new();
        let critical = rules.critical_indicators();
        assert!(!critical.is_empty());
        assert!(critical.iter().all(|i| i.is_critical()));
    }

    #[test]
    fn test_acwr_calculation() {
        let rules = AthleteHealthMonitoringRules::new();

        // 正常范围
        let acwr = rules.calculate_acwr(600.0, 500.0);
        assert!((acwr - 1.2).abs() < 0.01);

        // 边界情况
        let acwr_zero = rules.calculate_acwr(100.0, 0.0);
        assert_eq!(acwr_zero, 0.0);
    }

    #[test]
    fn test_monotony_calculation() {
        let rules = AthleteHealthMonitoringRules::new();

        // 高单调性（每日负荷相同）
        let high_monotony = rules.calculate_monotony(&[100.0, 100.0, 100.0, 100.0, 100.0, 100.0, 100.0]);
        assert!(high_monotony < 0.01); // 应该接近 0

        // 低单调性（每日负荷变化大）
        let low_monotony = rules.calculate_monotony(&[0.0, 200.0, 50.0, 150.0, 0.0, 200.0, 100.0]);
        assert!(low_monotony > 0.5);

        // 空数组
        let empty = rules.calculate_monotony(&[]);
        assert_eq!(empty, 0.0);
    }

    #[test]
    fn test_strain_calculation() {
        let rules = AthleteHealthMonitoringRules::new();

        let strain = rules.calculate_strain(700.0, 1.5);
        assert!((strain - 1050.0).abs() < 0.01);
    }

    #[test]
    fn test_overtraining_risk_assessment() {
        let rules = AthleteHealthMonitoringRules::new();

        // 正常状态
        let stage = rules.assess_overtraining_risk(1.0, 1.5, 2.0, 80, 80);
        assert_eq!(stage, OvertrainingStage::Normal);

        // 功能性过度训练
        let stage = rules.assess_overtraining_risk(1.4, 2.2, 6.0, 65, 65);
        assert!(matches!(
            stage,
            OvertrainingStage::FunctionalOverreaching | OvertrainingStage::NonFunctionalOverreaching
        ));

        // 过度训练综合征
        let stage = rules.assess_overtraining_risk(2.0, 3.0, 15.0, 40, 40);
        assert_eq!(stage, OvertrainingStage::OvertrainingSyndrome);
    }

    #[test]
    fn test_comprehensive_assessment() {
        let rules = AthleteHealthMonitoringRules::new();

        // 优秀状态
        let grade = rules.comprehensive_assessment(true, true, 90, 2, false);
        assert_eq!(grade, HealthAssessmentGrade::Excellent);

        // 良好状态
        let grade = rules.comprehensive_assessment(true, true, 80, 4, false);
        assert_eq!(grade, HealthAssessmentGrade::Good);

        // 不适合训练
        let grade = rules.comprehensive_assessment(false, false, 50, 8, true);
        assert_eq!(grade, HealthAssessmentGrade::Unfit);
    }

    #[test]
    fn test_checklists() {
        let rules = AthleteHealthMonitoringRules::new();

        assert!(!rules.daily_checklist().is_empty());
        assert!(!rules.weekly_checklist().is_empty());
        assert!(!rules.annual_checkup_items().is_empty());

        assert!(rules.daily_checklist().contains(&"晨起心率测量"));
        assert!(rules.annual_checkup_items().contains(&"心电图检查"));
    }

    #[test]
    fn test_rule_trait() {
        let rules = AthleteHealthMonitoringRules::new();

        assert_eq!(rules.metadata().name, "运动员健康监测规则");
        assert_eq!(rules.category(), RuleCategory::Sports);

        let explanation = rules.explain();
        assert!(explanation.contains("训练负荷管理"));
        assert!(explanation.contains("过度训练预防"));
    }
}