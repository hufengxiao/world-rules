//! 购物礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ShoppingEtiquetteRules,
    name: "购物礼仪",
    desc: "购物社交礼仪",
    origin: "国际",
    tags: ["社交", "消费"],
    category: RuleCategory::social("shopping_etiquette"),
    sections: [("试穿", section_0), ("排队", section_1)]
}

impl ShoppingEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["爱护商品", "归还原位", "不过度试穿"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["遵守排队秩序", "不插队", "结账时不玩手机"]
    }
}
