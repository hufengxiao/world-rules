//! 三级跳远规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 三级跳远规则
pub struct TripleJumpRules {
    metadata: RuleMetadata,
}

impl TripleJumpRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "三级跳远规则",
                "三级跳远比赛基本规则"
            )
            .with_origin("爱尔兰")
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

    /// 三跳顺序
    pub fn jump_sequence(&self) -> Vec<&'static str> {
        vec![
            "第一跳: 单脚跳",
            "第二跳: 跨步跳",
            "第三跳: 跳跃",
            "必须保持节奏",
            "每跳衔接顺畅",
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

    /// 技术要点
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "单脚跳: 同脚起跳落地",
            "跨步跳: 换脚落地",
            "跳跃: 双脚落地沙坑",
            "三跳比例优化",
            "保持速度和节奏",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "踩过犯规线",
            "单脚跳后换脚",
            "跨步跳后触地",
            "沙坑外落地",
            "试跳超时",
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

    /// 世界纪录
    pub fn world_records(&self) -> Vec<&'static str> {
        vec![
            "男子: 18.29米(爱德华兹)",
            "女子: 15.67米(伊巴古恩)",
            "室内纪录不同",
            "不断更新",
            "各国纪录",
        ]
    }
}

impl Default for TripleJumpRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for TripleJumpRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("triple_jump")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【三级跳远规则】\n\n\
            三跳顺序:\n{}\n\n\
            技术要点:\n{}\n\n\
            犯规规则:\n{}\n\n\
            比赛规则:\n{}\n",
            self.jump_sequence().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technique().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triple_jump_rules() {
        let rules = TripleJumpRules::new();
        assert!(!rules.jump_sequence().is_empty());
    }
}