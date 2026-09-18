//! 养宠守法
//!
//! 养犬登记、遛狗牵绳与环境卫生

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: PetKeepingStandardRules,
    name: "养宠守法",
    desc: "养犬登记、遛狗牵绳与环境卫生",
    origin: "中国",
    tags: ["法律", "养宠", "犬", "规范"]
}

impl PetKeepingStandardRules {
    /// 依法登记
    pub fn register(&self) -> Vec<&'static str> {
        vec!["养犬需登记", "办理犬证", "打疫苗", "依规喂养"]
    }

    /// 遛狗牵绳
    pub fn leash(&self) -> Vec<&'static str> {
        vec!["出门牵绳", "大型犬戴嘴", "避开人群儿童", "看管好犬只"]
    }

    /// 环境卫生
    pub fn hygiene(&self) -> Vec<&'static str> {
        vec!["清理犬粪便", "不污染公共", "高峰错时", "保持整洁"]
    }

    /// 伤人责任
    pub fn liability(&self) -> Vec<&'static str> {
        vec!["犬只伤人担责", "医费应赔", "善后沟通", "依法处理"]
    }
}

impl Rule for PetKeepingStandardRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("pet")
    }

    fn explain(&self) -> String {
        format!(
            "【养宠守法】\n{}",
            [
                format!(
                    "依法登记：\\n{}",
                    self.register()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "遛狗牵绳：\\n{}",
                    self.leash()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环境卫生：\\n{}",
                    self.hygiene()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "伤人责任：\\n{}",
                    self.liability()
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
    fn test_petkeepingstandardrules_basic() {
        let rules = PetKeepingStandardRules::new();
        assert_eq!(rules.metadata().name, "养宠守法");
        assert!(!rules.register().is_empty());
        assert!(!rules.leash().is_empty());
        assert!(!rules.hygiene().is_empty());
        assert!(!rules.liability().is_empty());
    }

    #[test]
    fn test_petkeepingstandardrules_validation() {
        let rules = PetKeepingStandardRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::law("pet"));
    }

    #[test]
    fn test_petkeepingstandardrules_explain() {
        let rules = PetKeepingStandardRules::new();
        let e = rules.explain();
        assert!(e.contains("依法登记"));
        assert!(e.contains("遛狗牵绳"));
        assert!(e.contains("环境卫生"));
    }
}
