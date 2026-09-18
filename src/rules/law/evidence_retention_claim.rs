//! 证据保存与举证
//!
//! 打官司维权时收集保存证据与承担举证责任的要点

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: EvidenceRetentionClaimRules,
    name: "证据保存与举证",
    desc: "打官司维权时收集保存证据与承担举证责任的要点",
    origin: "中国",
    tags: ["法律", "证据", "举证", "诉讼"]
}

impl EvidenceRetentionClaimRules {
    /// 证据种类
    pub fn types(&self) -> Vec<&'static str> {
        vec![
            "书证契约合同",
            "物证实物",
            "视听电子记录",
            "证人证言勘验笔录",
        ]
    }

    /// 收集保存
    pub fn collect(&self) -> Vec<&'static str> {
        vec![
            "纠纷即收凭证",
            "合同票据留存",
            "聊天转账截图",
            "原物原件妥保管",
        ]
    }

    /// 举证原则
    pub fn burden(&self) -> Vec<&'static str> {
        vec![
            "谁主张谁举证",
            "证据充分有力",
            "缺陷证据难采信",
            "及时提交不拖延",
        ]
    }

    /// 证据效力
    pub fn effect(&self) -> Vec<&'static str> {
        vec![
            "真实性完整性",
            "合法取得才有效",
            "原始证据优先",
            "必要时公证保全",
        ]
    }
}

impl Rule for EvidenceRetentionClaimRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("evidence_retention")
    }

    fn explain(&self) -> String {
        format!(
            "【证据保存与举证】\n{}",
            [
                format!(
                    "证据种类：\\n{}",
                    self.types()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "收集保存：\\n{}",
                    self.collect()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "举证原则：\\n{}",
                    self.burden()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "证据效力：\\n{}",
                    self.effect()
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
    fn test_evidenceretentionclaimrules_basic() {
        let rules = EvidenceRetentionClaimRules::new();
        assert_eq!(rules.metadata().name, "证据保存与举证");
        assert!(!rules.types().is_empty());
        assert!(!rules.collect().is_empty());
        assert!(!rules.burden().is_empty());
        assert!(!rules.effect().is_empty());
    }

    #[test]
    fn test_evidenceretentionclaimrules_validation() {
        let rules = EvidenceRetentionClaimRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("evidence_retention"));
    }

    #[test]
    fn test_evidenceretentionclaimrules_explain() {
        let rules = EvidenceRetentionClaimRules::new();
        let e = rules.explain();
        assert!(e.contains("证据种类"));
        assert!(e.contains("收集保存"));
        assert!(e.contains("举证原则"));
    }
}
