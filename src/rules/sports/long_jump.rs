//! 跳远规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 跳远规则
pub struct LongJumpRules {
    metadata: RuleMetadata,
}

impl LongJumpRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "跳远规则",
                "跳远比赛基本规则"
            )
            .with_origin("古代希腊")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "助跑道长度: 至少40米",
            "助跑道宽度: 1.22米",
            "起跳板长度: 1.22米",
            "起跳板宽度: 20厘米",
            "落地沙坑长度: 7-9米",
        ]
    }

    /// 起跳规则
    pub fn takeoff_rules(&self) -> Vec<&'static str> {
        vec![
            "必须在起跳板后起跳",
            "起跳板有犯规线",
            "犯规线宽10厘米",
            "踩过犯规线犯规",
            "正确起跳技术",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "每人3次试跳",
            "取最好成绩",
            "决赛8人",
            "决赛6次试跳",
            "最好成绩获胜",
        ]
    }

    /// 投掷技术
    pub fn jumping_techniques(&self) -> Vec<&'static str> {
        vec![
            "蹲踞式跳远",
            "挺身式跳远",
            "走步式跳远",
            "助跑速度关键",
            "起跳角度约18-24度",
        ]
    }

    /// 测量规则
    pub fn measurement(&self) -> Vec<&'static str> {
        vec![
            "从最近落地痕迹测量",
            "测量至起跳板前沿",
            "精确到0.01米",
            "金属卷尺测量",
            "测量员记录",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "踩过犯规线",
            "起跳板前起跳",
            "沙坑外落地",
            "试跳超时",
            "违规动作",
        ]
    }

    /// 风速限制
    pub fn wind_limits(&self) -> Vec<&'static str> {
        vec![
            "顺风超过2米/秒记录无效",
            "风速测量设备",
            "室内比赛无风速限制",
            "风速记录",
            "影响世界纪录认证",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "沙坑平整松软",
            "助跑道安全",
            "裁判监督",
            "医疗支持",
            "装备检查",
        ]
    }
}

impl Default for LongJumpRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for LongJumpRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("long_jump")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【跳远规则】\n\n\
            场地规格:\n{}\n\n\
            比赛规则:\n{}\n\n\
            投掷技术:\n{}\n\n\
            犯规规则:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.jumping_techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_long_jump_rules() {
        let rules = LongJumpRules::new();
        assert!(!rules.field_specifications().is_empty());
    }
}