//! 斯诺克基本规则
//!
//! 斯诺克台球的开球、击球与计分规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: SnookerBasicsRules,
    name: "斯诺克基本规则",
    desc: "斯诺克台球的开球、击球与计分规则",
    origin: "国际",
    tags: ["体育", "斯诺克", "台球"]
}

impl SnookerBasicsRules {
    /// 台面与球
    pub fn table_balls(&self) -> Vec<&'static str> {
        vec![
            "台面有彩球布局",
            "红球与彩球分值不同",
            "开球击红球",
            "交替击红与彩球",
        ]
    }

    /// 击球规则
    pub fn shot(&self) -> Vec<&'static str> {
        vec![
            "先击目标红球",
            "进红球后击彩球",
            "未得分则换对方",
            "犯规对方获自由球",
        ]
    }

    /// 计分方式
    pub fn scoring(&self) -> Vec<&'static str> {
        vec!["红球一分一球", "彩球按分值", "最后清彩球", "分数高者获胜"]
    }

    /// 礼仪
    pub fn manner(&self) -> Vec<&'static str> {
        vec![
            "对方击球时安静",
            "不击打对方球",
            "保持球台整洁",
            "尊重裁判与对手",
        ]
    }
}

impl Rule for SnookerBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("snooker")
    }

    fn explain(&self) -> String {
        format!(
            "【斯诺克基本规则】\n{}",
            [
                format!(
                    "台面与球：\\n{}",
                    self.table_balls()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "击球规则：\\n{}",
                    self.shot()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分方式：\\n{}",
                    self.scoring()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼仪：\\n{}",
                    self.manner()
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
    fn test_snookerbasicsrules_basic() {
        let rules = SnookerBasicsRules::new();
        assert_eq!(rules.metadata().name, "斯诺克基本规则");
        assert!(!rules.table_balls().is_empty());
        assert!(!rules.shot().is_empty());
        assert!(!rules.scoring().is_empty());
        assert!(!rules.manner().is_empty());
    }

    #[test]
    fn test_snookerbasicsrules_validation() {
        let rules = SnookerBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("snooker"));
    }

    #[test]
    fn test_snookerbasicsrules_explain() {
        let rules = SnookerBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("台面与球"));
        assert!(e.contains("击球规则"));
        assert!(e.contains("计分方式"));
    }
}
