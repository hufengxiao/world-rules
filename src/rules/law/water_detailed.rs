//! 水法详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: WaterDetailedRules,
    name: "水法详解",
    desc: "水法详解",
    origin: "中国",
    tags: ["法律", "资源"],
    category: RuleCategory::law("water_detailed"),
    sections: [("管理", section_0), ("保护", section_1)]
}

impl WaterDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["取水许可", "水权交易"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["水功能区", "饮用水源"]
    }
}
