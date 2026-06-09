//! 拓扑学定律
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: TopologyRules,
    name: "拓扑学定律",
    desc: "拓扑学定律",
    origin: "国际",
    tags: ["科学", "数学"],
    category: RuleCategory::science("topology"),
    sections: [("基本概念", section_0), ("定理", section_1)]
}

impl TopologyRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["开集与闭集", "连续映射", "同胚"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["布劳威尔不动点定理", "欧拉示性数", "若尔当曲线定理"]
    }
}
