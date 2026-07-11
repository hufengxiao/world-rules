//! 中国传统成年礼仪 - 冠礼与笄礼
//!
//! 冠礼是中国古代男子的成年礼，笄礼是女子的成年礼。
//! 这些礼仪标志着青少年正式步入成年，承担社会责任。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseComingOfAgeRules,
    name: "中国传统成年礼",
    desc: "冠礼笄礼 - 古代成人礼仪式规范",
    origin: "中国",
    tags: ["社交", "礼仪", "成年礼", "传统"]
}

impl ChineseComingOfAgeRules {
    /// 冠礼流程（男子成年礼）
    pub fn guanli_procedure(&self) -> Vec<&'static str> {
        vec![
            "冠礼于二十岁举行",
            "前期三日，主人告于庙",
            "设洗于阼阶东南",
            "宾至，主人迎于门外",
            "冠者立于席中，宾揖之",
            "初加缁布冠，祝曰：令月吉日，始加元服",
            "再加皮弁，祝曰：吉月令辰，乃申尔服",
            "三加爵弁，祝曰：以岁之正，以月之令",
            "冠者见母，母拜之",
            "宾字冠者，赐以表字",
            "冠者见于兄弟、姑姊",
            "冠者见于君、卿大夫、乡先生",
        ]
    }

    /// 笄礼流程（女子成年礼）
    pub fn jili_procedure(&self) -> Vec<&'static str> {
        vec![
            "笄礼于十五岁举行",
            "择日，主人告于庙",
            "设席于房中，设洗于阼阶东南",
            "宾至，主人迎于门外",
            "笄者立于席中，宾揖之",
            "初加发笄，祝曰：令月吉日，始加元服",
            "再加发簪，祝曰：吉月令辰，乃申尔服",
            "三加钗冠，祝曰：以岁之正，以月之令",
            "笄者见母，母拜之",
            "宾字笄者，赐以表字",
            "笄者见于兄弟、姑姊",
        ]
    }

    /// 冠礼三加意义
    pub fn guanli_meaning(&self) -> Vec<&'static str> {
        vec![
            "初加缁布冠 - 保有古风，不忘本始",
            "再加皮弁 - 参与军政，保家卫国",
            "三加爵弁 - 参与祭祀，尊祖敬宗",
            "三加之意，由卑至尊，循序渐进",
        ]
    }

    /// 笄礼三加意义
    pub fn jili_meaning(&self) -> Vec<&'static str> {
        vec![
            "初加发笄 - 标志成年，端庄自持",
            "再加发簪 - 贤淑温婉，持家有道",
            "三加钗冠 - 德才兼备，相夫教子",
            "笄礼之意，养成德行，规范言行",
        ]
    }

    /// 成年礼象征意义
    pub fn symbolism(&self) -> Vec<&'static str> {
        vec![
            "加冠/笄 - 地位改变，责任加重",
            "赐字 - 新身份，新使命",
            "见母 - 感恩养育，传承美德",
            "见尊长 - 融入社会，承担责任",
            "三加礼 - 阶段成长，渐次完善",
        ]
    }

    /// 现代意义
    pub fn modern_significance(&self) -> Vec<&'static str> {
        vec![
            "增强成年意识和责任感",
            "传承中华优秀传统文化",
            "促进家庭和睦、社会和谐",
            "培养青年担当精神",
            "延续华夏礼仪文明",
        ]
    }
}

impl Rule for ChineseComingOfAgeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_coming_of_age")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国传统成年礼",
            &[
                ("冠礼流程", &self.guanli_procedure()),
                ("笄礼流程", &self.jili_procedure()),
                ("冠礼三加意义", &self.guanli_meaning()),
                ("笄礼三加意义", &self.jili_meaning()),
                ("象征意义", &self.symbolism()),
                ("现代意义", &self.modern_significance()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coming_of_age_rules() {
        let rules = ChineseComingOfAgeRules::new();
        assert_eq!(rules.metadata().name, "中国传统成年礼");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_guanli_procedure() {
        let rules = ChineseComingOfAgeRules::new();
        let procedure = rules.guanli_procedure();
        assert!(procedure.iter().any(|p| p.contains("二十岁")));
        assert!(procedure.iter().any(|p| p.contains("三加")));
        assert!(procedure.len() >= 10);
    }

    #[test]
    fn test_jili_procedure() {
        let rules = ChineseComingOfAgeRules::new();
        let procedure = rules.jili_procedure();
        assert!(procedure.iter().any(|p| p.contains("十五岁")));
        assert!(procedure.iter().any(|p| p.contains("笄")));
        assert!(procedure.len() >= 8);
    }

    #[test]
    fn test_symbolism() {
        let rules = ChineseComingOfAgeRules::new();
        let symbolism = rules.symbolism();
        assert!(symbolism.iter().any(|s| s.contains("责任")));
        assert!(symbolism.len() >= 4);
    }
}
