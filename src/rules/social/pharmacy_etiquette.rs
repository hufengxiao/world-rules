//! 药店购药礼仪
//!
//! 药店咨询、购药与用药咨询时的秩序与礼貌

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PharmacyEtiquetteRules,
    name: "药店购药礼仪",
    desc: "药店咨询、购药与用药咨询时的秩序与礼貌",
    origin: "国际",
    tags: ["社交", "礼仪", "药店", "购药", "用药"]
}

impl PharmacyEtiquetteRules {
    /// 有序购药
    pub fn queue(&self) -> Vec<&'static str> {
        vec![
            "排队等待购药咨询",
            "不高声插队",
            "尊重工作人员按序接待",
            "人多时耐心等候",
        ]
    }

    /// 清晰说明
    pub fn specify(&self) -> Vec<&'static str> {
        vec![
            "说明症状或用药需求",
            "告知既往病史与药物过敏",
            "说不清时咨询药师建议",
            "核对药品名称与剂量",
        ]
    }

    /// 遵守用药建议
    pub fn follow(&self) -> Vec<&'static str> {
        vec![
            "尊重药师的专业建议",
            "遵医嘱说明书中用量",
            "不擅自超量使用",
            "有疑问及时复询",
        ]
    }

    /// 心怀尊重
    pub fn respect(&self) -> Vec<&'static str> {
        vec![
            "尊重工作人员辛勤",
            "不无理催促或责难",
            "购药后致谢",
            "特殊用药咨询医核实",
        ]
    }
}

impl Rule for PharmacyEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("pharmacy")
    }

    fn explain(&self) -> String {
        format!(
            "【药店购药礼仪】\n{}",
            [
                format!(
                    "有序购药：\\n{}",
                    self.queue()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "清晰说明：\\n{}",
                    self.specify()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "遵守用药建议：\\n{}",
                    self.follow()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "心怀尊重：\\n{}",
                    self.respect()
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
    fn test_pharmacyetiquetterules_basic() {
        let rules = PharmacyEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "药店购药礼仪");
        assert!(!rules.queue().is_empty());
        assert!(!rules.specify().is_empty());
        assert!(!rules.follow().is_empty());
        assert!(!rules.respect().is_empty());
    }

    #[test]
    fn test_pharmacyetiquetterules_validation() {
        let rules = PharmacyEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("pharmacy"));
    }

    #[test]
    fn test_pharmacyetiquetterules_explain() {
        let rules = PharmacyEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("有序购药"));
        assert!(e.contains("清晰说明"));
        assert!(e.contains("遵守用药建议"));
    }
}
