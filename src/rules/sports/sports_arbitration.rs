//! 体育仲裁规则
//!
//! 基于 CAS（国际体育仲裁法院）标准，包含仲裁程序、申诉规则、纪律处罚、争议解决等。
//!
//! # 规则体系
//!
//! - CAS 国际体育仲裁法院规则
//! - 国际单项体育组织仲裁规则
//! - 国家体育仲裁规则
//!
//! # Examples
//!
//! ```
//! use world_rules::rules::sports::sports_arbitration::SportsArbitrationRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = SportsArbitrationRules::new();
//! assert!(!rules.arbitration_types().is_empty());
//! assert!(rules.filing_deadline_days() > 0);
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 仲裁类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArbitrationType {
    /// 普通仲裁程序
    Ordinary,
    /// 申诉仲裁程序
    Appeal,
    /// 咨询意见程序
    Advisory,
    /// 紧急仲裁程序
    Provisional,
}

impl ArbitrationType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            ArbitrationType::Ordinary => "普通仲裁程序",
            ArbitrationType::Appeal => "申诉仲裁程序",
            ArbitrationType::Advisory => "咨询意见程序",
            ArbitrationType::Provisional => "紧急仲裁程序",
        }
    }

    /// 获取程序时限（天）
    pub fn timeline_days(&self) -> u32 {
        match self {
            ArbitrationType::Ordinary => 180,   // 6个月
            ArbitrationType::Appeal => 90,      // 3个月
            ArbitrationType::Advisory => 60,    // 2个月
            ArbitrationType::Provisional => 15, // 紧急程序15天
        }
    }

    /// 是否需要缴纳保证金
    pub fn requires_deposit(&self) -> bool {
        true
    }
}

/// 仲裁庭组成方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TribunalComposition {
    /// 独任仲裁员
    SingleArbitrator,
    /// 三人仲裁庭
    ThreeArbitrators,
    /// 快速程序独任
    ExpeditedSingle,
}

impl TribunalComposition {
    /// 获取仲裁员人数
    pub fn arbitrator_count(&self) -> u32 {
        match self {
            TribunalComposition::SingleArbitrator => 1,
            TribunalComposition::ThreeArbitrators => 3,
            TribunalComposition::ExpeditedSingle => 1,
        }
    }

    /// 获取名称
    pub fn name(&self) -> &'static str {
        match self {
            TribunalComposition::SingleArbitrator => "独任仲裁员",
            TribunalComposition::ThreeArbitrators => "三人仲裁庭",
            TribunalComposition::ExpeditedSingle => "快速程序独任",
        }
    }
}

/// 申诉类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppealType {
    /// 纪律处罚申诉
    DisciplinarySanction,
    /// 兴奋剂处罚申诉
    DopingSanction,
    /// 资格认定申诉
    EligibilityDecision,
    /// 选拔决定申诉
    SelectionDecision,
    /// 比赛结果申诉
    CompetitionResult,
    /// 许可证申诉
    LicenseDecision,
}

impl AppealType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            AppealType::DisciplinarySanction => "纪律处罚申诉",
            AppealType::DopingSanction => "兴奋剂处罚申诉",
            AppealType::EligibilityDecision => "资格认定申诉",
            AppealType::SelectionDecision => "选拔决定申诉",
            AppealType::CompetitionResult => "比赛结果申诉",
            AppealType::LicenseDecision => "许可证申诉",
        }
    }

    /// 获取申诉期限（天）
    pub fn appeal_deadline_days(&self) -> u32 {
        match self {
            AppealType::DisciplinarySanction => 21,
            AppealType::DopingSanction => 21,
            AppealType::EligibilityDecision => 21,
            AppealType::SelectionDecision => 10, // 选拔决定申诉期限较短
            AppealType::CompetitionResult => 10,
            AppealType::LicenseDecision => 21,
        }
    }

    /// 是否可以申请暂缓执行
    pub fn can_request_stay(&self) -> bool {
        matches!(
            self,
            AppealType::DisciplinarySanction
                | AppealType::DopingSanction
                | AppealType::LicenseDecision
        )
    }
}

/// 纪律处罚类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisciplinarySanctionType {
    /// 警告
    Warning,
    /// 罚款
    Fine,
    /// 禁赛
    Suspension,
    /// 取消比赛成绩
    Disqualification,
    /// 剥夺奖牌/奖杯
    StripMedal,
    /// 终身禁赛
    LifetimeBan,
    /// 经济制裁
    FinancialSanction,
    /// 社区服务
    CommunityService,
    /// 教育课程
    EducationalProgram,
}

