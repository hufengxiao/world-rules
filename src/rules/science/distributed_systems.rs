//! 分布式系统理论
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: DistributedSystemsRules,
    name: "分布式系统理论",
    desc: "分布式系统理论定律",
    origin: "国际",
    tags: ["科学", "计算机"],
    category: RuleCategory::science("distributed_systems"),
    sections: [("一致性", section_0), ("容错", section_1)]
}

impl DistributedSystemsRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["CAP定理", "Paxos算法", "Raft共识"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["拜占庭容错", "故障检测"]
    }
}
