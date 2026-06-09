//! 口腔健康规则
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: OralHealthRules,
    name: "口腔健康规则",
    desc: "口腔健康护理规则",
    origin: "国际",
    tags: ["健康", "口腔"],
    category: RuleCategory::health("oral_health"),
    sections: [("刷牙", section_0), ("检查", section_1)]
}

impl OralHealthRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["早晚刷牙每次3分钟", "正确刷牙方法", "定期换牙刷"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["半年洗牙一次", "定期口腔检查", "发现问题及时治疗"]
    }
}
