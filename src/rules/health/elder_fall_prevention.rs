//! 老人防跌倒
//!
//! 预防老年人跌倒的居家改造、锻炼与行动安全规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ElderFallPreventionRules,
    name: "老人防跌倒",
    desc: "预防老年人跌倒的居家改造、锻炼与行动安全规则",
    origin: "国际",
    tags: ["健康", "老人", "跌倒", "预防", "居家安全"]
}

impl ElderFallPreventionRules {
    /// 环境改造
    pub fn home(&self) -> Vec<&'static str> {
        vec![
            "过道保持无障碍与照明充足",
            "浴室铺防滑垫装扶手",
            "固定松散地垫与电线",
            "常用物品放在易取高度",
        ]
    }

    /// 身体锻炼
    pub fn exercise(&self) -> Vec<&'static str> {
        vec![
            "坚持平衡与肌力训练",
            "适度活动避免久坐",
            "进行改善步态的练习",
            "锻炼循序渐进量力",
        ]
    }

    /// 行动习惯
    pub fn habits(&self) -> Vec<&'static str> {
        vec![
            "起身站稳再行走",
            "穿合脚防滑鞋拖鞋",
            "夜间如厕开灯稳扶",
            "不攀高取物",
        ]
    }

    /// 健康与照护
    pub fn care(&self) -> Vec<&'static str> {
        vec![
            "定期检查视听力",
            "留意药物引起的头晕",
            "跌倒后即使轻微也评估",
            "照顾者协助防范",
        ]
    }
}

impl Rule for ElderFallPreventionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::health("elder_fall")
    }

    fn explain(&self) -> String {
        format!(
            "【老人防跌倒】\n{}",
            [
                format!(
                    "环境改造：\\n{}",
                    self.home()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "身体锻炼：\\n{}",
                    self.exercise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "行动习惯：\\n{}",
                    self.habits()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "健康与照护：\\n{}",
                    self.care()
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
    fn test_elderfallpreventionrules_basic() {
        let rules = ElderFallPreventionRules::new();
        assert_eq!(rules.metadata().name, "老人防跌倒");
        assert!(!rules.home().is_empty());
        assert!(!rules.exercise().is_empty());
        assert!(!rules.habits().is_empty());
        assert!(!rules.care().is_empty());
    }

    #[test]
    fn test_elderfallpreventionrules_validation() {
        let rules = ElderFallPreventionRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::health("elder_fall"));
    }

    #[test]
    fn test_elderfallpreventionrules_explain() {
        let rules = ElderFallPreventionRules::new();
        let e = rules.explain();
        assert!(e.contains("环境改造"));
        assert!(e.contains("身体锻炼"));
        assert!(e.contains("行动习惯"));
    }
}
