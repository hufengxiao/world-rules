//! 宠物礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: PetEtiquetteRules,
    name: "宠物礼仪",
    desc: "养宠社交礼仪",
    origin: "国际",
    tags: ["社交", "宠物"],
    category: RuleCategory::social("pet_etiquette"),
    sections: [("外出", section_0), ("公共场所", section_1)]
}

impl PetEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["牵绳遛狗", "清理宠物粪便", "避开怕动物的人"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["不带入餐厅", "控制宠物行为", "防止吠叫扰民"]
    }
}
