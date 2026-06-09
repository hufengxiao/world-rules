//! 皮肤健康规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SkinHealthRules,
    name: "皮肤健康规则",
    desc: "皮肤健康护理规则",
    origin: "国际",
    tags: ["健康", "护肤"],
    category: RuleCategory::health("skin_health"),
    sections: [("清洁", section_0), ("防晒", section_1)]
}

impl SkinHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["温和洁面", "不过度清洁", "卸妆彻底"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["日常防晒SPF30+", "补涂防晒", "物理防晒"]
    }
}
