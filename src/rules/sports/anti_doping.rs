//! 反兴奋剂规则
//!
//! 基于 WADA（世界反兴奋剂机构）标准，包含禁用物质清单、检查程序、违规处罚等。
//!
//! # 规则体系
//!
//! - WADA 世界反兴奋剂条例
//! - 国际单项体育组织反兴奋剂规则
//! - 国家反兴奋剂条例
//!
//! # Examples
//!
//! ```
//! use world_rules::rules::sports::anti_doping::AntiDopingRules;
//! use world_rules::rules::core::Rule;
//!
//! let rules = AntiDopingRules::new();
//! assert!(!rules.prohibited_substances_list().is_empty());
//! assert!(!rules.doping_control_procedures().is_empty());
//! ```

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 禁用物质类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProhibitedSubstanceType {
    /// 蛋白同化制剂（类固醇）
    AnabolicAgents,
    /// 肽类激素及相关物质
    PeptideHormones,
    /// β2-激动剂
    Beta2Agonists,
    /// 激素及代谢调节剂
    HormoneModulators,
    /// 利尿剂和掩蔽剂
    Diuretics,
    /// 兴奋剂
    Stimulants,
    /// 麻醉剂
    Narcotics,
    /// 大麻素
    Cannabinoids,
    /// 糖皮质激素
    Glucocorticoids,
    /// β-受体阻滞剂
    BetaBlockers,
}

impl ProhibitedSubstanceType {
    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            ProhibitedSubstanceType::AnabolicAgents => "蛋白同化制剂",
            ProhibitedSubstanceType::PeptideHormones => "肽类激素",
            ProhibitedSubstanceType::Beta2Agonists => "β2-激动剂",
            ProhibitedSubstanceType::HormoneModulators => "激素调节剂",
            ProhibitedSubstanceType::Diuretics => "利尿剂",
            ProhibitedSubstanceType::Stimulants => "兴奋剂",
            ProhibitedSubstanceType::Narcotics => "麻醉剂",
            ProhibitedSubstanceType::Cannabinoids => "大麻素",
            ProhibitedSubstanceType::Glucocorticoids => "糖皮质激素",
            ProhibitedSubstanceType::BetaBlockers => "β-受体阻滞剂",
        }
    }

    /// 获取禁用场合
    pub fn prohibition_context(&self) -> ProhibitionContext {
        match self {
            ProhibitedSubstanceType::AnabolicAgents
            | ProhibitedSubstanceType::PeptideHormones
            | ProhibitedSubstanceType::HormoneModulators
            | ProhibitedSubstanceType::Diuretics => ProhibitionContext::AllTimes,
            ProhibitedSubstanceType::Beta2Agonists => ProhibitionContext::InCompetition,
            ProhibitedSubstanceType::Stimulants
            | ProhibitedSubstanceType::Narcotics
            | ProhibitedSubstanceType::Cannabinoids
            | ProhibitedSubstanceType::Glucocorticoids => ProhibitionContext::InCompetition,
            ProhibitedSubstanceType::BetaBlockers => ProhibitionContext::InCompetition,
        }
    }
}

/// 禁用场合
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProhibitionContext {
    /// 所有时间禁用（赛内和赛外）
    AllTimes,
    /// 仅赛内禁用
    InCompetition,
    /// 特定运动项目禁用
    SpecificSports,
}

/// 违规类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AntiDopingViolationType {
    /// 使用或企图使用禁用物质/方法
    Use,
    /// 拒绝或未完成样本采集
    Refusal,
    /// 疏忽未报备行踪
    WhereaboutsFailure,
    /// 篡改或企图篡改样本
    Tampering,
    /// 持有禁用物质/方法
    Possession,
    /// 贩运禁用物质/方法
    Trafficking,
    /// 施用或企图施用禁用物质/方法
    Administration,
    /// 禁止协作
    ProhibitedAssociation,
    /// 妨碍兴奋剂管制
    Complicity,
}

