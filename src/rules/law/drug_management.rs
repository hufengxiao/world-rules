//! 药品管理法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 药品管理法规则
pub struct DrugManagementLawRules {
    metadata: RuleMetadata,
}

impl DrugManagementLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "药品管理法规则",
                "中国药品管理法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "药品管理".into()]),
        }
    }

    /// 药品研制规则
    pub fn drug_research(&self) -> Vec<&'static str> {
        vec![
            "药品研制原则",
            "临床试验管理",
            "药品注册申请",
            "药品审评审批",
            "研制资料管理",
            "研制伦理审查",
            "研制信息公开",
            "研制监管机制",
        ]
    }

    /// 药品生产规则
    pub fn drug_production(&self) -> Vec<&'static str> {
        vec![
            "药品生产许可",
            "生产质量管理规范",
            "生产过程控制",
            "生产记录管理",
            "生产检验要求",
            "生产环境管理",
            "生产变更管理",
            "生产监督检查",
        ]
    }

    /// 药品经营规则
    pub fn drug_distribution(&self) -> Vec<&'static str> {
        vec![
            "药品经营许可",
            "经营质量管理规范",
            "药品采购管理",
            "药品销售管理",
            "药品储存管理",
            "药品运输管理",
            "药品追溯体系",
            "经营监督检查",
        ]
    }

    /// 药品使用规则
    pub fn drug_use(&self) -> Vec<&'static str> {
        vec![
            "医疗机构用药管理",
            "处方药使用管理",
            "非处方药使用管理",
            "特殊药品使用管理",
            "药品调配管理",
            "药品不良反应监测",
            "用药安全管理",
            "用药指导服务",
        ]
    }

    /// 特殊药品管理
    pub fn special_drug_management(&self) -> Vec<&'static str> {
        vec![
            "麻醉药品管理",
            "精神药品管理",
            "医疗用毒性药品管理",
            "放射性药品管理",
            "特殊药品许可",
            "特殊药品储存",
            "特殊药品使用",
            "特殊药品追溯",
        ]
    }

    /// 药品不良反应
    pub fn adverse_reaction(&self) -> Vec<&'static str> {
        vec![
            "不良反应监测报告",
            "不良反应评价分析",
            "不良反应信息发布",
            "不良反应处理措施",
            "不良反应预防措施",
            "不良反应责任追究",
            "不良反应补偿机制",
            "不良反应预警机制",
        ]
    }

    /// 药品监督管理
    pub fn drug_supervision(&self) -> Vec<&'static str> {
        vec![
            "药品监督检查",
            "药品抽检检验",
            "药品信息发布",
            "药品风险评估",
            "药品召回管理",
            "药品处罚措施",
            "药品信用管理",
            "药品举报处理",
        ]
    }

    /// 药品违法行为
    pub fn drug_violations(&self) -> Vec<&'static str> {
        vec![
            "生产销售假药行为",
            "生产销售劣药行为",
            "无证生产经营药品",
            "违规生产药品行为",
            "违规经营药品行为",
            "违规使用药品行为",
            "伪造药品资料行为",
            "违法特殊药品行为",
        ]
    }
}

impl Default for DrugManagementLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DrugManagementLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("drug_management")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【药品管理法规则】\n\n药品研制:\n{}\n\n药品生产:\n{}\n\n药品经营:\n{}\n",
            self.drug_research().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.drug_production().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.drug_distribution().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drug_management_law_rules() {
        let rules = DrugManagementLawRules::new();
        assert!(!rules.drug_research().is_empty());
        assert!(!rules.drug_production().is_empty());
    }
}
