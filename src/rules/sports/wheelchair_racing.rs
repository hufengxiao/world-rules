//! 轮椅竞速规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 轮椅竞速规则
pub struct WheelchairRacingRules {
    metadata: RuleMetadata,
}

impl WheelchairRacingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "轮椅竞速规则",
                "轮椅竞速比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "残奥".into()]),
        }
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "100米",
            "200米",
            "400米",
            "800米",
            "1500米",
            "5000米",
            "马拉松",
        ]
    }

    /// 分级规则
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "T51-54分级",
            "残疾程度评估",
            "轮椅类型分级",
            "分级认证",
            "技术评估",
        ]
    }

    /// 轮椅规格
    pub fn wheelchair_specifications(&self) -> Vec<&'static str> {
        vec![
            "竞速轮椅设计",
            "三轮设计",
            "重量限制",
            "尺寸规定",
            "安全要求",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "道次分配",
            "起跑规则",
            "跑道使用",
            "超车规则",
            "终点判定",
        ]
    }

    /// 技术规则
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "推轮椅技术",
            "速度控制",
            "转向技术",
            "节奏控制",
            "能量效率",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "抢跑",
            "跑道侵占",
            "妨碍他人",
            "轮椅违规",
            "危险动作",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "轮椅安全检查",
            "头盔可选",
            "跑道安全",
            "医疗支持",
            "紧急预案",
        ]
    }

    /// 计时规则
    pub fn timing(&self) -> Vec<&'static str> {
        vec![
            "电子计时",
            "精确到0.01秒",
            "分段计时",
            "成绩公布",
            "世界纪录认证",
        ]
    }
}

impl Default for WheelchairRacingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for WheelchairRacingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("wheelchair_racing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【轮椅竞速规则】\n\n\
            比赛距离:\n{}\n\n\
            分级规则:\n{}\n\n\
            技术规则:\n{}\n\n\
            犯规规则:\n{}\n",
            self.distances().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.classification().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technique().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wheelchair_racing_rules() {
        let rules = WheelchairRacingRules::new();
        assert!(!rules.distances().is_empty());
    }
}