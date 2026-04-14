//! 外商投资法基础规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 外商投资法规则
pub struct ForeignInvestmentLawRules {
    metadata: RuleMetadata,
}

impl ForeignInvestmentLawRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "外商投资法规则",
                "中国外商投资法基础知识"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "外商投资".into()]),
        }
    }

    /// 外商投资原则
    pub fn investment_principles(&self) -> Vec<&'static str> {
        vec![
            "开放原则: 投资开放",
            "公平原则: 公平待遇",
            "透明原则: 透明管理",
            "法治原则:依法管理",
            "安全原则: 国家安全",
            "保护原则: 权益保护",
            "促进原则: 投资促进",
            "服务原则: 投资服务",
        ]
    }

    /// 外商投资形式
    pub fn investment_forms(&self) -> Vec<&'static str> {
        vec![
            "外商独资企业",
            "中外合资企业",
            "中外合作企业",
            "外商投资股份公司",
            "外商投资合伙企业",
            "外商投资项目投资",
            "外商并购投资",
            "外商再投资",
        ]
    }

    /// 外商投资准入
    pub fn investment_access(&self) -> Vec<&'static str> {
        vec![
            "负面清单管理制度",
            "禁止投资领域",
            "限制投资领域",
            "许可投资领域",
            "准入审批程序",
            "准入备案程序",
            "准入条件要求",
            "准入信息报告",
        ]
    }

    /// 外商投资者权利
    pub fn investor_rights(&self) -> Vec<&'static str> {
        vec![
            "公平竞争权利",
            "平等保护权利",
            "征收补偿权利",
            "知识产权保护",
            "参与标准制定",
            "政府采购参与",
            "政策公平待遇",
            "救济申诉权利",
        ]
    }

    /// 外商投资管理
    pub fn investment_management(&self) -> Vec<&'static str> {
        vec![
            "信息报告制度",
            "投资信息报送",
            "投资变更登记",
            "投资年度报告",
            "投资监督检查",
            "投资安全审查",
            "投资信用管理",
            "投资统计制度",
        ]
    }

    /// 外商投资促进
    pub fn investment_promotion(&self) -> Vec<&'static str> {
        vec![
            "投资优惠政策",
            "投资服务措施",
            "投资便利措施",
            "投资引导服务",
            "投资信息服务",
            "投资咨询服务",
            "投资投诉受理",
            "投资协调机制",
        ]
    }

    /// 外商投资保护
    pub fn investment_protection(&self) -> Vec<&'static str> {
        vec![
            "征收征用保护",
            "财产权利保护",
            "知识产权保护",
            "商业秘密保护",
            "技术合作保护",
            "政策稳定性保护",
            "行政行为监督",
            "投诉救济机制",
        ]
    }

    /// 外商投资违法行为
    pub fn investment_violations(&self) -> Vec<&'static str> {
        vec![
            "禁止领域投资",
            "限制领域违规投资",
            "虚假投资信息报告",
            "逃避安全审查",
            "违反准入条件",
            "违反管理规定",
            "侵犯知识产权",
            "不正当竞争行为",
        ]
    }
}

impl Default for ForeignInvestmentLawRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ForeignInvestmentLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("foreign_investment")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【外商投资法规则】\n\n投资原则:\n{}\n\n投资形式:\n{}\n\n投资准入:\n{}\n",
            self.investment_principles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.investment_forms().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.investment_access().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foreign_investment_law_rules() {
        let rules = ForeignInvestmentLawRules::new();
        assert!(!rules.investment_principles().is_empty());
        assert!(!rules.investment_forms().is_empty());
    }
}
