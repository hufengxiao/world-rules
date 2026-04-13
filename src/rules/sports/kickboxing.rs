//! 踢拳规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 踢拳规则
pub struct KickboxingRules {
    metadata: RuleMetadata,
}

impl KickboxingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "踢拳规则",
                "踢拳比赛基本规则"
            )
            .with_origin("日本")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛规则
    pub fn competition_rules(&self) -> Vec<&'static str> {
        vec![
            "比赛回合: 3-5回合",
            "每回合3分钟",
            "回合间休息1分钟",
            "积分制或KO制",
            "裁判评分",
        ]
    }

    /// 允许技术
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 直拳、勾拳、摆拳",
            "腿法: 扫踢、直踢、侧踢",
            "膝法: 仅限部分规则",
            "组合技术",
            "防守技术",
        ]
    }

    /// 禁止动作
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "攻击后脑和颈部",
            "攻击眼睛",
            "肘击",
            "攻击倒地选手",
            "咬人和抓挠",
        ]
    }

    /// 得分标准
    pub fn scoring_criteria(&self) -> Vec<&'static str> {
        vec![
            "有效打击得分",
            "清晰有力加分",
            "技术多样性",
            "场上控制",
            "进攻主动性",
        ]
    }

    /// 比赛场地
    pub fn ring_specifications(&self) -> Vec<&'static str> {
        vec![
            "擂台尺寸: 6-7米正方形",
            "擂台高度: 1.2米",
            "围绳: 4条",
            "地面垫子和帆布",
            "角落配色",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "拳套: 10盎司",
            "护齿必须",
            "短裤",
            "护腿可选",
            "护胸可选",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "赛前体检",
            "裁判监督",
            "医疗支持",
            "选手保护",
            "比赛中止规则",
        ]
    }

    /// 比赛形式
    pub fn competition_formats(&self) -> Vec<&'static str> {
        vec![
            "K-1规则",
            "全接触规则",
            "低踢规则",
            "国际规则",
            "业余规则",
        ]
    }
}

impl Default for KickboxingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for KickboxingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("kickboxing")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【踢拳规则】\n\n\
            允许技术:\n{}\n\n\
            禁止动作:\n{}\n\n\
            得分标准:\n{}\n\n\
            装备要求:\n{}\n",
            self.permitted_techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring_criteria().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.equipment().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kickboxing_rules() {
        let rules = KickboxingRules::new();
        assert!(!rules.permitted_techniques().is_empty());
    }
}