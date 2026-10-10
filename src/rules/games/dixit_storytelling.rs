//! 故事猜图（Dixit）
//!
//! Dixit 是想象力的说书猜图游戏，讲述者用一句话描述手中卡牌，其他玩家据话选出最贴切的牌，猜中与误导皆有得分。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: DixitStorytellingRules,
    name: "故事猜图（Dixit）",
    desc: "Dixit 是想象力的说书猜图游戏，讲述者用一句话描述手中卡牌，其他玩家据话选出最贴切的牌，猜中与误导皆有得分。",
    origin: "由 Jean-Louis Roubira 于2008年设计",
    tags: ["聚会", "想象", "猜图", "桌面游戏"]
}

impl DixitStorytellingRules {
    /// 游戏准备
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "每人手持若干张图案卡",
            "备好计分标记",
            "选定一人作讲述者",
            "确定目标分数",
        ]
    }

    /// 讲述环节
    pub fn storytelling(&self) -> Vec<&'static str> {
        vec![
            "讲述者选一张手牌",
            "用一句话或短语描述",
            "可诗意可含蓄但不可太直白",
            "不得透露牌面细节",
        ]
    }

    /// 出牌与投票
    pub fn voting(&self) -> Vec<&'static str> {
        vec![
            "其他玩家各选一张贴合描述的牌",
            "所有牌洗混后编号摆出",
            "玩家投票选出讲述者的牌",
            "不可投自己的牌",
        ]
    }

    /// 计分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "描述太直白人人猜中讲述者不得分",
            "无人猜中讲述者也难得分",
            "猜中者与投票给其牌者均得分",
            "先达目标分者获胜",
        ]
    }
}

impl Rule for DixitStorytellingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("dixit")
    }

    fn explain(&self) -> String {
        format!(
            "【故事猜图（Dixit）】\n{}",
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
                    "讲述环节：\\n{}",
                    self.storytelling()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "出牌与投票：\\n{}",
                    self.voting()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "计分规则：\\n{}",
                    self.scoring()
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
    fn test_dixitstorytellingrules_basic() {
        let rules = DixitStorytellingRules::new();
        assert_eq!(rules.metadata().name, "故事猜图（Dixit）");
        assert!(!rules.setup().is_empty());
        assert!(!rules.storytelling().is_empty());
        assert!(!rules.voting().is_empty());
        assert!(!rules.scoring().is_empty());
    }

    #[test]
    fn test_dixitstorytellingrules_validation() {
        let rules = DixitStorytellingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("dixit"));
    }

    #[test]
    fn test_dixitstorytellingrules_explain() {
        let rules = DixitStorytellingRules::new();
        let e = rules.explain();
        assert!(e.contains("游戏准备"));
        assert!(e.contains("讲述环节"));
        assert!(e.contains("出牌与投票"));
    }
}
