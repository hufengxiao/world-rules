//! 泡脚养生
//!
//! 温水泡脚促进循环放松的好处与注意事项

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FootSoakWellnessRules,
    name: "泡脚养生",
    desc: "温水泡脚促进循环放松的好处与注意事项",
    origin: "中国",
    tags: ["健康", "泡脚", "养生", "循环"]
}

impl FootSoakWellnessRules {
    /// 基本做法
    pub fn method(&self) -> Vec<&'static str> {
        vec![
            "温水泡脚约四十分钟内",
            "水温适中勿过烫",
            "水面过踝",
            "可按摩足底",
        ]
    }

    /// 养生好处
    pub fn benefit(&self) -> Vec<&'static str> {
        vec!["促进下肢循环", "缓解疲劳", "助眠放松", "暖和足部"]
    }

    /// 注意禁忌
    pub fn caution(&self) -> Vec<&'static str> {
        vec![
            "饭后不就立刻泡",
            "足部破损慎泡",
            "水温过高伤皮肤",
            "时间过长易久静",
        ]
    }

    /// 特殊人群
    pub fn special(&self) -> Vec<&'static str> {
        vec![
            "糖尿病者防烫伤",
            "静脉曲张勿久泡",
            "孕期询问医生",
            "心脏病人适度",
        ]
    }
}

impl Rule for FootSoakWellnessRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("foot_soak")
    }

    fn explain(&self) -> String {
        format!(
            "【泡脚养生】\n{}",
            [
                format!(
                    "基本做法：\\n{}",
                    self.method()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "养生好处：\\n{}",
                    self.benefit()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "注意禁忌：\\n{}",
                    self.caution()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊人群：\\n{}",
                    self.special()
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
    fn test_footsoakwellnessrules_basic() {
        let rules = FootSoakWellnessRules::new();
        assert_eq!(rules.metadata().name, "泡脚养生");
        assert!(!rules.method().is_empty());
        assert!(!rules.benefit().is_empty());
        assert!(!rules.caution().is_empty());
        assert!(!rules.special().is_empty());
    }

    #[test]
    fn test_footsoakwellnessrules_validation() {
        let rules = FootSoakWellnessRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("foot_soak"));
    }

    #[test]
    fn test_footsoakwellnessrules_explain() {
        let rules = FootSoakWellnessRules::new();
        let e = rules.explain();
        assert!(e.contains("基本做法"));
        assert!(e.contains("养生好处"));
        assert!(e.contains("注意禁忌"));
    }
}
