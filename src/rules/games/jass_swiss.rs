//! 瑞士雅斯
//!
//! 瑞士国民牌戏，以牌型与分值计分的吃墩游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: JassSwissRules,
    name: "瑞士雅斯",
    desc: "瑞士国民牌戏，以牌型与分值计分的吃墩游戏",
    origin: "瑞士",
    tags: ["扑克", "吃墩", "瑞士"]
}

impl JassSwissRules {
    /// 发牌准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "使用三十六张瑞士牌",
            "四人两两结对",
            "每人发九张牌",
            "轮流坐庄定将",
        ]
    }

    /// 将牌选择
    pub fn trump(&self) -> Vec<&'static str> {
        vec![
            "庄家可定花色将",
            "亦可选无将玩法",
            "或选底层高牌玩法",
            "依牌型策略选择",
        ]
    }

    /// 牌值计分
    pub fn points(&self) -> Vec<&'static str> {
        vec![
            "杰克为最高将牌",
            "九次之A再次",
            "KQ十依次排列",
            "合计分值计胜负",
        ]
    }

    /// 特殊牌型
    pub fn combos(&self) -> Vec<&'static str> {
        vec![
            "三张同点连出称三类",
            "四张同点连出称四类",
            "连牌顺子亦可得分",
            "依约定牌型加计分",
        ]
    }
}

impl Rule for JassSwissRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("jass_swiss")
    }

    fn explain(&self) -> String {
        format!(
            "【瑞士雅斯】\n{}",
            [
                format!(
                    "发牌准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "将牌选择：\\n{}",
                    self.trump()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "牌值计分：\\n{}",
                    self.points()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "特殊牌型：\\n{}",
                    self.combos()
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
    fn test_jassswissrules_basic() {
        let rules = JassSwissRules::new();
        assert_eq!(rules.metadata().name, "瑞士雅斯");
        assert!(!rules.setup().is_empty());
        assert!(!rules.trump().is_empty());
        assert!(!rules.points().is_empty());
        assert!(!rules.combos().is_empty());
    }

    #[test]
    fn test_jassswissrules_validation() {
        let rules = JassSwissRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("jass_swiss"));
    }

    #[test]
    fn test_jassswissrules_explain() {
        let rules = JassSwissRules::new();
        let e = rules.explain();
        assert!(e.contains("发牌准备"));
        assert!(e.contains("将牌选择"));
        assert!(e.contains("牌值计分"));
    }
}
