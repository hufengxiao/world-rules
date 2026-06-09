//! 飞机礼仪
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: AirplaneEtiquetteRules,
    name: "飞机礼仪",
    desc: "飞机乘坐礼仪",
    origin: "国际",
    tags: ["社交", "旅行"],
    category: RuleCategory::social("airplane_etiquette"),
    sections: [("登机", section_0), ("飞行中", section_1)]
}

impl AirplaneEtiquetteRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["按区域排队", "快速入座", "行李放好"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["调低音量", "不脱鞋", "适度使用卫生间"]
    }
}
