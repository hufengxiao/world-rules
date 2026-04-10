//! 撑杆跳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 撑杆跳规则
pub struct PoleVaultRules {
    metadata: RuleMetadata,
}

impl PoleVaultRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "撑杆跳规则",
                "撑杆跳比赛基本规则"
            )
            .with_origin("英国")
            .with_tags(vec!["体育".into(), "田径".into()]),
        }
    }

    /// 场地规格
    pub fn field_specifications(&self) -> Vec<&'static str> {
        vec![
            "助跑道长: 至少40米",
            "助跑道宽: 1.22米",
            "插斗长度: 1米",
            "插斗宽度: 0.6米",
            "落地垫面积足够",
        ]
    }

    /// 撑杆规格
    pub fn pole_specifications(&self) -> Vec<&'static str> {
        vec![
            "撑杆材质: 碳纤维或玻璃纤维",
            "长度: 根据运动员选择",
            "直径: 规定范围",
            "弯曲度",
            "运动员自带",
        ]
    }

    /// 横杆规格
    pub fn crossbar_specifications(&self) -> Vec<&'static str> {
        vec![
            "横杆长度: 4.5米",
            "横杆重量: 不超过2.5公斤",
            "两端支架支撑",
            "横杆可旋转",
            "横杆高度可调整",
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
            "可以更改撑杆",
            "连续失败淘汰",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "横杆掉落",
            "触碰横杆支架",
            "手在撑杆上移动",
            "越过横杆后触碰",
            "未完成动作",
        ]
    }

    /// 技术要求
    pub fn technique(&self) -> Vec<&'static str> {
        vec![
            "助跑速度",
            "插杆时机",
            "摆动技术",
            "转体过杆",
            "安全落地",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "落地垫必须足够厚",
            "插斗安全检查",
            "撑杆检查",
            "裁判监督",
            "医疗支持",
        ]
    }
}

impl Default for PoleVaultRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PoleVaultRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("pole_vault")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【撑杆跳规则】\n\n\
            场地规格:\n{}\n\n\
            比赛规则:\n{}\n\n\
            犯规规则:\n{}\n\n\
            技术要求:\n{}\n",
            self.field_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.fouls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.technique().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pole_vault_rules() {
        let rules = PoleVaultRules::new();
        assert!(!rules.field_specifications().is_empty());
    }
}