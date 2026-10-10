//! 姻亲相处礼仪
//!
//! 婆媳翁婿等姻亲之间的相处分寸与礼节

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InLawRelationshipEtiquetteRules,
    name: "姻亲相处礼仪",
    desc: "婆媳翁婿等姻亲之间的相处分寸与礼节",
    origin: "中国",
    tags: ["家庭", "关系", "礼仪"]
}

impl InLawRelationshipEtiquetteRules {
    /// 晚辈之道
    pub fn junior_duty(&self) -> Vec<&'static str> {
        vec![
            "尊敬长辈多问候",
            "逢年过节探望",
            "礼数周全不怠慢",
            "有分歧委婉沟通",
        ]
    }

    /// 长辈之度
    pub fn senior_grace(&self) -> Vec<&'static str> {
        vec![
            "尊重小家庭界限",
            "少干涉子女生活",
            "不偏袒不挑剔",
            "多包容少计较",
        ]
    }

    /// 相处分寸
    pub fn boundaries(&self) -> Vec<&'static str> {
        vec![
            "保持适度距离感",
            "有事商量不擅作主张",
            "避免当面指责",
            "经济往来账目清楚",
        ]
    }

    /// 关系维护
    pub fn maintenance(&self) -> Vec<&'static str> {
        vec!["多念对方好处", "矛盾私下化解", "不搬弄是非", "以和睦为贵"]
    }
}

impl Rule for InLawRelationshipEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("in_law_relationship_etiquette")
    }

    fn explain(&self) -> String {
        format!(
            "【姻亲相处礼仪】\n{}",
            [
                format!(
                    "晚辈之道：\\n{}",
                    self.junior_duty()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "长辈之度：\\n{}",
                    self.senior_grace()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "相处分寸：\\n{}",
                    self.boundaries()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "关系维护：\\n{}",
                    self.maintenance()
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
    fn test_inlawrelationshipetiquetterules_basic() {
        let rules = InLawRelationshipEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "姻亲相处礼仪");
        assert!(!rules.junior_duty().is_empty());
        assert!(!rules.senior_grace().is_empty());
        assert!(!rules.boundaries().is_empty());
        assert!(!rules.maintenance().is_empty());
    }

    #[test]
    fn test_inlawrelationshipetiquetterules_validation() {
        let rules = InLawRelationshipEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("in_law_relationship_etiquette")
        );
    }

    #[test]
    fn test_inlawrelationshipetiquetterules_explain() {
        let rules = InLawRelationshipEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("晚辈之道"));
        assert!(e.contains("长辈之度"));
        assert!(e.contains("相处分寸"));
    }
}
