//! 花样滑冰规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 花样滑冰比赛规则 (详细版)
pub struct FigureSkatingDetailedRules {
    metadata: RuleMetadata,
}

impl FigureSkatingDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "花样滑冰比赛规则",
                "花样滑冰比赛详细规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "单人滑: 男子/女子",
            "双人滑",
            "冰舞",
            "团体赛",
            "花样滑冰综合",
        ]
    }

    /// 评分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "技术分数: TES",
            "节目内容分: PCS",
            "难度系数",
            "执行分数",
            "扣分项目",
        ]
    }

    /// 技术动作
    pub fn technical_elements(&self) -> Vec<&'static str> {
        vec![
            "跳跃: 各种跳跃动作",
            "旋转: 各种旋转姿势",
            "步法: 连接步法",
            " lifts: 双人托举",
            "螺旋线: 双人动作",
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
            "自由滑",
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

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "冰面安全检查",
            "医疗支持",
            "摔倒保护",
            "装备检查",
            "比赛中止规则",
        ]
    }
}

impl Default for FigureSkatingDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for FigureSkatingDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("figure_skating_detailed")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【花样滑冰比赛规则】\n\n\
            评分系统:\n{}\n\n\
            技术动作:\n{}\n\n\
            犯规扣分:\n{}\n\n\
            装备要求:\n{}\n",
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technical_elements().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.deductions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_figure_skating_detailed_rules() {
        let rules = FigureSkatingDetailedRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}