impl AntiDopingViolationType {
    /// 获取违规类型名称
    pub fn name(&self) -> &'static str {
        match self {
            AntiDopingViolationType::Use => "使用禁用物质/方法",
            AntiDopingViolationType::Refusal => "拒绝样本采集",
            AntiDopingViolationType::WhereaboutsFailure => "行踪报备违规",
            AntiDopingViolationType::Tampering => "篡改样本",
            AntiDopingViolationType::Possession => "持有禁用物质",
            AntiDopingViolationType::Trafficking => "贩运禁用物质",
            AntiDopingViolationType::Administration => "施用禁用物质",
            AntiDopingViolationType::ProhibitedAssociation => "禁止协作",
            AntiDopingViolationType::Complicity => "妨碍兴奋剂管制",
        }
    }

    /// 获取基准禁赛期（月）
    pub fn base_suspension_months(&self) -> u32 {
        match self {
            AntiDopingViolationType::Use => 48,        // 4年
            AntiDopingViolationType::Refusal => 48,
            AntiDopingViolationType::WhereaboutsFailure => 12, // 12个月
            AntiDopingViolationType::Tampering => 48,
            AntiDopingViolationType::Possession => 48,
            AntiDopingViolationType::Trafficking => 48,
            AntiDopingViolationType::Administration => 48,
            AntiDopingViolationType::ProhibitedAssociation => 24,
            AntiDopingViolationType::Complicity => 24,
        }
    }
}

/// 兴奋剂检查程序阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DopingControlPhase {
    /// 通知运动员
    Notification,
    /// 样本采集
    SampleCollection,
    /// 样本处理
    SampleProcessing,
    /// 样本运输
    SampleTransport,
    /// 实验室分析
    LaboratoryAnalysis,
    /// 结果管理
    ResultsManagement,
}

impl DopingControlPhase {
    /// 获取阶段名称
    pub fn name(&self) -> &'static str {
        match self {
            DopingControlPhase::Notification => "通知阶段",
            DopingControlPhase::SampleCollection => "样本采集",
            DopingControlPhase::SampleProcessing => "样本处理",
            DopingControlPhase::SampleTransport => "样本运输",
            DopingControlPhase::LaboratoryAnalysis => "实验室分析",
            DopingControlPhase::ResultsManagement => "结果管理",
        }
    }
}

/// 治疗用药豁免（TUE）状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TUEStatus {
    /// 已批准
    Approved,
    /// 待审核
    Pending,
    /// 已拒绝
    Rejected,
    /// 已撤销
    Revoked,
}

/// 反兴奋剂规则
pub struct AntiDopingRules {
    metadata: RuleMetadata,
}

