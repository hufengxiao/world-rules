//! 飞盘接抛
//!
//! 飞盘投掷、接盘与风向技巧

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FrisbeePlayCatchRules,
    name: "飞盘接抛",
    desc: "飞盘投掷、接盘与风向技巧",
    origin: "国际",
    tags: ["游戏", "飞盘", "户外"]
}

impl FrisbeePlayCatchRules {
    /// 握盘抛法
    pub fn throw(&self) -> Vec<&'static str> {
        vec!["侧身持盘", "手贴盘缘出手", "手腕发力", "平稳送出"]
    }

    /// 接盘技巧
    pub fn catch(&self) -> Vec<&'static str> {
        vec!["看准落下位置", "双手接稳", "身体迎上", "接住为止"]
    }

    /// 风向注意
    pub fn wind(&self) -> Vec<&'static str> {
        vec!["顺风抛远近有别", "逆风加力", "风力大调整", "预判轨迹"]
    }

    /// 场地合作
    pub fn cooperate(&self) -> Vec<&'static str> {
        vec!["开阔场地玩", "注意他人", "轮流接抛", "尽兴安全"]
    }
}

impl Rule for FrisbeePlayCatchRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("frisbee")
    }

    fn explain(&self) -> String {
        format!(
            "【飞盘接抛】\n{}",
            [
                format!(
                    "握盘抛法：\\n{}",
                    self.throw()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "接盘技巧：\\n{}",
                    self.catch()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "风向注意：\\n{}",
                    self.wind()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "场地合作：\\n{}",
                    self.cooperate()
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
    fn test_frisbeeplaycatchrules_basic() {
        let rules = FrisbeePlayCatchRules::new();
        assert_eq!(rules.metadata().name, "飞盘接抛");
        assert!(!rules.throw().is_empty());
        assert!(!rules.catch().is_empty());
        assert!(!rules.wind().is_empty());
        assert!(!rules.cooperate().is_empty());
    }

    #[test]
    fn test_frisbeeplaycatchrules_validation() {
        let rules = FrisbeePlayCatchRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("frisbee"));
    }

    #[test]
    fn test_frisbeeplaycatchrules_explain() {
        let rules = FrisbeePlayCatchRules::new();
        let e = rules.explain();
        assert!(e.contains("握盘抛法"));
        assert!(e.contains("接盘技巧"));
        assert!(e.contains("风向注意"));
    }
}
