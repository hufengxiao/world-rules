//! 七项全能规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 七项全能规则
pub struct HeptathlonRules {
    metadata: RuleMetadata,
}

impl HeptathlonRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "七项全能规则",
                "田径七项全能比赛规则"
            )
            .with_origin("美国")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "第一天: 100米栏、跳高、铅球、200米",
            "第二天: 跳远、标枪、800米",
            "七个项目总计",
            "两天完成",
            "积分计算",
        ]
    }

    /// 积分系统
    pub fn scoring_system(&self) -> Vec<&'static str> {
        vec![
            "国际田联积分表",
            "成绩换算积分",
            "项目积分公式",
            "总积分排名",
            "积分下限",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "两天比赛安排",
            "项目间隔",
            "休息时间",
            "犯规规则",
            "退赛规则",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "径赛犯规",
            "田赛犯规",
            "起跑犯规",
            "试跳失败",
            "试投失败",
        ]
    }

    /// 成绩记录
    pub fn records(&self) -> Vec<&'static str> {
        vec![
            "每项成绩记录",
            "积分计算",
            "总积分",
            "排名规则",
            "记录标准",
        ]
    }

    /// 休息规则
    pub fn rest_rules(&self) -> Vec<&'static str> {
        vec![
            "项目间休息",
            "比赛间隔",
            "补充营养",
            "医疗支持",
            "恢复时间",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "跑步装备",
            "跳跃装备",
            "投掷装备",
            "防护装备",
            "比赛服装",
        ]
    }
}

impl Default for HeptathlonRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HeptathlonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("heptathlon")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【七项全能规则】\n\n\
            比赛项目:\n{}\n\n\
            积分系统:\n{}\n\n\
            比赛规则:\n{}\n\n\
            犯规规则:\n{}\n",
            self.events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heptathlon_rules() {
        let rules = HeptathlonRules::new();
        assert!(!rules.events().is_empty());
    }
}