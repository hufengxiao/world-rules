//! 世界拳击组织规则
//!
//! WBO是四大拳击组织之一，拥有自己的世界冠军腰带和排名系统

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 世界拳击组织规则
pub struct BoxingWboRules {
    metadata: RuleMetadata,
}

impl BoxingWboRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("世界拳击组织规则", "WBO世界冠军赛和排名规则")
                .with_origin("波多黎各")
                .with_tags(vec!["体育".into(), "拳击".into(), "职业拳击".into()]),
        }
    }

    /// 重量级别
    pub fn weight_classes(&self) -> Vec<&'static str> {
        vec![
            "迷你蝇量级(105磅/47.6kg)",
            "轻蝇量级(108磅/49kg)",
            "蝇量级(112磅/50.8kg)",
            "超蝇量级(115磅/52.2kg)",
            "雏量级(118磅/53.5kg)",
            "超雏量级(122磅/55.3kg)",
            "羽量级(126磅/57.2kg)",
            "超羽量级(130磅/59kg)",
            "轻量级(135磅/61.2kg)",
            "超轻量级(140磅/63.5kg)",
            "次中量级(147磅/66.7kg)",
            "超次中量级(154磅/69.9kg)",
            "中量级(160磅/72.6kg)",
            "超中量级(168磅/76.2kg)",
            "轻重量级(175磅/79.4kg)",
            "次重量级(200磅/90.7kg)",
            "重量级(200磅以上)",
        ]
    }

    /// 冠军赛规则
    pub fn championship_rules(&self) -> Vec<&'static str> {
        vec![
            "12回合世界冠军赛",
            "每回合3分钟",
            "回合间休息1分钟",
            "三名裁判评分",
            "10分制评分系统",
            "必须称重通过才能参赛",
        ]
    }

    /// 排名系统
    pub fn ranking_system(&self) -> Vec<&'static str> {
        vec![
            "前15名排名拳手",
            "每月更新排名",
            "根据比赛成绩积分",
            "强制性挑战权",
            "拳手需保持活跃度",
            "地区冠军排名加分",
        ]
    }

    /// 挑战权规则
    pub fn mandatory_challenge(&self) -> Vec<&'static str> {
        vec![
            "第一名拳手获得强制挑战权",
            "冠军须在规定时间内接受挑战",
            "拒绝强制挑战将被剥夺头衔",
            "可协商延期最多一次",
            "临时头衔持有者需在180天内统一",
        ]
    }

    /// 药物检测
    pub fn drug_testing(&self) -> Vec<&'static str> {
        vec![
            "VADA清洁拳击计划",
            "赛前药检必做",
            "赛后随机抽检",
            "违禁物质零容忍",
            "阳性结果禁赛2年以上",
        ]
    }

    /// 禁止行为
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "击打后脑",
            "击打下体",
            "头撞",
            "肘击",
            "搂抱过久",
            "推搡对手",
            "用拳套边缘击打",
            "击打已倒地对手",
        ]
    }

    /// 击倒规则
    pub fn knockdown_rules(&self) -> Vec<&'static str> {
        vec![
            "被击倒后裁判开始数秒",
            "对手必须在中立角等候",
            "10秒内无法站起判定KO",
            "每回合三次击倒自动终止",
            "站起后裁判评估继续能力",
        ]
    }

    /// 胜利判定
    pub fn victory_methods(&self) -> Vec<&'static str> {
        vec![
            "判定胜利: 多数裁判评分",
            "KO胜利: 对手无法站起",
            "TKO胜利: 裁判终止比赛",
            "RTD胜利: 拳手团队放弃",
            "DQ胜利: 对手取消资格",
            "MD/SD胜利: 分歧判定",
        ]
    }
}

impl Default for BoxingWboRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BoxingWboRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("boxing_wbo")
    }

    fn explain(&self) -> String {
        format!(
            "【世界拳击组织规则】\n\n\
            重量级别:\n{}\n\n\
            冠军赛规则:\n{}\n\n\
            排名系统:\n{}\n\n\
            禁止行为:\n{}\n",
            self.weight_classes()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.championship_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.ranking_system()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.fouls()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boxing_wbo_rules() {
        let rules = BoxingWboRules::new();
        assert_eq!(rules.metadata().name, "世界拳击组织规则");
        assert!(!rules.weight_classes().is_empty());
        assert!(!rules.championship_rules().is_empty());
    }

    #[test]
    fn test_boxing_wbo_weight_classes() {
        let rules = BoxingWboRules::new();
        let classes = rules.weight_classes();
        assert!(classes.iter().any(|c| c.contains("重量级")));
        assert!(classes.iter().any(|c| c.contains("中量级")));
    }

    #[test]
    fn test_boxing_wbo_mandatory() {
        let rules = BoxingWboRules::new();
        assert!(!rules.mandatory_challenge().is_empty());
        assert!(rules.mandatory_challenge().contains(&"第一名拳手获得强制挑战权"));
    }
}