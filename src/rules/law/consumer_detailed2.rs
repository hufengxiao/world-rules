//! 消费者权益详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: ConsumerDetailed2Rules,
    name: "消费者权益详解2",
    desc: "消费者权益法详解2",
    origin: "中国",
    tags: ["法律", "消费"],
    category: RuleCategory::law("consumer_detailed2"),
    sections: [("三包", section_0), ("维权", section_1)]
}

impl ConsumerDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["修理", "更换", "退货"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["投诉", "调解", "仲裁", "诉讼"]
    }
}
