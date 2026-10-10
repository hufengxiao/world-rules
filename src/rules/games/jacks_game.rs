//! 抓子游戏
//!
//! 抛球抓子的手眼协调传统游戏

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: JacksGameRules,
    name: "抓子游戏",
    desc: "抛球抓子的手眼协调传统游戏",
    origin: "欧美",
    tags: ["儿童", "技巧", "传统"]
}

impl JacksGameRules {
    /// 游戏准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "备一个弹球与十枚抓子",
            "平整桌面或地面",
            "两人或多人轮流",
            "约定回合规则",
        ]
    }

    /// 基本玩法
    pub fn basic(&self) -> Vec<&'static str> {
        vec![
            "抛球撒开抓子",
            "抛球后抓取规定数量",
            "再接住落下的球",
            "未抓住或碰动他子即失手",
        ]
    }

    /// 抓取等级
    pub fn levels(&self) -> Vec<&'static str> {
        vec![
            "先一次抓一枚",
            "再一次抓两枚",
            "逐级增至一次抓十枚",
            "依次完成各级",
        ]
    }

    /// 胜负判定
    pub fn winning(&self) -> Vec<&'static str> {
        vec![
            "率先完成全部等级者胜",
            "失手则轮换他人",
            "可约定计分制",
            "锻炼手眼协调",
        ]
    }
}

impl Rule for JacksGameRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("jacks_game")
    }

    fn explain(&self) -> String {
        format!(
            "【抓子游戏】\n{}",
            [
                format!(
                    "游戏准备：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "基本玩法：\\n{}",
                    self.basic()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "抓取等级：\\n{}",
                    self.levels()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负判定：\\n{}",
                    self.winning()
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
    fn test_jacksgamerules_basic() {
        let rules = JacksGameRules::new();
        assert_eq!(rules.metadata().name, "抓子游戏");
        assert!(!rules.setup().is_empty());
        assert!(!rules.basic().is_empty());
        assert!(!rules.levels().is_empty());
        assert!(!rules.winning().is_empty());
    }

    #[test]
    fn test_jacksgamerules_validation() {
        let rules = JacksGameRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("jacks_game"));
    }

    #[test]
    fn test_jacksgamerules_explain() {
        let rules = JacksGameRules::new();
        let e = rules.explain();
        assert!(e.contains("游戏准备"));
        assert!(e.contains("基本玩法"));
        assert!(e.contains("抓取等级"));
    }
}
