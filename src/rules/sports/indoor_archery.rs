//! 射箭比赛规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 射箭比赛规则 - 室内射箭
pub struct IndoorArcheryRules {
    metadata: RuleMetadata,
}

impl IndoorArcheryRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "室内射箭规则",
                "室内射箭比赛基本规则"
            )
            .with_origin("国际")
            .with_tags(vec!["体育".into(), "射箭".into()]),
        }
    }

    /// 比赛距离
    pub fn distances(&self) -> Vec<&'static str> {
        vec![
            "18米: 标准室内距离",
            "25米: 另一室内距离",
            "室内靶标准",
            "室内场地限制",
            "光线控制",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "单轮比赛",
            "双轮比赛",
            "个人赛",
            "团体赛",
            "混合赛",
        ]
    }

    /// 箭数规则
    pub fn arrow_rules(&self) -> Vec<&'static str> {
        vec![
            "每轮36箭",
            "每组3箭",
            "时间限制",
            "记分规则",
            "箭数累计",
        ]
    }

    /// 靶面规格
    pub fn target_specifications(&self) -> Vec<&'static str> {
        vec![
            "直径40厘米",
            "10环同心圆",
            "颜色区分",
            "得分区域",
            "靶面更换",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "最高10分",
            "环值对应分数",
            "X环: 10分中心",
            "总成绩计算",
            "同分判定",
        ]
    }

    /// 时间规则
    pub fn time_limits(&self) -> Vec<&'static str> {
        vec![
            "每箭2分钟",
            "每组10分钟",
            "计时设备",
            "超时处罚",
            "裁判控制",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "弓具标准",
            "箭具标准",
            "辅助装备",
            "服装要求",
            "安全装备",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "统一射箭指挥",
            "停止信号遵守",
            "安全区域",
            "医疗支持",
            "设备检查",
        ]
    }
}

impl Default for IndoorArcheryRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for IndoorArcheryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("indoor_archery")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【室内射箭规则】\n\n\
            比赛距离:\n{}\n\n\
            得分规则:\n{}\n\n\
            时间规则:\n{}\n\n\
            安全规则:\n{}\n",
            self.distances().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.time_limits().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indoor_archery_rules() {
        let rules = IndoorArcheryRules::new();
        assert!(!rules.distances().is_empty());
    }
}