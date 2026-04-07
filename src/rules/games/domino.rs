//! 多米诺骨牌规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 多米诺骨牌规则
pub struct DominoRules {
    metadata: RuleMetadata,
}

impl DominoRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "多米诺骨牌规则",
                "多米诺骨牌基本玩法"
            )
            .with_origin("欧洲")
            .with_tags(vec!["游戏".into(), "骨牌".into()]),
        }
    }

    /// 骨牌组成
    pub fn domino_tiles(&self) -> Vec<&'static str> {
        vec![
            "标准套装: 28张(双六套装)",
            "每张骨牌有两个数字区",
            "数字从0到6(或更高)",
            "同数字骨牌称为双牌",
            "如双三、双六等",
        ]
    }

    /// 基本玩法
    pub fn basic_play(&self) -> Vec<&'static str> {
        vec![
            "骨牌需连接匹配数字",
            "双牌可横放或直放",
            "轮流出牌",
            "无法出牌时需抽牌或跳过",
            "先出完所有牌者获胜",
        ]
    }

    /// 连接规则
    pub fn connection_rules(&self) -> Vec<&'static str> {
        vec![
            "必须匹配数字",
            "数字端连接相同数字端",
            "双牌可放置在任意一端",
            "连接形成一条线",
            "可分支放置(如允许)",
        ]
    }

    /// 经典玩法
    pub fn classic_game(&self) -> Vec<&'static str> {
        vec![
            "每人抽7张牌(2人)",
            "5张牌(3-4人)",
            "双六先出牌",
            "轮流出牌连接",
            "无法出牌则跳过",
            "先清空手牌获胜",
        ]
    }

    /// 计分玩法
    pub fn scoring_game(&self) -> Vec<&'static str> {
        vec![
            "清空手牌者得对手剩余牌点数",
            "达到目标分数获胜",
            "通常目标为100或200分",
            "对手牌点数总和为得分",
        ]
    }

    /// 封锁玩法
    pub fn block_game(&self) -> Vec<&'static str> {
        vec![
            "无法出牌时跳过",
            "所有人无法出牌时游戏结束",
            "剩余牌点数最少者获胜",
            "也称\"堵塞\"玩法",
        ]
    }

    /// 多米诺效应(连锁)
    pub fn chain_effect(&self) -> Vec<&'static str> {
        vec![
            "骨牌依次倒下产生连锁",
            "骨牌间距影响倒下速度",
            "能量传递遵循物理规律",
            "可用于艺术展示",
            "世界纪录达数百万张",
        ]
    }
}

impl Default for DominoRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DominoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("domino")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【多米诺骨牌规则】\n\n\
            骨牌组成:\n{}\n\n\
            基本玩法:\n{}\n\n\
            连接规则:\n{}\n\n\
            多米诺效应:\n{}\n",
            self.domino_tiles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.basic_play().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.connection_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.chain_effect().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domino_rules() {
        let rules = DominoRules::new();
        assert!(!rules.domino_tiles().is_empty());
    }
}