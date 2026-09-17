//! 与门卫保安相处
//!
//! 出入小区对保安、门卫的礼貌

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DoormanGreetingCourtesyRules,
    name: "与门卫保安相处",
    desc: "出入小区对保安、门卫的礼貌",
    origin: "中国",
    tags: ["社交", "门卫", "保安", "礼貌"]
}

impl DoormanGreetingCourtesyRules {
    /// 出入问候
    pub fn greeting(&self) -> Vec<&'static str> {
        vec!["进出点头问好", "微笑打招呼", "不甩脸无礼", "尊重劳动"]
    }

    /// 配合门禁
    pub fn entry(&self) -> Vec<&'static str> {
        vec!["配合刷门禁", "访客如实说明", "不麻烦照样", "理解查验"]
    }

    /// 求助帮忙
    pub fn help(&self) -> Vec<&'static str> {
        vec!["提重物可求助", "代取快递道谢", "请求讲清楚", "谢谢辛勤"]
    }

    /// 体恤值守
    pub fn considerate(&self) -> Vec<&'static str> {
        vec!["夜间出入轻些", "不苛责盘问", "互相体谅", "和谐门卫关系"]
    }
}

impl Rule for DoormanGreetingCourtesyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("doorman")
    }

    fn explain(&self) -> String {
        format!(
            "【与门卫保安相处】\n{}",
            [
                format!(
                    "出入问候：\\n{}",
                    self.greeting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "配合门禁：\\n{}",
                    self.entry()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "求助帮忙：\\n{}",
                    self.help()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "体恤值守：\\n{}",
                    self.considerate()
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
    fn test_doormangreetingcourtesyrules_basic() {
        let rules = DoormanGreetingCourtesyRules::new();
        assert_eq!(rules.metadata().name, "与门卫保安相处");
        assert!(!rules.greeting().is_empty());
        assert!(!rules.entry().is_empty());
        assert!(!rules.help().is_empty());
        assert!(!rules.considerate().is_empty());
    }

    #[test]
    fn test_doormangreetingcourtesyrules_validation() {
        let rules = DoormanGreetingCourtesyRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("doorman"));
    }

    #[test]
    fn test_doormangreetingcourtesyrules_explain() {
        let rules = DoormanGreetingCourtesyRules::new();
        let e = rules.explain();
        assert!(e.contains("出入问候"));
        assert!(e.contains("配合门禁"));
        assert!(e.contains("求助帮忙"));
    }
}
