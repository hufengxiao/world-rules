//! 高尔夫社交礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: GolfEtiquetteRules,
    name: "高尔夫社交礼仪",
    desc: "高尔夫球场社交礼仪",
    origin: "国际",
    tags: ["社交", "运动"],
    category: RuleCategory::social("golf_etiquette"),
    sections: [("球场", section_0), ("速度", section_1)]
}

impl GolfEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["保持安静", "修复球痕", "不踩推击线"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["保持打球节奏", "让后组先行", "准备好再打"]
    }
}
