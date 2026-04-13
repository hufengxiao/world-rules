//! 网球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 网球规则 (Netball)
pub struct NetballRules {
    metadata: RuleMetadata,
}

impl NetballRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "网球规则",
                "女子网球比赛规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "团队".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 4节",
            "每节15分钟",
            "有效时间制",
            "节间休息",
            "加时赛规则",
        ]
    }

    /// 场地规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "场地尺寸: 30.5×15.25米",
            "球门柱高度: 3.05米",
            "球门无篮板",
            "三分区域",
            "场地划分",
        ]
    }

    /// 队员配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "每队7人上场",
            "位置固定",
            "GS: 射手",
            "GA: 攻击手",
            "其他位置",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "传球技术",
            "射门技术",
            "防守技术",
            "跑位技术",
            "步法规则",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "持球超过3秒",
            "步数违规",
            "接触犯规",
            "位置违规",
            " obstruction",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "网球",
            "比赛服装",
            "位置标识",
            "防护装备",
            "场地装备",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "球进入球门",
            "有效进球",
            "得分统计",
            "比分记录",
            "胜负判定",
        ]
    }
}

impl Default for NetballRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for NetballRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("netball")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【网球规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            犯规规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.court_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_netball_rules() {
        let rules = NetballRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}