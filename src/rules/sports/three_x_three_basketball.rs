//! 3×3篮球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 3×3篮球规则
pub struct ThreeXThreeBasketballRules {
    metadata: RuleMetadata,
}

impl ThreeXThreeBasketballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "3×3篮球规则",
                "3×3篮球比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "篮球".into()]),
        }
    }

    /// 球场规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "半场标准篮球场",
            "尺寸: 15×11米",
            "三分线距离6.75米",
            "篮筐高度3.05米",
            "无禁区线",
        ]
    }

    /// 队伍构成
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队3人上场",
            "1名替补",
            "无固定位置",
            "轮转自由",
            "教练可选",
        ]
    }

    /// 比赛时间
    pub fn game_duration(&self) -> Vec<&'static str> {
        vec![
            "单节10分钟",
            "有效时间制",
            "最后2分钟停表",
            "先达21分获胜",
            "加时赛",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "一分球: 罚球",
            "一分球: 两分区内",
            "两分球: 三分线外",
            "先达21分获胜",
            "时间结束比分高者胜",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "个人犯规不计",
            "技术犯规罚球",
            "犯规达6次罚球",
            "恶意犯规处罚",
            "比赛控制犯规",
        ]
    }

    /// 发球规则
    pub fn possession_rules(&self) -> Vec<&'static str> {
        vec![
            "进球后交换球权",
            "防守方发球",
            "半圆弧外发球",
            "12秒进攻时间",
            "球权交换",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "奥运会项目",
            "世界巡回赛",
            "世界锦标赛",
            "国内联赛",
            "街头篮球",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "标准篮球",
            "运动服",
            "运动鞋",
            "号码标识",
            "可选护具",
        ]
    }
}

impl Default for ThreeXThreeBasketballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ThreeXThreeBasketballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("three_x_three_basketball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【3×3篮球规则】\n\n\
            球场规格:\n{}\n\n\
            得分规则:\n{}\n\n\
            犯规规则:\n{}\n\n\
            发球规则:\n{}\n",
            self.court_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.possession_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_x_three_basketball_rules() {
        let rules = ThreeXThreeBasketballRules::new();
        assert!(!rules.court_specifications().is_empty());
    }
}