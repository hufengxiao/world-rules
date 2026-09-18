//! 露与霜的形成
//!
//! 夜晚水汽凝结成露、低温凝华成霜的物态变化

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DewFrostFormationRules,
    name: "露与霜的形成",
    desc: "夜晚水汽凝结成露、低温凝华成霜的物态变化",
    origin: "中国",
    tags: ["科学", "气象", "露", "霜"]
}

impl DewFrostFormationRules {
    /// 形成条件
    pub fn condition(&self) -> Vec<&'static str> {
        vec![
            "夜间气温下降",
            "空气水汽饱和",
            "地面物体散热降温",
            "晴朗无风气稳",
        ]
    }

    /// 露与霜区别
    pub fn difference(&self) -> Vec<&'static str> {
        vec!["冰点以上凝成露", "冰点以下结成霜", "露是露滴", "霜是冰晶"]
    }

    /// 常见位置
    pub fn location(&self) -> Vec<&'static str> {
        vec![
            "草叶叶片结露",
            "金属面易结露",
            "车窗玻璃挂露",
            "朝北处霜常留",
        ]
    }

    /// 现象意义
    pub fn meaning(&self) -> Vec<&'static str> {
        vec![
            "反映夜间晴稳",
            "霜冻伤农作物",
            "露水滋润植被",
            "古时农时参考",
        ]
    }
}

impl Rule for DewFrostFormationRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("dew_frost")
    }

    fn explain(&self) -> String {
        format!(
            "【露与霜的形成】\n{}",
            [
                format!(
                    "形成条件：\\n{}",
                    self.condition()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "露与霜区别：\\n{}",
                    self.difference()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "常见位置：\\n{}",
                    self.location()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "现象意义：\\n{}",
                    self.meaning()
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
    fn test_dewfrostformationrules_basic() {
        let rules = DewFrostFormationRules::new();
        assert_eq!(rules.metadata().name, "露与霜的形成");
        assert!(!rules.condition().is_empty());
        assert!(!rules.difference().is_empty());
        assert!(!rules.location().is_empty());
        assert!(!rules.meaning().is_empty());
    }

    #[test]
    fn test_dewfrostformationrules_validation() {
        let rules = DewFrostFormationRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::science("dew_frost"));
    }

    #[test]
    fn test_dewfrostformationrules_explain() {
        let rules = DewFrostFormationRules::new();
        let e = rules.explain();
        assert!(e.contains("形成条件"));
        assert!(e.contains("露与霜区别"));
        assert!(e.contains("常见位置"));
    }
}