impl AntiDopingRules {
    /// 创建新的反兴奋剂规则实例
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("反兴奋剂规则", "WADA 世界反兴奋剂条例框架")
                .with_origin("世界反兴奋剂机构(WADA)")
                .with_tags(vec![
                    "体育".into(),
                    "反兴奋剂".into(),
                    "禁用物质".into(),
                    "兴奋剂检查".into(),
                ]),
        }
    }

    /// 禁用物质清单（主要类别）
    pub fn prohibited_substances_list(&self) -> Vec<ProhibitedSubstanceType> {
        vec![
            ProhibitedSubstanceType::AnabolicAgents,
            ProhibitedSubstanceType::PeptideHormones,
            ProhibitedSubstanceType::Beta2Agonists,
            ProhibitedSubstanceType::HormoneModulators,
            ProhibitedSubstanceType::Diuretics,
            ProhibitedSubstanceType::Stimulants,
            ProhibitedSubstanceType::Narcotics,
            ProhibitedSubstanceType::Cannabinoids,
            ProhibitedSubstanceType::Glucocorticoids,
            ProhibitedSubstanceType::BetaBlockers,
        ]
    }

    /// 禁用方法清单
    pub fn prohibited_methods(&self) -> Vec<&'static str> {
        vec![
            "提高氧气转运能力（如血液兴奋剂）",
            "化学和物理篡改（如样本篡改）",
            "基因和细胞兴奋剂",
            "血液回输",
            "人工提升摄氧能力",
        ]
    }

    /// 兴奋剂检查程序
    pub fn doping_control_procedures(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("赛前检查", "在比赛当天采集样本"),
            ("赛外检查", "非比赛期间突击检查"),
            ("行踪报备", "运动员提供每日1小时可接受检查时段"),
            ("样本采集", "采集尿液和/或血液样本"),
            ("样本分割", "将样本分为A瓶和B瓶"),
            ("实验室分析", "WADA认证实验室分析A瓶样本"),
            ("结果通知", "向运动员通知分析结果"),
        ]
    }

    /// 样本采集要求
    pub fn sample_collection_requirements(&self) -> Vec<&'static str> {
        vec![
            "检查官必须出示有效授权证明",
            "运动员有权要求核实检查官身份",
            "采集过程须在监督下进行",
            "样本必须密封并正确标记",
            "运动员确认样本信息并签名",
            "样本必须在规定时间内送达实验室",
            "全程记录采集过程",
        ]
    }

    /// 治疗用药豁免（TUE）规则
    pub fn tue_rules(&self) -> Vec<&'static str> {
        vec![
            "TUE申请必须提前提交（紧急情况除外）",
            "必须证明治疗必要性",
            "无其他允许的替代治疗方法",
            "不会对公平竞争产生额外优势",
            "必须获得反兴奋剂组织批准",
            "TUE有效期有限，需要续期",
            "TUE可以被撤销或拒绝",
        ]
    }

    /// 违规处罚原则
    pub fn sanction_principles(&self) -> Vec<&'static str> {
        vec![
            "禁赛期根据违规类型和严重程度确定",
            "首次违规通常禁赛4年（故意违规）",
            "非故意违规可减少禁赛期",
            "第二次违规禁赛期加倍",
            "第三次违规可能终身禁赛",
            "可考虑提供实质性协助而减少禁赛期",
            "未成年人适用特殊保护条款",
        ]
    }

    /// 计算违规禁赛期
    pub fn calculate_suspension(&self, violation: AntiDopingViolationType, is_first_violation: bool, is_intentional: bool) -> u32 {
        let base = violation.base_suspension_months();
        
        if !is_first_violation {
            // 第二次违规加倍
            return base * 2;
        }
        
        if !is_intentional {
            // 非故意违规可减半
            return base / 2;
        }
        
        base
    }

    /// 运动员权利
    pub fn athlete_rights(&self) -> Vec<&'static str> {
        vec![
            "有权要求核实检查官身份",
            "有权选择采集容器",
            "有权要求同性检查官",
            "有权获得样本副本",
            "有权在B瓶样本分析时在场",
            "有权申请治疗用药豁免",
            "有权对违规裁决提出上诉",
            "有权获得法律代表协助",
        ]
    }

    /// 运动员义务
    pub fn athlete_obligations(&self) -> Vec<&'static str> {
        vec![
            "不得使用禁用物质和方法",
            "准确报备每日行踪信息",
            "接受赛内和赛外检查",
            "配合样本采集程序",
            "妥善保管个人物品",
            "及时申请治疗用药豁免",
            "检查时提供有效身份证件",
        ]
    }

    /// 行踪报备要求
    pub fn whereabouts_requirements(&self) -> Vec<&'static str> {
        vec![
            "每日提供至少1小时可接受检查时段",
            "每季度提交行踪信息",
            "及时更新变更信息",
            "提供详细地址和联系方式",
            "在指定地点等待检查",
            "错过三次检查构成违规",
        ]
    }

    /// 实验室认证要求
    pub fn laboratory_requirements(&self) -> Vec<&'static str> {
        vec![
            "必须获得WADA认证",
            "遵守ISO 17025标准",
            "定期参加能力验证",
            "使用WADA批准的分析方法",
            "保持样本链完整",
            "严格保护运动员隐私",
            "及时报告分析结果",
        ]
    }

    /// 结果管理程序
    pub fn results_management_procedures(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("初步审查", "确认样本完整性和分析有效性"),
            ("通知运动员", "告知检测结果和后续程序"),
            ("B瓶分析", "运动员有权要求分析B瓶样本"),
            ("听证会", "运动员可参加听证会并陈述"),
            ("裁决", "反兴奋剂组织做出最终裁决"),
            ("上诉", "运动员可向体育仲裁法院上诉"),
        ]
    }

    /// 教育和预防措施
    pub fn education_measures(&self) -> Vec<&'static str> {
        vec![
            "反兴奋剂教育课程",
            "禁用物质清单培训",
            "治疗用药豁免申请指导",
            "营养补充剂风险评估",
            "检查程序培训",
            "运动员支持人员教育",
            "青少年反兴奋剂宣传",
        ]
    }

    /// 特殊人群保护措施
    pub fn protected_person_rules(&self) -> Vec<&'static str> {
        vec![
            "未成年人适用减轻处罚条款",
            "需要监护人参与检查程序",
            "教育优先于处罚",
            "保护隐私信息",
            "提供特殊心理支持",
            "康复后可申请重返赛场",
        ]
    }

    /// 验证物质是否禁用
    pub fn is_substance_prohibited(&self, substance_type: ProhibitedSubstanceType, is_in_competition: bool) -> bool {
        match substance_type.prohibition_context() {
            ProhibitionContext::AllTimes => true,
            ProhibitionContext::InCompetition => is_in_competition,
            ProhibitionContext::SpecificSports => true, // 需要具体运动项目判断
        }
    }
}

