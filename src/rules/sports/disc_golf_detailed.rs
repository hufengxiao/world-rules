//! 飞盘高尔夫详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 飞盘高尔夫详细规则 (Disc Golf Detailed)
pub struct DiscGolfDetailedRules {
    metadata: RuleMetadata,
}

impl DiscGolfDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "飞盘高尔夫详细规则",
                "飞盘高尔夫比赛详细规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "休闲".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "单轮比赛",
            "多轮比赛",
            "团体比赛",
            "锦标赛",
            "表演比赛",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛采用杆数制",
            "最少杆数获胜",
            "18洞比赛",
            "罚杆规则",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn course_specifications(&self) -> Vec<&'static str> {
        vec![
            "洞数: 9洞或18洞",
            "目标筐直径: 52厘米",
            "目标筐高度: 1.2米",
            "场地布局",
            "障碍设置",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "后手投掷",
            "前手投掷",
            "距离投掷",
            "精准投掷",
            "特殊投掷",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "每洞杆数",
            "总杆数计算",
            "标准杆: Par",
            "罚杆规定",
            "记录标准",
        ]
    }

    /// 飞盘类型
    pub fn disc_types(&self) -> Vec<&'static str> {
        vec![
            "距离盘: Driver",
            "中等距离盘: Mid-range",
            "精准盘: Putter",
            "规格要求",
            "飞盘选择",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "飞盘套装",
            "目标筐",
            "场地装备",
            "比赛服装",
            "计分板",
        ]
    }
}

impl Default for DiscGolfDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DiscGolfDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("disc_golf_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【飞盘高尔夫详细规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.course_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disc_golf_detailed_rules() {
        let rules = DiscGolfDetailedRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}