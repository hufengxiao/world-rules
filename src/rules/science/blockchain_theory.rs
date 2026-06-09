//! 区块链理论
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: BlockchainTheoryRules,
    name: "区块链理论",
    desc: "区块链技术理论定律",
    origin: "国际",
    tags: ["科学", "计算机"],
    category: RuleCategory::science("blockchain_theory"),
    sections: [("共识机制", section_0), ("密码学", section_1)]
}

impl BlockchainTheoryRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["工作量证明", "权益证明", "拜占庭容错"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["哈希函数", "默克尔树", "数字签名"]
    }
}
