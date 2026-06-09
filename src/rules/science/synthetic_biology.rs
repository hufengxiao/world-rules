//! 合成生物学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SyntheticBiologyRules,
    name: "合成生物学定律",
    desc: "合成生物学定律",
    origin: "国际",
    tags: ["科学", "生物"],
    category: RuleCategory::science("synthetic_biology"),
    sections: [("设计原则", section_0), ("应用", section_1)]
}

impl SyntheticBiologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["模块化设计", "标准化生物部件", "正交性原则"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["生物燃料", "生物传感器", "基因治疗"]
    }
}
