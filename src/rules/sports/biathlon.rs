//! 冬季两项规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 冬季两项规则 (滑雪射击)
pub struct BiathlonRules {
    metadata: RuleMetadata,
}

impl BiathlonRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "冬季两项规则",
                "滑雪射击组合运动规则"
            )
            .with_origin("挪威")
            .with_tags(vec!["体育".into(), "冬季".into()]),
        }
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "10公里短距离",
            "12.5公里追逐赛",
            "15公里个人赛",
            "20公里个人赛",
            "4×7.5公里接力",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "滑雪射击交替",
            "射击位置",
            "罚圈规则",
            "时间计算",
            "出发规则",
        ]
    }

    /// 射击规则
    pub fn shooting_rules(&self) -> Vec<&'static str> {
        vec![
            "靶标距离: 50米",
            "卧射与立射",
            "靶标尺寸",
            "射击次数",
            "罚圈距离: 150米",
        ]
    }

    /// 滑雪规则
    pub fn skiing_rules(&self) -> Vec<&'static str> {
        vec![
            "自由技术",
            "滑行技术",
            "路线要求",
            "装备交换",
            "时间限制",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "滑雪装备",
            "步枪装备",
            "射击靶标",
            "防护装备",
            "服装要求",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "步枪安全",
            "射击安全",
            "滑雪安全",
            "医疗支持",
            "应急处理",
        ]
    }

    /// 评分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "总时间计算",
            "罚圈时间",
            "射击准确率",
            "排名规则",
            "同分处理",
        ]
    }
}

impl Default for BiathlonRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BiathlonRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("biathlon")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【冬季两项规则】\n\n\
            比赛项目:\n{}\n\n\
            射击规则:\n{}\n\n\
            装备要求:\n{}\n\n\
            安全规则:\n{}\n",
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.shooting_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biathlon_rules() {
        let rules = BiathlonRules::new();
        assert!(!rules.competition_events().is_empty());
    }
}