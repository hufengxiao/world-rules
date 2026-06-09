//! 老年健康规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ElderlyHealthRules,
    name: "老年健康规则",
    desc: "老年健康管理规则",
    origin: "国际",
    tags: ["健康", "老年"],
    category: RuleCategory::health("elderly_health"),
    sections: [("运动", section_0), ("防跌", section_1)]
}

impl ElderlyHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["太极拳", "散步", "平衡训练"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["家居改造", "穿防滑鞋", "定期检查视力"]
    }
}