impl DisciplinarySanctionType {
    /// 获取处罚名称
    pub fn name(&self) -> &'static str {
        match self {
            DisciplinarySanctionType::Warning => "警告",
            DisciplinarySanctionType::Fine => "罚款",
            DisciplinarySanctionType::Suspension => "禁赛",
            DisciplinarySanctionType::Disqualification => "取消比赛成绩",
            DisciplinarySanctionType::StripMedal => "剥夺奖牌",
            DisciplinarySanctionType::LifetimeBan => "终身禁赛",
            DisciplinarySanctionType::FinancialSanction => "经济制裁",
            DisciplinarySanctionType::CommunityService => "社区服务",
            DisciplinarySanctionType::EducationalProgram => "教育课程",
        }
    }

    /// 是否可申诉
    pub fn is_appealable(&self) -> bool {
        true // 所有处罚都可以申诉
    }

    /// 获取处罚严重等级（1-5）
    pub fn severity_level(&self) -> u32 {
        match self {
            DisciplinarySanctionType::Warning => 1,
            DisciplinarySanctionType::EducationalProgram => 1,
            DisciplinarySanctionType::CommunityService => 2,
            DisciplinarySanctionType::Fine => 2,
            DisciplinarySanctionType::FinancialSanction => 3,
            DisciplinarySanctionType::Disqualification => 3,
            DisciplinarySanctionType::Suspension => 4,
            DisciplinarySanctionType::StripMedal => 4,
            DisciplinarySanctionType::LifetimeBan => 5,
        }
    }
}

/// 违纪行为类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisciplinaryOffenseType {
    /// 暴力行为
    Violence,
    /// 不当言论
    OffensiveBehavior,
    /// 腐败行为
    Corruption,
    /// 操纵比赛
    MatchFixing,
    /// 赌博违规
    BettingViolation,
    /// 骚扰行为
    Harassment,
    /// 不服从裁判
    RefusalToComply,
    /// 虚假声明
    FalseRepresentation,
    /// 违反体育精神
    UnsportsmanlikeConduct,
    /// 违反安全规定
    SafetyViolation,
}

impl DisciplinaryOffenseType {
    /// 获取违纪类型名称
    pub fn name(&self) -> &'static str {
        match self {
            DisciplinaryOffenseType::Violence => "暴力行为",
            DisciplinaryOffenseType::OffensiveBehavior => "不当言论",
            DisciplinaryOffenseType::Corruption => "腐败行为",
            DisciplinaryOffenseType::MatchFixing => "操纵比赛",
            DisciplinaryOffenseType::BettingViolation => "赌博违规",
            DisciplinaryOffenseType::Harassment => "骚扰行为",
            DisciplinaryOffenseType::RefusalToComply => "不服从裁判",
            DisciplinaryOffenseType::FalseRepresentation => "虚假声明",
            DisciplinaryOffenseType::UnsportsmanlikeConduct => "违反体育精神",
            DisciplinaryOffenseType::SafetyViolation => "违反安全规定",
        }
    }

    /// 获取建议的最低处罚
    pub fn minimum_sanction(&self) -> DisciplinarySanctionType {
        match self {
            DisciplinaryOffenseType::Violence => DisciplinarySanctionType::Suspension,
            DisciplinaryOffenseType::OffensiveBehavior => DisciplinarySanctionType::Fine,
            DisciplinaryOffenseType::Corruption => DisciplinarySanctionType::Suspension,
            DisciplinaryOffenseType::MatchFixing => DisciplinarySanctionType::LifetimeBan,
            DisciplinaryOffenseType::BettingViolation => DisciplinarySanctionType::Suspension,
            DisciplinaryOffenseType::Harassment => DisciplinarySanctionType::Suspension,
            DisciplinaryOffenseType::RefusalToComply => DisciplinarySanctionType::Warning,
            DisciplinaryOffenseType::FalseRepresentation => DisciplinarySanctionType::Fine,
            DisciplinaryOffenseType::UnsportsmanlikeConduct => DisciplinarySanctionType::Warning,
            DisciplinaryOffenseType::SafetyViolation => DisciplinarySanctionType::Fine,
        }
    }

    /// 是否属于严重违纪
    pub fn is_serious_offense(&self) -> bool {
        matches!(
            self,
            DisciplinaryOffenseType::Violence
                | DisciplinaryOffenseType::Corruption
                | DisciplinaryOffenseType::MatchFixing
                | DisciplinaryOffenseType::BettingViolation
                | DisciplinaryOffenseType::Harassment
        )
    }
}

