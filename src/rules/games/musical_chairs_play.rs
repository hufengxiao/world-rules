//! 抢椅子游戏
//!
//! 抢椅子的音乐、停椅与淘汰规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MusicalChairsPlayRules,
    name: "抢椅子游戏",
    desc: "抢椅子的音乐、停椅与淘汰规则",
    origin: "国际",
    tags: ["游戏", "抢椅子", "派对"]
}

impl MusicalChairsPlayRules {
    /// 布置规则
    pub fn setup(&self) -> Vec<&'static str> {
        vec!["椅子比人数少一", "围圈摆放", "音乐响起绕行", "停乐即抢坐"]
    }

    /// 抢坐公平
    pub fn play(&self) -> Vec<&'static str> {
        vec!["停乐立刻找座", "无座即刻淘汰", "撤椅继续", "直到剩一人"]
    }

    /// 安全提醒
    pub fn safety(&self) -> Vec<&'static str> {
        vec!["不推挤不抢撞", "椅子放平稳", "跌倒互让", "安全第一"]
    }

    /// 欢乐进行
    pub fn fun(&self) -> Vec<&'static str> {
        vec!["淘汰不气馁", "旁观鼓掌", "轮流当裁判", "乐在其中"]
    }
}

impl Rule for MusicalChairsPlayRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("musical_chairs")
    }

    fn explain(&self) -> String {
        format!(
            "【抢椅子游戏】\n{}",
            [
                format!(
                    "布置规则：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "抢坐公平：\\n{}",
                    self.play()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全提醒：\\n{}",
                    self.safety()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "欢乐进行：\\n{}",
                    self.fun()
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
    fn test_musicalchairsplayrules_basic() {
        let rules = MusicalChairsPlayRules::new();
        assert_eq!(rules.metadata().name, "抢椅子游戏");
        assert!(!rules.setup().is_empty());
        assert!(!rules.play().is_empty());
        assert!(!rules.safety().is_empty());
        assert!(!rules.fun().is_empty());
    }

    #[test]
    fn test_musicalchairsplayrules_validation() {
        let rules = MusicalChairsPlayRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("musical_chairs"));
    }

    #[test]
    fn test_musicalchairsplayrules_explain() {
        let rules = MusicalChairsPlayRules::new();
        let e = rules.explain();
        assert!(e.contains("布置规则"));
        assert!(e.contains("抢坐公平"));
        assert!(e.contains("安全提醒"));
    }
}
