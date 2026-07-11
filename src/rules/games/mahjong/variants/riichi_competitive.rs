//! 日本立直麻将竞技规则
//!
//! 正式竞技麻将规则，适用于锦标赛和专业比赛

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 日本立直麻将竞技规则
pub struct RiichiCompetitiveRules {
    metadata: RuleMetadata,
}

impl RiichiCompetitiveRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("日本立直麻将竞技规则", "专业比赛用立直麻将规则")
                .with_origin("日本")
                .with_tags(vec!["游戏".into(), "麻将".into(), "竞技".into()]),
        }
    }

    /// 基本设置
    pub fn basic_settings(&self) -> Vec<&'static str> {
        vec![
            "使用136张牌(无花牌)",
            "四人游戏，每人起手13张",
            "竞技规则严格执行",
            "禁止食替(吃打同一种牌)",
            "禁止振听立直",
        ]
    }

    /// 立直规则
    pub fn riichi_rules(&self) -> Vec<&'static str> {
        vec![
            "立直需支付1000点供托",
            "立直后禁止更改手牌",
            "立直后必须暗杠或荣和",
            "一发:立直后第一巡胡牌+1番",
            "两立直:配牌时双立直",
            "里宝牌:立直胡牌可翻里宝",
        ]
    }

    /// 竞技役种(格式化)
    pub fn competitive_yaku_formatted(&self) -> Vec<&'static str> {
        vec![
            "立直: 1番",
            "一发: 1番",
            "门前清自摸和: 1番",
            "平和: 1番",
            "断幺九: 1番",
            "役牌: 1番",
            "立直双倍: 2番",
            "三色同刻: 2番",
            "一气通贯: 2番",
            "对对和: 2番",
            "三暗刻: 2番",
            "混全带幺九: 2番",
            "七对子: 2番",
            "清全带幺九: 3番",
            "混一色: 3番",
            "纯全带幺九: 3番",
            "清一色: 6番",
        ]
    }

    /// 竞技役种
    pub fn competitive_yaku(&self) -> Vec<(&'static str, u8)> {
        vec![
            // 基本役
            ("立直", 1),
            ("门前清自摸和", 1),
            ("断幺九", 1),
            ("平和", 1),
            ("一杯口", 1),
            ("役牌", 1),
            // 中级役
            ("三色同顺", 2),
            ("一气通贯", 2),
            ("混全带幺九", 2),
            ("七对子", 2),
            ("对对和", 2),
            ("三暗刻", 2),
            ("三色同刻", 2),
            ("混老头", 2),
            // 高级役
            ("两立直", 2),
            ("三色同顺(副露)", 1),
            ("一气通贯(副露)", 1),
            ("纯全带幺九", 3),
            ("混一色", 3),
            ("清一色", 6),
        ]
    }

    /// 役满规则
    pub fn yakuman_rules(&self) -> Vec<&'static str> {
        vec![
            "天和:庄家起手胡牌",
            "地和:闲家第一巡胡牌",
            "大三元:三种三元牌刻子",
            "四暗刻:四个暗刻",
            "字一色:全字牌",
            "绿一色:全绿牌(23468条)",
            "清老头:全幺九牌",
            "国士无双:十三幺",
            "小四喜:三种风牌刻子+一对",
            "大四喜:四种风牌刻子",
            "九莲宝灯:同花色特定牌型",
        ]
    }

    /// 竞技计分
    pub fn scoring_rules(&self) -> Vec<&'static str> {
        vec![
            "符数计算严格按规则",
            "番符制计分",
            "切上满贯:4番30符或3番60符",
            "跳满:6-7番",
            "倍满:8-10番",
            "三倍满:11-12番",
            "役满:13番以上或役满役",
            "累计役满:多个役满叠加",
        ]
    }

    /// 流局规则
    pub fn draw_rules(&self) -> Vec<&'static str> {
        vec![
            "荒牌流局:牌墙摸完",
            "流局满贯:摸完所有牌后结算",
            "四风连打:第一巡四家打同风牌",
            "四立直:四家立直",
            "四杠子:四家开杠",
            "九种九牌:起手九种幺九牌",
            "三家和:三家同时荣和",
        ]
    }

    /// 禁止事项
    pub fn prohibited_actions(&self) -> Vec<&'static str> {
        vec![
            "禁止食替",
            "禁止振听立直",
            "禁止食替暗刻",
            "禁止故意拖延",
            "禁止暗号交流",
            "禁止偷看他人手牌",
            "违规处罚点数",
        ]
    }
}

impl Rule for RiichiCompetitiveRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("mahjong_riichi_competitive")
    }

    fn validate(
        &self,
        _ctx: &crate::rules::core::ValidateContext,
    ) -> crate::rules::core::RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "日本立直麻将竞技规则",
            &[
                ("基本设置", &self.basic_settings()),
                ("立直规则", &self.riichi_rules()),
                ("竞技役种", &self.competitive_yaku_formatted()),
                ("役满规则", &self.yakuman_rules()),
                ("计分规则", &self.scoring_rules()),
                ("流局规则", &self.draw_rules()),
                ("禁止事项", &self.prohibited_actions()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let rules = RiichiCompetitiveRules::new();
        assert_eq!(rules.metadata().name, "日本立直麻将竞技规则");
        assert!(!rules.explain().is_empty());
    }
}