/// 争议解决程序阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisputeResolutionStage {
    /// 初步审查
    PreliminaryReview,
    /// 调解程序
    Mediation,
    /// 仲裁庭组建
    TribunalFormation,
    /// 书面审理
    WrittenProcedure,
    /// 听证程序
    Hearing,
    /// 裁决阶段
    Decision,
}

impl DisputeResolutionStage {
    /// 获取阶段名称
    pub fn name(&self) -> &'static str {
        match self {
            DisputeResolutionStage::PreliminaryReview => "初步审查",
            DisputeResolutionStage::Mediation => "调解程序",
            DisputeResolutionStage::TribunalFormation => "仲裁庭组建",
            DisputeResolutionStage::WrittenProcedure => "书面审理",
            DisputeResolutionStage::Hearing => "听证程序",
            DisputeResolutionStage::Decision => "裁决阶段",
        }
    }

    /// 获取建议时限（天）
    pub fn suggested_timeline_days(&self) -> u32 {
        match self {
            DisputeResolutionStage::PreliminaryReview => 7,
            DisputeResolutionStage::Mediation => 30,
            DisputeResolutionStage::TribunalFormation => 14,
            DisputeResolutionStage::WrittenProcedure => 30,
            DisputeResolutionStage::Hearing => 7,
            DisputeResolutionStage::Decision => 21,
        }
    }
}

/// 仲裁费用类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArbitrationCostType {
    /// 申请费
    FilingFee,
    /// 仲裁员费用
    ArbitratorFee,
    /// 管理费
    AdministrativeFee,
    /// 专家证人费用
    ExpertWitnessFee,
    /// 法律代表费用
    LegalRepresentationFee,
    /// 翻译费用
    TranslationFee,
    /// 听证场地费用
    HearingVenueFee,
}

impl ArbitrationCostType {
    /// 获取费用名称
    pub fn name(&self) -> &'static str {
        match self {
            ArbitrationCostType::FilingFee => "申请费",
            ArbitrationCostType::ArbitratorFee => "仲裁员费用",
            ArbitrationCostType::AdministrativeFee => "管理费",
            ArbitrationCostType::ExpertWitnessFee => "专家证人费用",
            ArbitrationCostType::LegalRepresentationFee => "法律代表费用",
            ArbitrationCostType::TranslationFee => "翻译费用",
            ArbitrationCostType::HearingVenueFee => "听证场地费用",
        }
    }

    /// 是否可申请减免
    pub fn can_request_waiver(&self) -> bool {
        matches!(
            self,
            ArbitrationCostType::FilingFee
                | ArbitrationCostType::AdministrativeFee
                | ArbitrationCostType::ArbitratorFee
        )
    }
}

/// 体育仲裁规则
#[derive(Debug, Clone)]
pub struct SportsArbitrationRules {
    /// 规则元数据
    metadata: RuleMetadata,
}

