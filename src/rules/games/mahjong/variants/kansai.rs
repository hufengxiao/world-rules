//! 关西麻将规则
//!
//! 关西地区流行麻将变体

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 关西麻将规则
pub struct KansaiMahjongRules {
    metadata: RuleMetadata,
}

impl KansaiMahjongRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("关西麻将规则", "日本关西地区流行麻将规则")
                .with_origin("日本关西")
                .with_tags(vec!["游戏".into(), "麻将".into(), "关西".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "四人游戏，每人起手13张",
            "保留立直基本规则",
            "地方特色役种",
            "重视速度和灵活",
        ]
    }

    /// 关西特色役种
    pub fn kansai_yaku(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基础役
            ("立直", 1),
            ("断幺九", 1),
            ("平和", 1),
            ("一杯口", 1),
            // 关西特色
            ("关西断幺", 1),
            ("快速立直", 2),
            ("大阪胡", 2),
            ("京都七对", 2),
            // 中级役
            ("三色同顺", 2),
            ("一气通贯", 2),
            ("混一色", 2),
            ("对对和", 2),
            // 高级役
            ("清一色", 5),
            ("关西清", 6),
        ]
    }

    /// 地方规则
    pub fn local_rules(&self) -> Vec<&'static str> {
        vec![
            "部分役种番数调整",
            "快速胡牌有奖励",
            "允许吃断幺",
            "立直规则宽松",
            "重视手牌灵活性",
        ]
    }

    /// 计分规则
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "基本计分同立直",
            "地方役加番",
            "快速胡牌加番",
            "简化符数计算",
            "关西地区特色",
        ]
    }

    /// 庄家规则
    pub fn banker_rules(&self) -> Vec<&'static str> {
        vec!["庄家胡牌连庄", "庄家番数翻倍", "关西式连庄", "更快轮庄节奏"]
    }

    /// 禁止事项
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec!["禁止恶意拖延", "禁止串通", "禁止偷看", "遵守地方礼仪"]
    }

    /// 文化特色
    pub fn cultural_notes(&self) -> Vec<&'static str> {
        vec![
            "大阪商业文化影响",
            "重视效率速度",
            "灵活多变风格",
            "关西方言交流",
            "地方麻将文化",
        ]
    }
}

impl Rule for KansaiMahjongRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_kansai")
    }

    fn validate(
        &self,
        _ctx: &crate::rules::core::ValidateContext,
    ) -> crate::rules::core::RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "关西麻将规则",
            &[
                ("基本设置", &self.basic_settings()),
                (
                    "关西特色役种",
                    &self
                        .kansai_yaku()
                        .iter()
                        .map(|(name, fan)| format!("{}: {}番", name, fan))
                        .collect::<Vec<_>>(),
                ),
                ("地方规则", &self.local_rules()),
                ("计分规则", &self.scoring_rules()),
                ("庄家规则", &self.banker_rules()),
                ("禁止事项", &self.prohibited_actions()),
                ("文化特色", &self.cultural_notes()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = KansaiMahjongRules::new();
        assert_eq!(rules.metadata().name, "关西麻将规则");
        assert!(!rules.explain().is_empty());
    }
}
