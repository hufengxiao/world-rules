//! 散打规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 散打规则
pub struct SandaRules {
    metadata: RuleMetadata,
}

impl SandaRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "散打规则",
                "散打比赛基本规则"
            )
            .with_origin("中国")
            .with_tags(vec!["体育".into(), "格斗".into()]),
        }
    }

    /// 比赛场地
    pub fn arena_specifications(&self) -> Vec<&'static str> {
        vec![
            "擂台: 8米×8米正方形",
            "擂台高度: 0.6米",
            "围绳: 高出擂台0.9米",
            "地面: 垫子和帆布",
            "四个角落标识",
        ]
    }

    /// 比赛回合
    pub fn rounds(&self) -> Vec<&'static str> {
        vec![
            "每场比赛3回合",
            "每回合2分钟",
            "回合间休息1分钟",
            "决赛可增至5回合",
            "少年比赛缩短时间",
        ]
    }

    /// 体重级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "48公斤级",
            "52公斤级",
            "56公斤级",
            "60公斤级",
            "65公斤级",
            "70公斤级",
            "75公斤级",
            "80公斤级",
            "85公斤级",
            "90公斤级以上",
        ]
    }

    /// 允许技术
    pub fn permitted_techniques(&self) -> Vec<&'static str> {
        vec![
            "拳法: 直拳、摆拳、勾拳",
            "腿法: 蹬腿、踹腿、扫腿",
            "摔法: 接腿摔、贴身摔",
            "膝法: 仅限部分规则",
            "组合技术运用",
        ]
    }

    /// 得分规则
    pub fn scoring(&self) -> Vec<&'static str> {
        vec![
            "有效打击头部得2分",
            "有效打击躯干得1分",
            "成功摔倒对手得2分",
            "对手倒地得1分",
            "明显优势可加分",
        ]
    }

    /// 禁止动作
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "攻击后脑和颈部",
            "攻击眼睛",
            "肘击头部",
            "攻击倒地选手",
            "咬人和抓挠",
            "辱骂对手或裁判",
        ]
    }

    /// 犯规处罚
    pub fn penalties(&self) -> Vec<&'static str> {
        vec![
            "警告: 轻微犯规",
            "扣分: 严重犯规",
            "取消资格: 多次犯规",
            "医疗检查暂停",
            "判定胜负",
        ]
    }

    /// 裁判组成
    pub fn officials(&self) -> Vec<&'static str> {
        vec![
            "台上裁判: 比赛指挥",
            "边裁: 5人评分",
            "计时员: 时间控制",
            "记录员: 记录比赛",
            "医务监督: 安全保障",
        ]
    }
}

impl Default for SandaRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for SandaRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("sanda")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【散打规则】\n\n\
            允许技术:\n{}\n\n\
            得分规则:\n{}\n\n\
            禁止动作:\n{}\n\n\
            比赛回合:\n{}\n",
            self.permitted_techniques().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.scoring().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_actions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.rounds().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanda_rules() {
        let rules = SandaRules::new();
        assert!(!rules.permitted_techniques().is_empty());
    }
}