impl SportsArbitrationRules {
    /// 创建新的体育仲裁规则
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("体育仲裁规则", "CAS 国际体育仲裁法院规则"),
        }
    }

    /// 获取所有仲裁类型
    pub fn arbitration_types(&self) -> Vec<ArbitrationType> {
        vec![
            ArbitrationType::Ordinary,
            ArbitrationType::Appeal,
            ArbitrationType::Advisory,
            ArbitrationType::Provisional,
        ]
    }

    /// 获取所有申诉类型
    pub fn appeal_types(&self) -> Vec<AppealType> {
        vec![
            AppealType::DisciplinarySanction,
            AppealType::DopingSanction,
            AppealType::EligibilityDecision,
            AppealType::SelectionDecision,
            AppealType::CompetitionResult,
            AppealType::LicenseDecision,
        ]
    }

    /// 获取所有纪律处罚类型
    pub fn disciplinary_sanction_types(&self) -> Vec<DisciplinarySanctionType> {
        vec![
            DisciplinarySanctionType::Warning,
            DisciplinarySanctionType::Fine,
            DisciplinarySanctionType::Suspension,
            DisciplinarySanctionType::Disqualification,
            DisciplinarySanctionType::StripMedal,
            DisciplinarySanctionType::LifetimeBan,
            DisciplinarySanctionType::FinancialSanction,
            DisciplinarySanctionType::CommunityService,
            DisciplinarySanctionType::EducationalProgram,
        ]
    }

    /// 获取所有违纪行为类型
    pub fn disciplinary_offense_types(&self) -> Vec<DisciplinaryOffenseType> {
        vec![
            DisciplinaryOffenseType::Violence,
            DisciplinaryOffenseType::OffensiveBehavior,
            DisciplinaryOffenseType::Corruption,
            DisciplinaryOffenseType::MatchFixing,
            DisciplinaryOffenseType::BettingViolation,
            DisciplinaryOffenseType::Harassment,
            DisciplinaryOffenseType::RefusalToComply,
            DisciplinaryOffenseType::FalseRepresentation,
            DisciplinaryOffenseType::UnsportsmanlikeConduct,
            DisciplinaryOffenseType::SafetyViolation,
        ]
    }

    /// 获取所有争议解决阶段
    pub fn dispute_resolution_stages(&self) -> Vec<DisputeResolutionStage> {
        vec![
            DisputeResolutionStage::PreliminaryReview,
            DisputeResolutionStage::Mediation,
            DisputeResolutionStage::TribunalFormation,
            DisputeResolutionStage::WrittenProcedure,
            DisputeResolutionStage::Hearing,
            DisputeResolutionStage::Decision,
        ]
    }

    /// 获取标准申请期限（天）
    pub fn filing_deadline_days(&self) -> u32 {
        21 // CAS 标准申诉期限
    }

    /// 获取紧急程序期限（天）
    pub fn expedited_deadline_days(&self) -> u32 {
        15
    }

    /// 计算争议解决总时长估算（天）
    pub fn estimated_total_duration(&self) -> u32 {
        self.dispute_resolution_stages()
            .iter()
            .map(|s| s.suggested_timeline_days())
            .sum()
    }

    /// 判断申诉是否在期限内
    pub fn is_appeal_timely(&self, days_since_decision: u32, appeal_type: AppealType) -> bool {
        days_since_decision <= appeal_type.appeal_deadline_days()
    }

    /// 获取建议的仲裁庭组成方式
    pub fn recommended_tribunal_composition(
        &self,
        dispute_amount: Option<u32>,
        urgency: bool,
    ) -> TribunalComposition {
        if urgency {
            return TribunalComposition::ExpeditedSingle;
        }
        match dispute_amount {
            Some(amount) if amount > 100_000 => TribunalComposition::ThreeArbitrators,
            _ => TribunalComposition::SingleArbitrator,
        }
    }

    /// 计算最低保证金（瑞士法郎）
    pub fn calculate_min_deposit(&self, dispute_amount: u32) -> u32 {
        // CAS 最低保证金规则
        let base = 1_000; // 基础保证金
        let percentage = dispute_amount / 100; // 1% 比例
        base.max(percentage).max(500) // 最低500瑞士法郎
    }

    /// 获取 CAS 总部信息
    pub fn cas_headquarters(&self) -> (&'static str, &'static str, &'static str) {
        ("瑞士", "洛桑", "Maison du Sport International")
    }

    /// 获取 CAS 分支机构
    pub fn cas_divisions(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("普通仲裁庭", "处理商业性体育争议"),
            ("申诉仲裁庭", "处理纪律处罚和资格决定申诉"),
            ("奥运特设仲裁庭", "奥运会期间处理争议"),
        ]
    }

    /// 检查是否可申请法律援助
    pub fn can_request_legal_aid(&self, annual_income: u32) -> bool {
        annual_income < 50_000 // 年收入低于5万瑞士法郎可申请
    }

    /// 获取裁决执行期限（天）
    pub fn enforcement_deadline_days(&self) -> u32 {
        30 // 裁决生效后30天内执行
    }

    /// 获取裁决异议期限（天）
    pub fn challenge_deadline_days(&self) -> u32 {
        30 // 裁决后30天内可提出异议
    }
}

