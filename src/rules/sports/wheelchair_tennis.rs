//! 轮椅网球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 轮椅网球规则
pub struct WheelchairTennisRules {
    metadata: RuleMetadata,
}

impl WheelchairTennisRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "轮椅网球规则",
                "轮椅网球比赛基本规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "残奥".into()]),
        }
    }

    /// 分级规则
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "公开级: 所有残疾",
            "Quad级: 四肢残疾",
            "分级认证",
            "医学评估",
            "功能评估",
        ]
    }

    /// 球场规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "标准网球场尺寸",
            "网高标准",
            "地面平整",
            "轮椅通行便利",
            "边线清晰",
        ]
    }

    /// 轮椅规格
    pub fn wheelchair_specifications(&self) -> Vec<&'static str> {
        vec![
            "竞速轮椅设计",
            "倾斜角度",
            "轮子尺寸",
            "防倾装置",
            "不得有尖锐部件",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "两跳规则",
            "第一次落地后击球",
            "第二次落地前击球",
            "轮转规则",
            "得分规则同标准网球",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "四局比赛",
            "抢七规则",
            "得分计算同标准网球",
            "决胜盘",
            "每球得分制",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "超过两跳",
            "轮椅违规",
            "触碰网",
            "不当行为",
            "危险动作",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "轮椅安全检查",
            "防倾装置",
            "避免碰撞",
            "医疗支持",
            "紧急预案",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "轮椅",
            "网球拍",
            "网球",
            "服装",
            "号码标识",
        ]
    }
}

impl Default for WheelchairTennisRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WheelchairTennisRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wheelchair_tennis")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【轮椅网球规则】\n\n\
            分级规则:\n{}\n\n\
            比赛规则:\n{}\n\n\
            犯规规则:\n{}\n\n\
            安全规则:\n{}\n",
            self.classification().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wheelchair_tennis_rules() {
        let rules = WheelchairTennisRules::new();
        assert!(!rules.classification().is_empty());
    }
}