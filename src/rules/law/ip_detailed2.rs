//! 知识产权详解2
use crate::rules::core::RuleCategory;
use crate::simple_rule;

simple_rule! {
    struct: IpDetailed2Rules,
    name: "知识产权详解2",
    desc: "知识产权法详解2",
    origin: "中国",
    tags: ["法律", "知识产权"],
    category: RuleCategory::law("ip_detailed2"),
    sections: [("专利", section_0), ("商标", section_1)]
}

impl IpDetailed2Rules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec!["申请流程", "无效宣告"]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec!["注册条件", "异议程序"]
    }
}
