//! 传统节日家庭团聚
//!
//! 中秋等传统节日家庭团聚赏月聚餐的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: FestivalFamilyGatheringRules,
    name: "传统节日家庭团聚",
    desc: "中秋等传统节日家庭团聚赏月聚餐的礼仪",
    origin: "中国",
    tags: ["社交", "礼仪", "中秋", "团聚"]
}

impl FestivalFamilyGatheringRules {
    /// 团聚安排
    pub fn plan(&self) -> Vec<&'static str> {
        vec![
            "提前安排团聚时间",
            "体谅家庭成员忙闲",
            "邀请长辈参与",
            "辅以清洁布置",
        ]
    }

    /// 餐桌共享
    pub fn meal(&self) -> Vec<&'static str> {
        vec![
            "共同分享月饼佳肴",
            "先礼让长辈开动",
            "适量取食不浪费",
            "欢聚谈论融洽",
        ]
    }

    /// 活动氛围
    pub fn activity(&self) -> Vec<&'static str> {
        vec![
            "赏月聊天添温馨",
            "陪长辈多聊聊",
            "拍照留念适时",
            "体恤家人放松",
        ]
    }

    /// 礼尚往来
    pub fn gift(&self) -> Vec<&'static str> {
        vec![
            "带合适礼品拜访",
            "体谅礼数适度",
            "团聚重于形式",
            "情意融融满载而归",
        ]
    }
}

impl Rule for FestivalFamilyGatheringRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("mooncake_gathering")
    }

    fn explain(&self) -> String {
        format!(
            "【传统节日家庭团聚】\n{}",
            [
                format!(
                    "团聚安排：\\n{}",
                    self.plan()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "餐桌共享：\\n{}",
                    self.meal()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "活动氛围：\\n{}",
                    self.activity()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "礼尚往来：\\n{}",
                    self.gift()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_festivalfamilygatheringrules_basic() {
        let rules = FestivalFamilyGatheringRules::new();
        assert_eq!(rules.metadata().name, "传统节日家庭团聚");
        assert!(!rules.plan().is_empty());
        assert!(!rules.meal().is_empty());
        assert!(!rules.activity().is_empty());
        assert!(!rules.gift().is_empty());
    }

    #[test]
    fn test_festivalfamilygatheringrules_validation() {
        let rules = FestivalFamilyGatheringRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("mooncake_gathering"));
    }

    #[test]
    fn test_festivalfamilygatheringrules_explain() {
        let rules = FestivalFamilyGatheringRules::new();
        let e = rules.explain();
        assert!(e.contains("团聚安排"));
        assert!(e.contains("餐桌共享"));
        assert!(e.contains("活动氛围"));
    }
}
