//! 麻将礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MahjongEtiquetteRules,
    name: "麻将礼仪",
    desc: "麻将桌上社交礼仪",
    origin: "中国",
    tags: ["社交", "游戏"],
    category: RuleCategory::social("mahjong_etiquette"),
    sections: [("行为", section_0), ("沟通", section_1)]
}

impl MahjongEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["不偷看他人牌", "不故意拖延", "输赢保持风度"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不暗示他人", "不议论牌局", "尊重对手"]
    }
}
