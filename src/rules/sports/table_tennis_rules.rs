//! 乒乓球基本规则
//!
//! 乒乓球发球、得分、赛制与器材的基本规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TableTennisPlayRules,
    name: "乒乓球基本规则",
    desc: "乒乓球发球、得分、赛制与器材的基本规则",
    origin: "国际",
    tags: ["体育", "乒乓球", "规则"]
}

impl TableTennisPlayRules {
    /// 发球规则
    pub fn serve(&self) -> Vec<&'static str> {
        vec![
            "发球须垂直抛球",
            "抛球不低于16厘米",
            "球须越过球网上方",
            "发球时球在台面外",
        ]
    }

    /// 回合与得分
    pub fn rally(&self) -> Vec<&'static str> {
        vec![
            "球触对方台面得分",
            "未过网或出界丢分",
            "擦边算有效",
            "每球定胜负得1分",
        ]
    }

    /// 赛制
    pub fn format(&self) -> Vec<&'static str> {
        vec![
            "每局先得11分者胜",
            "十平后领先两分得胜",
            "五局三胜或七局四胜",
            "轮流发球每两分交换",
        ]
    }

    /// 器材场地
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "球台长2米74宽1米52",
            "球拍两面不同色",
            "胶皮不得涂改",
            "比赛用白或橙球",
        ]
    }
}

impl Rule for TableTennisPlayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("table_tennis")
    }

    fn explain(&self) -> String {
        format!(
            "【乒乓球基本规则】\n{}",
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
                    "回合与得分：\\n{}",
                    self.rally()
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
                format!(
                    "器材场地：\\n{}",
                    self.equipment()
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
    fn test_tabletennisplayrules_basic() {
        let rules = TableTennisPlayRules::new();
        assert_eq!(rules.metadata().name, "乒乓球基本规则");
        assert!(!rules.serve().is_empty());
        assert!(!rules.rally().is_empty());
        assert!(!rules.format().is_empty());
        assert!(!rules.equipment().is_empty());
    }

    #[test]
    fn test_tabletennisplayrules_validation() {
        let rules = TableTennisPlayRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::sports("table_tennis"));
    }

    #[test]
    fn test_tabletennisplayrules_explain() {
        let rules = TableTennisPlayRules::new();
        let e = rules.explain();
        assert!(e.contains("发球规则"));
        assert!(e.contains("回合与得分"));
        assert!(e.contains("赛制"));
    }
}
