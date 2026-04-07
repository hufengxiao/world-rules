//! 台球规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 台球类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BilliardsType {
    /// 中式八球
    ChineseEightBall,
    /// 九球
    NineBall,
    /// 斯诺克
    Snooker,
}

impl BilliardsType {
    pub fn name(&self) -> &'static str {
        match self {
            BilliardsType::ChineseEightBall => "中式八球",
            BilliardsType::NineBall => "九球",
            BilliardsType::Snooker => "斯诺克",
        }
    }
}

/// 台球规则
pub struct BilliardsRules {
    metadata: RuleMetadata,
    billiards_type: BilliardsType,
}

impl BilliardsRules {
    pub fn new(billiards_type: BilliardsType) -> Self {
        Self {
            metadata: RuleMetadata::new(
                format!("{}规则", billiards_type.name()),
                "台球运动规则"
            )
            .with_origin("国际"),
            billiards_type,
        }
    }

    /// 球数
    pub fn ball_count(&self) -> u8 {
        match self.billiards_type {
            BilliardsType::ChineseEightBall => 16, // 1-15号+白球
            BilliardsType::NineBall => 10,         // 1-9号+白球
            BilliardsType::Snooker => 22,          // 15红+6彩+白球
        }
    }

    /// 基本规则
    pub fn basic_rules(&self) -> Vec<&'static str> {
        match self.billiards_type {
            BilliardsType::ChineseEightBall => vec![
                "15个目标球+1个白球",
                "分为全色球(1-7)和花色球(9-15)",
                "先打进自己球组，再打黑8获胜",
                "白球进袋或飞出台面为犯规",
            ],
            BilliardsType::NineBall => vec![
                "9个目标球+1个白球",
                "必须从号码最小的球开始打",
                "将9号球打进即获胜",
            ],
            BilliardsType::Snooker => vec![
                "15个红球+6个彩球+1个白球",
                "红球1分，黄2分，绿3分，棕4分",
                "蓝5分，粉6分，黑7分",
                "必须红球和彩球交替打",
            ],
        }
    }

    /// 犯规规则
    pub fn foul_rules(&self) -> Vec<&'static str> {
        vec![
            "白球进袋",
            "白球飞出台面",
            "先击中错误的球",
            "没有碰到任何球",
            "连击",
        ]
    }

    /// 计分规则(斯诺克)
    pub fn snooker_scoring(&self) -> Vec<(&'static str, u8)> {
        vec![
            ("红球", 1),
            ("黄球", 2),
            ("绿球", 3),
            ("棕球", 4),
            ("蓝球", 5),
            ("粉球", 6),
            ("黑球", 7),
        ]
    }

    /// 比赛形式
    pub fn match_formats(&self) -> Vec<&'static str> {
        match self.billiards_type {
            BilliardsType::ChineseEightBall => vec![
                "单败淘汰赛",
                "双败淘汰赛",
                "团体赛",
            ],
            BilliardsType::NineBall => vec![
                "抢局制(如抢9)",
                "长局制(决赛)",
            ],
            BilliardsType::Snooker => vec![
                "抢局制(如抢10)",
                "长局制(世锦赛决赛抢18)",
            ],
        }
    }
}

impl Rule for BilliardsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("billiards")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【{}规则】\n\n\
            球数: {}个\n\n\
            基本规则:\n{}\n\n\
            犯规规则:\n{}\n",
            self.billiards_type.name(),
            self.ball_count(),
            self.basic_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.foul_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_billiards_rules() {
        let rules = BilliardsRules::new(BilliardsType::ChineseEightBall);
        assert_eq!(rules.ball_count(), 16);
    }
}