//! 红绿灯木头人
//!
//! 红绿灯口令前进停止的规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: RedLightGreenRules,
    name: "红绿灯木头人",
    desc: "红绿灯口令前进停止的规则",
    origin: "国际",
    tags: ["游戏", "红绿灯", "户外", "儿童"]
}

impl RedLightGreenRules {
    /// 游戏设定
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "一人当发号者",
            "他人从端线出发",
            "念绿灯可前进",
            "红灯须停住",
        ]
    }

    /// 前进停止
    pub fn motion(&self) -> Vec<&'static str> {
        vec!["绿灯前进快", "红灯立刻立定", "动者回起点", "依次到达"]
    }

    /// 公平有趣
    pub fn fun(&self) -> Vec<&'static str> {
        vec!["不抢跑不赖", "发号清楚", "输赢无所谓", "重在嬉戏"]
    }

    /// 安全场地
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["选开阔场地", "避开车辆", "照看幼儿", "安全玩游戏"]
    }
}

impl Rule for RedLightGreenRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("red_light")
    }

    fn explain(&self) -> String {
        format!(
            "【红绿灯木头人】\n{}",
            [
                format!(
                    "游戏设定：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "前进停止：\\n{}",
                    self.motion()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公平有趣：\\n{}",
                    self.fun()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全场地：\\n{}",
                    self.safety()
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
    fn test_redlightgreenrules_basic() {
        let rules = RedLightGreenRules::new();
        assert_eq!(rules.metadata().name, "红绿灯木头人");
        assert!(!rules.setup().is_empty());
        assert!(!rules.motion().is_empty());
        assert!(!rules.fun().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_redlightgreenrules_validation() {
        let rules = RedLightGreenRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("red_light"));
    }

    #[test]
    fn test_redlightgreenrules_explain() {
        let rules = RedLightGreenRules::new();
        let e = rules.explain();
        assert!(e.contains("游戏设定"));
        assert!(e.contains("前进停止"));
        assert!(e.contains("公平有趣"));
    }
}