impl Default for SportsArbitrationRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SportsArbitrationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("arbitration")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arbitration_types() {
        let rules = SportsArbitrationRules::new();
        let types = rules.arbitration_types();
        assert_eq!(types.len(), 4);
        assert_eq!(ArbitrationType::Ordinary.name(), "普通仲裁程序");
        assert_eq!(ArbitrationType::Appeal.timeline_days(), 90);
    }

    #[test]
    fn test_appeal_types() {
        let rules = SportsArbitrationRules::new();
        let types = rules.appeal_types();
        assert_eq!(types.len(), 6);
        assert_eq!(AppealType::DopingSanction.appeal_deadline_days(), 21);
        assert!(AppealType::DopingSanction.can_request_stay());
    }

    #[test]
    fn test_disciplinary_sanctions() {
        let rules = SportsArbitrationRules::new();
        let sanctions = rules.disciplinary_sanction_types();
        assert_eq!(sanctions.len(), 9);
        assert!(DisciplinarySanctionType::LifetimeBan.is_appealable());
        assert_eq!(DisciplinarySanctionType::LifetimeBan.severity_level(), 5);
    }

    #[test]
    fn test_disciplinary_offenses() {
        let rules = SportsArbitrationRules::new();
        let offenses = rules.disciplinary_offense_types();
        assert_eq!(offenses.len(), 10);
        assert!(DisciplinaryOffenseType::MatchFixing.is_serious_offense());
        assert_eq!(
            DisciplinaryOffenseType::MatchFixing.minimum_sanction(),
            DisciplinarySanctionType::LifetimeBan
        );
    }

    #[test]
    fn test_dispute_resolution_stages() {
        let rules = SportsArbitrationRules::new();
        let stages = rules.dispute_resolution_stages();
        assert_eq!(stages.len(), 6);
        assert_eq!(
            DisputeResolutionStage::PreliminaryReview.suggested_timeline_days(),
            7
        );
    }

    #[test]
    fn test_filing_deadline() {
        let rules = SportsArbitrationRules::new();
        assert_eq!(rules.filing_deadline_days(), 21);
        assert_eq!(rules.expedited_deadline_days(), 15);
    }

    #[test]
    fn test_appeal_timely() {
        let rules = SportsArbitrationRules::new();
        // 15天内在21天期限内
        assert!(rules.is_appeal_timely(15, AppealType::DisciplinarySanction));
        // 25天超出21天期限
        assert!(!rules.is_appeal_timely(25, AppealType::DisciplinarySanction));
        // 10天在选拔决定10天期限内
        assert!(rules.is_appeal_timely(10, AppealType::SelectionDecision));
    }

    #[test]
    fn test_tribunal_composition() {
        let rules = SportsArbitrationRules::new();
        // 紧急案件使用快速程序
        assert_eq!(
            rules.recommended_tribunal_composition(None, true),
            TribunalComposition::ExpeditedSingle
        );
        // 小额争议使用独任仲裁员
        assert_eq!(
            rules.recommended_tribunal_composition(Some(50_000), false),
            TribunalComposition::SingleArbitrator
        );
        // 大额争议使用三人仲裁庭
        assert_eq!(
            rules.recommended_tribunal_composition(Some(200_000), false),
            TribunalComposition::ThreeArbitrators
        );
    }

    #[test]
    fn test_deposit_calculation() {
        let rules = SportsArbitrationRules::new();
        // 最低500瑞士法郎
        assert_eq!(rules.calculate_min_deposit(100), 500);
        // 争议金额1%计算
        assert_eq!(rules.calculate_min_deposit(100_000), 1000);
        assert_eq!(rules.calculate_min_deposit(200_000), 2000);
    }

    #[test]
    fn test_cas_info() {
        let rules = SportsArbitrationRules::new();
        let (country, city, _) = rules.cas_headquarters();
        assert_eq!(country, "瑞士");
        assert_eq!(city, "洛桑");

        let divisions = rules.cas_divisions();
        assert_eq!(divisions.len(), 3);
    }

    #[test]
    fn test_legal_aid() {
        let rules = SportsArbitrationRules::new();
        // 低收入可申请法律援助
        assert!(rules.can_request_legal_aid(30_000));
        // 高收入不可申请
        assert!(!rules.can_request_legal_aid(100_000));
    }

    #[test]
    fn test_enforcement_deadline() {
        let rules = SportsArbitrationRules::new();
        assert_eq!(rules.enforcement_deadline_days(), 30);
        assert_eq!(rules.challenge_deadline_days(), 30);
    }

    #[test]
    fn test_estimated_duration() {
        let rules = SportsArbitrationRules::new();
        // 各阶段时长总和
        let total = rules.estimated_total_duration();
        assert!(total > 0);
    }

    #[test]
    fn test_arbitrator_count() {
        assert_eq!(TribunalComposition::SingleArbitrator.arbitrator_count(), 1);
        assert_eq!(TribunalComposition::ThreeArbitrators.arbitrator_count(), 3);
        assert_eq!(TribunalComposition::ExpeditedSingle.arbitrator_count(), 1);
    }

    #[test]
    fn test_cost_types() {
        assert!(ArbitrationCostType::FilingFee.can_request_waiver());
        assert!(!ArbitrationCostType::TranslationFee.can_request_waiver());
    }
}
