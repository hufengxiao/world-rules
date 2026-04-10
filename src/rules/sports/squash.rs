//! 壁球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 壁球规则
pub struct SquashRules {
    metadata: RuleMetadata,
}

impl SquashRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "壁球规则",
                "壁球比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "球类".into()]),
        }
    }

    /// 场地规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "球场长度: 9.75米",
            "球场宽度: 6.4米",
            "前墙高度: 4.57米",
            "后墙高度: 2.13米",
            "球场面积: 约62平方米",
        ]
    }

    /// 球规格
    pub fn ball_specifications(&self) -> Vec<&'static str> {
        vec![
            "球直径: 约40毫米",
            "球重量: 约24克",
            "橡胶制",
            "不同速度等级",
            "比赛专用球",
        ]
    }

    /// 球拍规格
    pub fn racket_specifications(&self) -> Vec<&'static str> {
        vec![
            "球拍长度: 最长68.6厘米",
            "球拍宽度: 最宽21.3厘米",
            "重量: 适中",
            "材料: 复合材料",
            "击球面积",
        ]
    }

    /// 比赛规则
    pub fn match_rules(&self) -> Vec<&'static str> {
        vec![
            "每局11分",
            "至少领先2分获胜",
            "轮流发球",
            "发球区发球",
            "接球方选择接球",
        ]
    }

    /// 发球规则
    pub fn service_rules(&self) -> Vec<&'static str> {
        vec![
            "必须在发球区发球",
            "球必须击中前墙",
            "球落在对手区域",
            "一次发球机会",
            "发球犯规判失分",
        ]
    }

    /// 回球规则
    pub fn return_rules(&self) -> Vec<&'static str> {
        vec![
            "球必须击中前墙",
            "允许反弹后击球",
            "不能妨碍对手",
            "球不能触碰地面两次",
            "必须在对手回球前击球",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "对手犯规得分",
            "对手无法回球得分",
            "对手妨碍得分",
            "比赛中断判定",
            "决胜局规则",
        ]
    }

    /// 禁止行为
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "妨碍对手击球",
            "危险挥拍",
            "故意拖延",
            "不当行为",
            "犯规击球",
        ]
    }
}

impl Default for SquashRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SquashRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("squash")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【壁球规则】\n\n\
            场地规格:\n{}\n\n\
            比赛规则:\n{}\n\n\
            发球规则:\n{}\n\n\
            禁止行为:\n{}\n",
            self.court_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.match_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.service_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squash_rules() {
        let rules = SquashRules::new();
        assert!(!rules.court_specifications().is_empty());
    }
}