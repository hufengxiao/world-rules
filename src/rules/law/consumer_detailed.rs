//! 消费者权益详解
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ConsumerDetailedRules,
    name: "消费者权益详解",
    desc: "消费者权益法详解",
    origin: "中国",
    tags: ["法律", "消费"],
    category: RuleCategory::law("consumer_detailed"),
    sections: [("权利", section_0), ("救济", section_1)]
}

impl ConsumerDetailedRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["知情权", "选择权"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["七天无理由退货", "惩罚性赔偿"]
    }
}
