//! 袋球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 袋球规则 (Cornhole)
pub struct CornholeRules {
    metadata: RuleMetadata,
}

impl CornholeRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "袋球规则",
                "美国袋球运动规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "休闲".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛采用21分制",
            "轮流投掷",
            "每轮4袋",
            "得分规则",
            "比赛结束",
        ]
    }

    /// 场地规格
    pub fn court_specifications(&self) -> Vec<&'static str> {
        vec![
            "板框距离: 27英尺",
            "板框尺寸: 24×48英寸",
            "孔径: 6英寸",
            "投掷区域",
            "场地布置",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "投掷技术",
            "瞄准技术",
            "滑袋技术",
            "控制技术",
            "节奏控制",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "入孔: 3分",
            "板上: 1分",
            "取消规则",
            "得分计算",
            "比分记录",
        ]
    }

    /// 袋的规格
    pub fn bag_specifications(&self) -> Vec<&'static str> {
        vec![
            "袋尺寸: 6×6英寸",
            "袋重量: 14-16盎司",
            "材质要求",
            "颜色区分",
            "规格认证",
        ]
    }

    /// 团队配置
    pub fn team_composition(&self) -> Vec<&'static str> {
        vec![
            "单人比赛",
            "双人比赛",
            "每队4袋",
            "队员轮换",
            "比赛顺序",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "板框",
            "投掷袋",
            "场地装备",
            "计分板",
            "比赛服装",
        ]
    }
}

impl Default for CornholeRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for CornholeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("cornhole")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【袋球规则】\n\n\
            场地规格:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.court_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
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
    fn test_cornhole_rules() {
        let rules = CornholeRules::new();
        assert!(!rules.competition_rules().is_empty());
    }
}