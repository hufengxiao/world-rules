//! 可再生能源常识
//!
//! 太阳能风能水能等可再生能源的原理与优势

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RenewableEnergyBasicsRules,
    name: "可再生能源常识",
    desc: "太阳能风能水能等可再生能源的原理与优势",
    origin: "国际",
    tags: ["科学", "能源", "可再生", "环保"]
}

impl RenewableEnergyBasicsRules {
    /// 常见类型
    pub fn types(&self) -> Vec<&'static str> {
        vec![
            "太阳能光伏与热利用",
            "风力涡轮发电",
            "水力电站发电",
            "生物质与地热也是资源",
        ]
    }

    /// 工作原理
    pub fn principle(&self) -> Vec<&'static str> {
        vec![
            "光伏将光能转电能",
            "风机转动能发电",
            "水位落差转化为电能",
            "地热利用地下热能",
        ]
    }

    /// 优势与挑战
    pub fn pros_cons(&self) -> Vec<&'static str> {
        vec![
            "资源可再生碳排放低",
            "天气与地理限制影响",
            "储能需要配套",
            "成本随技术进步下降",
        ]
    }

    /// 科学生活
    pub fn daily(&self) -> Vec<&'static str> {
        vec![
            "日常节约能源",
            "关注用能结构与效率",
            "合理使用家用电器",
            "支持绿色低碳方式",
        ]
    }
}

impl Rule for RenewableEnergyBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("renewable_energy")
    }

    fn explain(&self) -> String {
        format!(
            "【可再生能源常识】\n{}",
            [
                format!(
                    "常见类型：\\n{}",
                    self.types()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "工作原理：\\n{}",
                    self.principle()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "优势与挑战：\\n{}",
                    self.pros_cons()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "科学生活：\\n{}",
                    self.daily()
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
    fn test_renewableenergybasicsrules_basic() {
        let rules = RenewableEnergyBasicsRules::new();
        assert_eq!(rules.metadata().name, "可再生能源常识");
        assert!(!rules.types().is_empty());
        assert!(!rules.principle().is_empty());
        assert!(!rules.pros_cons().is_empty());
        assert!(!rules.daily().is_empty());
    }

    #[test]
    fn test_renewableenergybasicsrules_validation() {
        let rules = RenewableEnergyBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("renewable_energy"));
    }

    #[test]
    fn test_renewableenergybasicsrules_explain() {
        let rules = RenewableEnergyBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("常见类型"));
        assert!(e.contains("工作原理"));
        assert!(e.contains("优势与挑战"));
    }
}
