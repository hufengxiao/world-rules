//! 花样滑冰冰舞规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 花样滑冰冰舞规则
pub struct IceDancingRules {
    metadata: RuleMetadata,
}

impl IceDancingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "花样滑冰冰舞规则",
                "花样滑冰冰舞比赛规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "韵律舞",
            "自由舞",
            "规定舞",
            "综合比赛",
            "团体比赛",
        ]
    }

    /// 评分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "技术分数: TES",
            "节目内容分: PCS",
            "韵律要求",
            "执行分数",
            "扣分项目",
        ]
    }

    /// 技术动作
    pub fn technical_elements(&self) -> Vec<&'static str> {
        vec![
            "舞步序列",
            " lifts托举",
            "旋转动作",
            "步法组合",
            "同步要求",
        ]
    }

    /// 节目内容
    pub fn program_components(&self) -> Vec<&'static str> {
        vec![
            "滑行技术",
            "连接动作",
            "表演执行",
            "编舞构成",
            "音乐诠释",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "短节目",
            "自由舞",
            "时间限制",
            "动作数量要求",
            "服装规定",
        ]
    }

    /// 犯规扣分
    pub fn deductions(&self) -> Vec<&'static str> {
        vec![
            "摔倒扣分",
            "时间超限扣分",
            "服装违规扣分",
            "动作违规扣分",
            "音乐违规扣分",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "花样滑冰鞋",
            "比赛服装",
            "无尖锐装饰",
            "服装整洁",
            "音乐选择",
        ]
    }
}

impl Default for IceDancingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for IceDancingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("ice_dancing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【花样滑冰冰舞规则】\n\n\
            比赛项目:\n{}\n\n\
            技术动作:\n{}\n\n\
            评分系统:\n{}\n\n\
            装备要求:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technical_elements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ice_dancing_rules() {
        let rules = IceDancingRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}