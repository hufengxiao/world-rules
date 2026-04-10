//! 赛艇规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 赛艇规则
pub struct RowingRules {
    metadata: RuleMetadata,
}

impl RowingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "赛艇规则",
                "赛艇比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "水上".into()]),
        }
    }

    /// 艇型分类
    pub fn boat_classes(&self) -> Vec<&'static str> {
        vec![
            "单人双桨(1x): 1人",
            "双人双桨(2x): 2人",
            "四人双桨(4x): 4人",
            "八人单桨有舵手(8+): 8人+舵手",
            "轻量级比赛有体重限制",
        ]
    }

    /// 比赛距离
    pub fn race_distances(&self) -> Vec<&'static str> {
        vec![
            "标准距离: 2000米",
            "青少年比赛: 1000-1500米",
            "奥运和世锦赛: 2000米",
            "分六条航道",
            "直线航道",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "预赛: 决定决赛资格",
            "复赛: 第二次机会赛",
            "半决赛: 决定决赛道次",
            "决赛A: 争夺奖牌",
            "决赛B-F: 排名赛",
        ]
    }

    /// 起航规则
    pub fn start_rules(&self) -> Vec<&'static str> {
        vec![
            "固定起航系统",
            "所有艇同时起航",
            "抢航: 提前起航",
            "一次抢航警告",
            "二次抢航取消资格",
        ]
    }

    /// 航道规则
    pub fn lane_rules(&self) -> Vec<&'static str> {
        vec![
            "必须保持在指定航道",
            "不得干扰其他艇",
            "不得越出航道",
            "终点判定以艇头为准",
            "并列时测量距离",
        ]
    }

    /// 技术要求
    pub fn technical_requirements(&self) -> Vec<&'static str> {
        vec![
            "正确的划桨技术",
            "配合和节奏",
            "舵手控制方向",
            "力量和耐力",
            "器械检查符合标准",
        ]
    }

    /// 裁判职责
    pub fn official_duties(&self) -> Vec<&'static str> {
        vec![
            "发令员: 负责起航",
            "航道裁判: 监督航道",
            "终点裁判: 判定名次",
            "丈量员: 测量距离",
            "仲裁: 处理申诉",
        ]
    }
}

impl Default for RowingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RowingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("rowing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【赛艇规则】\n\n\
            艇型分类:\n{}\n\n\
            比赛距离:\n{}\n\n\
            起航规则:\n{}\n\n\
            航道规则:\n{}\n",
            self.boat_classes().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.race_distances().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.start_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.lane_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rowing_rules() {
        let rules = RowingRules::new();
        assert!(!rules.boat_classes().is_empty());
    }
}