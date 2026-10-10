//! 电竞合同法律
//!
//! 电竞选手职业合同的权利义务与常见纠纷

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EsportsContractLawRules,
    name: "电竞合同法律",
    desc: "电竞选手职业合同的权利义务与常见纠纷",
    origin: "中国",
    tags: ["法律", "电竞", "合同"]
}

impl EsportsContractLawRules {
    /// 合同性质
    pub fn nature(&self) -> Vec<&'static str> {
        vec![
            "选手与俱乐部签职业合同",
            "兼具劳动与经纪属性",
            "未成年人签约需监护人同意",
            "条款须合法公平",
        ]
    }

    /// 主要条款
    pub fn clauses(&self) -> Vec<&'static str> {
        vec![
            "约定薪资奖金与发放",
            "明确训练比赛义务",
            "肖像权与直播分成",
            "合同期限与续约条件",
        ]
    }

    /// 常见纠纷
    pub fn disputes(&self) -> Vec<&'static str> {
        vec![
            "欠薪与奖金不发",
            "转会违约金过高",
            "直播收益分配争议",
            "单方解约责任",
        ]
    }

    /// 维权途径
    pub fn remedy(&self) -> Vec<&'static str> {
        vec![
            "先依合同协商",
            "可申请劳动仲裁",
            "经纪纠纷提起诉讼",
            "保留合同与聊天记录",
        ]
    }
}

impl Rule for EsportsContractLawRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("esports_contract_law")
    }

    fn explain(&self) -> String {
        format!(
            "【电竞合同法律】\n{}",
            [
                format!(
                    "合同性质：\\n{}",
                    self.nature()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "主要条款：\\n{}",
                    self.clauses()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "常见纠纷：\\n{}",
                    self.disputes()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "维权途径：\\n{}",
                    self.remedy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_esportscontractlawrules_basic() {
        let rules = EsportsContractLawRules::new();
        assert_eq!(rules.metadata().name, "电竞合同法律");
        assert!(!rules.nature().is_empty());
        assert!(!rules.clauses().is_empty());
        assert!(!rules.disputes().is_empty());
        assert!(!rules.remedy().is_empty());
    }

    #[test]
    fn test_esportscontractlawrules_validation() {
        let rules = EsportsContractLawRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("esports_contract_law"));
    }

    #[test]
    fn test_esportscontractlawrules_explain() {
        let rules = EsportsContractLawRules::new();
        let e = rules.explain();
        assert!(e.contains("合同性质"));
        assert!(e.contains("主要条款"));
        assert!(e.contains("常见纠纷"));
    }
}
