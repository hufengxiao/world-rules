//! 信号处理定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: SignalProcessingRules,
    name: "信号处理定律",
    desc: "信号处理定律",
    origin: "国际",
    tags: ["科学", "工程"],
    category: RuleCategory::science("signal_processing"),
    sections: [("变换", section_0), ("滤波", section_1)]
}

impl SignalProcessingRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["傅里叶变换", "小波变换"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["FIR滤波器", "卡尔曼滤波"]
    }
}
