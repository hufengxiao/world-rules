//! 飞镖详细规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 飞镖详细规则
pub struct DartsDetailedRules {
    metadata: RuleMetadata,
}

impl DartsDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "飞镖详细规则",
                "飞镖比赛详细规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "休闲".into()]),
        }
    }

    /// 比赛类型
    pub fn competition_types(&self) -> Vec<&'static str> {
        vec![
            "501比赛",
            "301比赛",
            "板球比赛",
            "团体比赛",
            "比赛分类",
        ]
    }

    /// 飞镖盘规格
    pub fn board_specifications(&self) -> Vec<&'static str> {
        vec![
            "镖盘直径: 451毫米",
            "靶心直径: 12.7毫米",
            "外靶心直径: 31.8毫米",
            "20个分区",
            "得分区域",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "靶心: 50分",
            "外靶心: 25分",
            "三倍区: 3倍得分",
            "双倍区: 2倍得分",
            "普通区: 正常得分",
        ]
    }

    /// 501规则
    pub fn x01_rules(&self) -> Vec<&'static str> {
        vec![
            "起始分数501/301",
            "减分到零",
            "必须双倍结束",
            "爆镖规则",
            "比赛轮次",
        ]
    }

    /// 板球规则
    pub fn cricket_rules(&self) -> Vec<&'static str> {
        vec![
            "目标数字: 15-20和靶心",
            "三个标记获胜",
            "关闭数字",
            "得分规则",
            "比赛结束",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "握镖技术",
            "投掷技术",
            "瞄准技术",
            "站立姿势",
            "节奏控制",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "飞镖",
            "镖盘",
            "比赛服装",
            "计分板",
            "附属装备",
        ]
    }
}

impl Default for DartsDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DartsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("darts_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【飞镖详细规则】\n\n\
            比赛类型:\n{}\n\n\
            得分规则:\n{}\n\n\
            501规则:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_types().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.x01_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_darts_detailed_rules() {
        let rules = DartsDetailedRules::new();
        assert!(!rules.competition_types().is_empty());
    }
}