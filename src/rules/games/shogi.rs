//! 将棋规则
use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;
simple_rule! { struct: ShogiRules, name: "将棋规则", desc: "日本将棋规则", origin: "日本", tags: ["游戏", "棋类"] }
impl ShogiRules {
    pub fn section_0(&self) -> Vec<&'static str> {
        vec![
            "9x9棋盘双方各20枚棋子",
            "棋子:王将/玉将飞车角行金将银将桂马香车步兵",
            "棋子初始位置固定排列",
        ]
    }

    pub fn section_1(&self) -> Vec<&'static str> {
        vec![
            "王将周围8格",
            "飞车横竖任意距离",
            "角行斜向任意距离",
            "金将周围6格",
            "银将前方和斜前方5格",
            "桂马前方两格+左右一格",
            "香车前方任意距离",
            "步兵前方一格",
        ]
    }

    pub fn section_2(&self) -> Vec<&'static str> {
        vec![
            "打入:吃掉的棋子可放回棋盘作为己方棋子",
            "升级:棋子进入敌方最后三排可升级(翻面)",
            "禁止打入步兵直接将死",
            "千日手:同一局面重复4次判和",
        ]
    }
}
impl Rule for ShogiRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }
    fn category(&self) -> RuleCategory {
        RuleCategory::games("shogi")
    }
    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }
    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "将棋规则",
            &[
                ("棋盘与棋子", &self.section_0()),
                ("走法", &self.section_1()),
                ("特殊规则", &self.section_2()),
            ],
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let r = ShogiRules::new();
        assert!(!r.explain().is_empty());
    }
}
