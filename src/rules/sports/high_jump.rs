//! 跳高规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 跳高规则
pub struct HighJumpRules {
    metadata: RuleMetadata,
}

impl HighJumpRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "跳高规则",
                "跳高比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "助跑道长度: 至少15米",
            "助跑道宽度: 不规定",
            "跳高架高度: 可调整",
            "落地垫面积足够",
            "横杆长度: 4米",
        ]
    }

    /// 横杆规格
    pub fn crossbar_specifications(&self) -> Vec<&'static str> {
        vec![
            "横杆长度: 4米",
            "横杆重量: 不超过2公斤",
            "两端支架支撑",
            "横杆可旋转",
            "高度调整规则",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "运动员选择高度",
            "每个高度3次试跳机会",
            "失败3次淘汰",
            "最后高度获胜",
            "同高度比较失败次数",
        ]
    }

    /// 试跳规则
    pub fn attempt_rules(&self) -> Vec<&'static str> {
        vec![
            "每次试跳限时1分钟",
            "可以请求调整高度",
            "可以放弃某高度",
            "连续失败淘汰",
            "高度递增规则",
        ]
    }

    /// 投掷技术
    pub fn jumping_techniques(&self) -> Vec<&'static str> {
        vec![
            "背越式跳高: 最常用",
            "剪式跳高",
            "俯卧式跳高",
            "跨越式跳高",
            "助跑和起跳配合",
        ]
    }

    /// 有效跳
    pub fn valid_jump(&self) -> Vec<&'static str> {
        vec![
            "成功越过横杆",
            "横杆不掉落",
            "单脚起跳",
            "安全落地",
            "动作完成",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "横杆掉落",
            "双脚起跳",
            "触碰支架",
            "试跳超时",
            "未完成动作",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "落地垫足够厚",
            "场地平整",
            "裁判监督",
            "医疗支持",
            "装备检查",
        ]
    }
}

impl Default for HighJumpRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for HighJumpRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("high_jump")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【跳高规则】\n\n\
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
    fn test_high_jump_rules() {
        let rules = HighJumpRules::new();
        assert!(!rules.field_specifications().is_empty());
    }
}