//! 羽毛球基本规则
//!
//! 羽毛球场馆、发球、得分与单双打基本规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BadmintonBasicsRules,
    name: "羽毛球基本规则",
    desc: "羽毛球场馆、发球、得分与单双打基本规则",
    origin: "国际",
    tags: ["体育", "羽毛球", "规则"]
}

impl BadmintonBasicsRules {
    /// 发球规则
    pub fn serve(&self) -> Vec<&'static str> {
        vec![
            "发球须过对角线区域",
            "发球者脚不过发球线",
            "球须过网落入对方区",
            "发球时球低于腰际击出",
        ]
    }

    /// 回合得分
    pub fn rally(&self) -> Vec<&'static str> {
        vec![
            "球落地或出界定胜负",
            "触网挂网界内算落点",
            "每球得分制",
            "一方失误对方得分",
        ]
    }

    /// 单双打
    pub fn singles_doubles(&self) -> Vec<&'static str> {
        vec![
            "单打边线与发球有区分",
            "双打注意连击规则",
            "发球换位依得分奇偶",
            "站位兼顾配合",
        ]
    }

    /// 赛制
    pub fn format(&self) -> Vec<&'static str> {
        vec![
            "每局21分制",
            "20平需领先2分",
            "赛点29平时先得30胜",
            "三局两胜常见",
        ]
    }
}

impl Rule for BadmintonBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("badminton")
    }

    fn explain(&self) -> String {
        format!(
            "【羽毛球基本规则】\n{}",
            [
                format!(
                    "发球规则：\\n{}",
                    self.serve()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回合得分：\\n{}",
                    self.rally()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "单双打：\\n{}",
                    self.singles_doubles()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "赛制：\\n{}",
                    self.format()
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
    fn test_badmintonbasicsrules_basic() {
        let rules = BadmintonBasicsRules::new();
        assert_eq!(rules.metadata().name, "羽毛球基本规则");
        assert!(!rules.serve().is_empty());
        assert!(!rules.rally().is_empty());
        assert!(!rules.singles_doubles().is_empty());
        assert!(!rules.format().is_empty());
    }

    #[test]
    fn test_badmintonbasicsrules_validation() {
        let rules = BadmintonBasicsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("badminton"));
    }

    #[test]
    fn test_badmintonbasicsrules_explain() {
        let rules = BadmintonBasicsRules::new();
        let e = rules.explain();
        assert!(e.contains("发球规则"));
        assert!(e.contains("回合得分"));
        assert!(e.contains("单双打"));
    }
}