impl Default for AntiDopingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AntiDopingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("anti_doping")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        // 反兴奋剂规则验证逻辑
        Ok(true)
    }

    fn explain(&self) -> String {
        let mut explanation = vec![
            "=== 反兴奋剂规则 ===".to_string(),
            "".to_string(),
            "本规则基于 WADA 世界反兴奋剂条例，涵盖：".to_string(),
            "1. 禁用物质和方法清单".to_string(),
            "2. 兴奋剂检查程序".to_string(),
            "3. 违规类型和处罚".to_string(),
            "4. 治疗用药豁免".to_string(),
            "5. 运动员权利和义务".to_string(),
        ];

        explanation.push(String::new());
        explanation.push("禁用物质类别:".to_string());
        for substance in self.prohibited_substances_list() {
            explanation.push(format!("  - {}", substance.name()));
        }

        explanation.push(String::new());
        explanation.push("禁用方法:".to_string());
        for method in self.prohibited_methods() {
            explanation.push(format!("  - {}", method));
        }

        explanation.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anti_doping_rules_creation() {
        let rules = AntiDopingRules::new();
        assert!(!rules.prohibited_substances_list().is_empty());
        assert!(!rules.prohibited_methods().is_empty());
    }

    #[test]
    fn test_prohibited_substance_types() {
        assert_eq!(
            ProhibitedSubstanceType::AnabolicAgents.name(),
            "蛋白同化制剂"
        );
        assert_eq!(
            ProhibitedSubstanceType::Stimulants.name(),
            "兴奋剂"
        );
    }

    #[test]
    fn test_prohibition_context() {
        // 所有时间禁用
        assert_eq!(
            ProhibitedSubstanceType::AnabolicAgents.prohibition_context(),
            ProhibitionContext::AllTimes
        );
        
        // 仅赛内禁用
        assert_eq!(
            ProhibitedSubstanceType::Stimulants.prohibition_context(),
            ProhibitionContext::InCompetition
        );
    }

    #[test]
    fn test_violation_types() {
        assert_eq!(AntiDopingViolationType::Use.name(), "使用禁用物质/方法");
        assert_eq!(AntiDopingViolationType::Use.base_suspension_months(), 48);
        assert_eq!(AntiDopingViolationType::WhereaboutsFailure.base_suspension_months(), 12);
    }

    #[test]
    fn test_suspension_calculation() {
        let rules = AntiDopingRules::new();
        
        // 首次故意违规 - 4年
        let suspension = rules.calculate_suspension(
            AntiDopingViolationType::Use,
            true,
            true
        );
        assert_eq!(suspension, 48);
        
        // 非故意违规 - 减半
        let suspension = rules.calculate_suspension(
            AntiDopingViolationType::Use,
            true,
            false
        );
        assert_eq!(suspension, 24);
        
        // 第二次违规 - 加倍
        let suspension = rules.calculate_suspension(
            AntiDopingViolationType::Use,
            false,
            true
        );
        assert_eq!(suspension, 96);
    }

    #[test]
    fn test_is_substance_prohibited() {
        let rules = AntiDopingRules::new();
        
        // 所有时间禁用的物质
        assert!(rules.is_substance_prohibited(
            ProhibitedSubstanceType::AnabolicAgents,
            false
        ));
        
        // 赛内禁用的物质 - 赛外不禁用
        assert!(!rules.is_substance_prohibited(
            ProhibitedSubstanceType::Stimulants,
            false
        ));
        
        // 赛内禁用的物质 - 赛内禁用
        assert!(rules.is_substance_prohibited(
            ProhibitedSubstanceType::Stimulants,
            true
        ));
    }

    #[test]
    fn test_doping_control_procedures() {
        let rules = AntiDopingRules::new();
        let procedures = rules.doping_control_procedures();
        assert!(!procedures.is_empty());
        
        // 检查包含关键程序
        let has_sample_collection = procedures.iter().any(|(name, _)| *name == "样本采集");
        assert!(has_sample_collection);
    }

    #[test]
    fn test_athlete_rights_and_obligations() {
        let rules = AntiDopingRules::new();
        
        // 权利和义务列表不应为空
        assert!(!rules.athlete_rights().is_empty());
        assert!(!rules.athlete_obligations().is_empty());
        
        // 检查关键权利
        let rights = rules.athlete_rights();
        let has_bottle_rights = rights.iter().any(|r| r.contains("B瓶"));
        assert!(has_bottle_rights);
    }

    #[test]
    fn test_tue_rules() {
        let rules = AntiDopingRules::new();
        let tue_rules = rules.tue_rules();
        assert!(!tue_rules.is_empty());
        
        // 检查关键规则
        let has_advance_submit = tue_rules.iter().any(|r| r.contains("提前提交"));
        assert!(has_advance_submit);
    }

    #[test]
    fn test_sample_collection_requirements() {
        let rules = AntiDopingRules::new();
        let requirements = rules.sample_collection_requirements();
        assert!(requirements.len() >= 5);
        
        // 检查关键要求
        let has_sealing = requirements.iter().any(|r| r.contains("密封"));
        assert!(has_sealing);
    }

    #[test]
    fn test_education_measures() {
        let rules = AntiDopingRules::new();
        let measures = rules.education_measures();
        assert!(!measures.is_empty());
        
        // 检查包含教育课程
        let has_education = measures.iter().any(|m| m.contains("教育"));
        assert!(has_education);
    }

    #[test]
    fn test_rule_trait_implementation() {
        use crate::rules::core::Rule;
        
        let rules = AntiDopingRules::new();
        
        // 测试 metadata
        let metadata = rules.metadata();
        assert!(metadata.name.contains("兴奋剂"));
        
        // 测试 category
        let category = rules.category();
        assert!(matches!(category, RuleCategory::Sports(_)));
        
        // 测试 explain
        let explanation = rules.explain();
        assert!(!explanation.is_empty());
        assert!(explanation.iter().any(|e| e.contains("WADA")));
    }

    #[test]
    fn test_doping_control_phases() {
        assert_eq!(DopingControlPhase::Notification.name(), "通知阶段");
        assert_eq!(DopingControlPhase::SampleCollection.name(), "样本采集");
        assert_eq!(DopingControlPhase::LaboratoryAnalysis.name(), "实验室分析");
    }

    #[test]
    fn test_whereabouts_requirements() {
        let rules = AntiDopingRules::new();
        let requirements = rules.whereabouts_requirements();
        
        // 检查每日1小时要求
        let has_daily_requirement = requirements.iter().any(|r| r.contains("每日"));
        assert!(has_daily_requirement);
        
        // 检查错过检查次数
        let has_missed_count = requirements.iter().any(|r| r.contains("三次"));
        assert!(has_missed_count);
    }

    #[test]
    fn test_protected_person_rules() {
        let rules = AntiDopingRules::new();
        let protected_rules = rules.protected_person_rules();
        
        assert!(!protected_rules.is_empty());
        
        // 检查未成年人保护
        let has_minor_protection = protected_rules.iter().any(|r| r.contains("未成年人"));
        assert!(has_minor_protection);
    }

    #[test]
    fn test_laboratory_requirements() {
        let rules = AntiDopingRules::new();
        let requirements = rules.laboratory_requirements();
        
        // 检查WADA认证要求
        let has_wada_cert = requirements.iter().any(|r| r.contains("WADA"));
        assert!(has_wada_cert);
        
        // 检查ISO标准
        let has_iso = requirements.iter().any(|r| r.contains("ISO"));
        assert!(has_iso);
    }
}