//! 海洋生物学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: MarineBiologyRules,
    name: "海洋生物学定律",
    desc: "海洋生物学定律",
    origin: "国际",
    tags: ["科学", "生物"],
    category: RuleCategory::science("marine_biology"),
    sections: [("生态系统", section_0), ("环境", section_1)]
}

impl MarineBiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["珊瑚礁生态", "深海热泉生态", "潮间带生态"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["海洋酸化", "海平面上升", "海洋污染"]
    }
}
