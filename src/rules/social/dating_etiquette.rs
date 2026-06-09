//! 约会礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: DatingEtiquetteRules,
    name: "约会礼仪",
    desc: "约会社交礼仪",
    origin: "国际",
    tags: ["社交", "约会"],
    category: RuleCategory::social("dating_etiquette"),
    sections: [("准备", section_0), ("行为", section_1)]
}

impl DatingEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["准时到达", "穿着得体", "准备话题"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["手机静音", "主动买单或AA", "尊重对方"]
    }
}
