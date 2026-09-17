//! 乒乓球发球
//!
//! 乒乓球发球规则、抛球与台内限制

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TableTennisServeRules,
    name: "乒乓球发球",
    desc: "乒乓球发球规则、抛球与台内限制",
    origin: "国际",
    tags: ["体育", "乒乓球", "发球"]
}

impl TableTennisServeRules {
    /// 抛球规则
    pub fn toss(&self) -> Vec<&'static str> {
        vec!["手掌平摊抛球", "垂直上抛", "抛高过十六厘米", "自由落体触球"]
    }

    /// 击球合法
    pub fn legal(&self) -> Vec<&'static str> {
        vec![
            "球先落已台再落对方台",
            "不得遮挡拍手",
            "触网重发",
            "出界犯规",
        ]
    }

    /// 轮流发球
    pub fn turns(&self) -> Vec<&'static str> {
        vec!["每两分换发球", "双方各发轮流", "接发失误", "发球得分"]
    }

    /// 球拍技术
    pub fn stroke(&self) -> Vec<&'static str> {
        vec!["把握抛击时机", "控制旋转落点", "变化发球", "掌握先机"]
    }
}

impl Rule for TableTennisServeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("pingpong_serve")
    }

    fn explain(&self) -> String {
        format!(
            "【乒乓球发球】\n{}",
            [
                format!(
                    "抛球规则：\\n{}",
                    self.toss()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "击球合法：\\n{}",
                    self.legal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "轮流发球：\\n{}",
                    self.turns()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "球拍技术：\\n{}",
                    self.stroke()
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
    fn test_tabletennisserverules_basic() {
        let rules = TableTennisServeRules::new();
        assert_eq!(rules.metadata().name, "乒乓球发球");
        assert!(!rules.toss().is_empty());
        assert!(!rules.legal().is_empty());
        assert!(!rules.turns().is_empty());
        assert!(!rules.stroke().is_empty());
    }

    #[test]
    fn test_tabletennisserverules_validation() {
        let rules = TableTennisServeRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("pingpong_serve"));
    }

    #[test]
    fn test_tabletennisserverules_explain() {
        let rules = TableTennisServeRules::new();
        let e = rules.explain();
        assert!(e.contains("抛球规则"));
        assert!(e.contains("击球合法"));
        assert!(e.contains("轮流发球"));
    }
}
