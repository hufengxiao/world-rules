//! 班卡西拉特规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 班卡西拉特规则 (印尼传统武术)
pub struct SilatRules {
    metadata: RuleMetadata,
}

impl SilatRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "班卡西拉特规则",
                "印尼传统武术规则"
            )
            .with_origin("印尼")
            .with_tags(vec!["体育".into(), "武术".into()]),
        }
    }

    /// 武术流派
    pub fn styles(&self) -> Vec<&'static str> {
        vec![
            "Pencak Silat",
            "Silat Harimau",
            "Silat Sera",
            "Silat Cimande",
            "各地流派",
        ]
    }

    /// 比赛项目
    pub fn competition_events(&self) -> Vec<&'static str> {
        vec![
            "单人对练",
            "双人对练",
            "套路表演",
            "实战比赛",
            "团体表演",
        ]
    }

    /// 技术动作
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法技术",
            "腿法技术",
            "摔法技术",
            "擒拿技术",
            "武器技法",
        ]
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛时间: 3分钟",
            "有效得分判定",
            "犯规扣分",
            "裁判评分",
            "安全保护",
        ]
    }

    /// 得分标准
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "有效打击得分",
            "摔倒对手得分",
            "技术表现分",
            "艺术表现分",
            "扣分项目",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "传统服装",
            "防护手套",
            "护具套装",
            "比赛场地",
            "传统武器",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "禁止危险动作",
            "裁判监督",
            "医疗支持",
            "护具检查",
            "比赛控制",
        ]
    }
}

impl Default for SilatRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SilatRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("silat")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【班卡西拉特规则】\n\n\
            武术流派:\n{}\n\n\
            比赛项目:\n{}\n\n\
            技术动作:\n{}\n\n\
            得分标准:\n{}\n",
            self.styles().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.competition_events().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_silat_rules() {
        let rules = SilatRules::new();
        assert!(!rules.styles().is_empty());
    }
}