//! 极限飞盘规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 极限飞盘规则 (Ultimate Frisbee)
pub struct UltimateFrisbeeRules {
    metadata: RuleMetadata,
}

impl UltimateFrisbeeRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "极限飞盘规则",
                "极限飞盘比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间限制",
            "得分区规则",
            "飞盘传递",
            "无接触规则",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 100×37米",
            "得分区: 18米深度",
            "边界线",
            "安全区域",
            "场地布置",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队7人上场",
            "场上位置",
            "替补队员",
            "换人规则",
            "队员轮换",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "投掷技术",
            "接盘技术",
            "跑位技术",
            "防守技术",
            "传递策略",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "接触犯规",
            "阻挡犯规",
            "走步犯规",
            "犯规处理",
            " Spirit of the Game",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "在得分区接盘得分",
            "有效得分",
            "得分统计",
            "比分记录",
            "比赛胜负",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "极限飞盘",
            "比赛服装",
            "场地装备",
            "计分板",
            "防护装备",
        ]
    }
}

impl Default for UltimateFrisbeeRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for UltimateFrisbeeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ultimate_frisbee")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【极限飞盘规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_ultimate_frisbee_rules() {
        let rules = UltimateFrisbeeRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}