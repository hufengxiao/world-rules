//! 轮椅篮球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 轮椅篮球规则
pub struct WheelchairBasketballRules {
    metadata: RuleMetadata,
}

impl WheelchairBasketballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "轮椅篮球规则",
                "轮椅篮球比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "残奥".into()]),
        }
    }

    /// 比赛时间
    pub fn game_duration(&self) -> Vec<&'static str> {
        vec![
            "四节比赛",
            "每节10分钟",
            "节间休息2分钟",
            "半场休息15分钟",
            "加时赛5分钟",
        ]
    }

    /// 球场规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "标准篮球场尺寸",
            "三分线距离",
            "篮筐高度: 3.05米",
            "无障碍设施",
            "地板平整",
        ]
    }

    /// 轮椅规格
    pub fn wheelchair_specifications(&self) -> Vec<&'static str> {
        vec![
            "轮椅高度限制",
            "座垫规格",
            "防倾装置",
            "轮子尺寸",
            "不得有尖锐部件",
        ]
    }

    /// 分级规则
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "1.0-4.5分级",
            "残疾程度评估",
            "队伍总分限制",
            "分级认证",
            "技术评估",
        ]
    }

    /// 技术规则
    pub fn technical_rules(&self) -> Vec<&'static str> {
        vec![
            "运球: 推轮椅两次后运球",
            "投篮规则",
            "传球规则",
            "3秒规则",
            "5秒规则",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "非法阻挡",
            "碰撞犯规",
            "轮椅接触犯规",
            "技术犯规",
            "个人犯规",
        ]
    }

    /// 队伍构成
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队5人上场",
            "替补队员",
            "总分限制: 14分",
            "位置分配",
            "教练指导",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "轮椅安全检查",
            "防倾装置必须",
            "禁止危险动作",
            "医疗支持",
            "紧急撤离",
        ]
    }
}

impl Default for WheelchairBasketballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WheelchairBasketballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wheelchair_basketball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【轮椅篮球规则】\n\n\
            分级规则:\n{}\n\n\
            轮椅规格:\n{}\n\n\
            技术规则:\n{}\n\n\
            犯规规则:\n{}\n",
            self.classification().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.wheelchair_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technical_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wheelchair_basketball_rules() {
        let rules = WheelchairBasketballRules::new();
        assert!(!rules.classification().is_empty());
    }
}