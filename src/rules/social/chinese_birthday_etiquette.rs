//! 中国传统寿礼礼仪
//!
//! 寿礼是为长辈庆祝生日的礼仪，体现孝道与敬老传统。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseBirthdayEtiquetteRules,
    name: "中国寿礼礼仪",
    desc: "传统祝寿礼仪规范",
    origin: "中国",
    tags: ["社交", "礼仪", "祝寿", "传统"]
}

impl ChineseBirthdayEtiquetteRules {
    /// 寿礼等级
    pub fn birthday_levels(&self) -> Vec<&'static str> {
        vec![
            "花甲寿 - 六十岁，干支纪年一甲子",
            "古稀寿 - 七十岁，'人生七十古来稀'",
            "耄耋寿 - 八十岁，'耄耋之年'",
            "期颐寿 - 百岁，'期颐之寿'",
            "花甲重开 - 一百二十岁",
            "古稀双庆 - 一百四十岁",
        ]
    }

    /// 寿礼筹备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前一月发请帖",
            "布置寿堂，张灯结彩",
            "正中悬挂'寿'字或寿星图",
            "准备寿桃、寿面、寿糕",
            "子女身着吉服",
            "安排祝寿仪式流程",
        ]
    }

    /// 祝寿仪式流程
    pub fn ceremony_procedure(&self) -> Vec<&'static str> {
        vec![
            "寿星正坐寿堂中央",
            "子女依次拜寿，行叩首礼",
            "孙辈拜寿，可简化为鞠躬",
            "亲友祝寿，献寿词、寿联",
            "敬献寿桃、寿面、寿糕",
            "祝寿宴席，合家欢聚",
            "观看戏曲、曲艺表演",
            "合影留念",
        ]
    }

    /// 寿礼禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "忌说'死'、'病'等不吉利字眼",
            "忌送钟表（谐音'送终'）",
            "忌送梨（谐音'离'）",
            "忌送伞（谐音'散'）",
            "忌送鞋（寓意'走人'）",
            "忌白色蜡烛",
            "忌送菊花（祭祀用花）",
            "男性忌送绿色帽子",
        ]
    }

    /// 传统寿礼礼品
    pub fn traditional_gifts(&self) -> Vec<&'static str> {
        vec![
            "寿桃 - 象征长寿",
            "寿面 - 寓意长寿面",
            "寿糕 - 祝福高寿",
            "寿字书法 - 吉祥祝福",
            "松鹤图 - 松鹤延年",
            "寿星像 - 福禄寿三星",
            "如意 - 万事如意",
            "红包 - 晚辈心意",
        ]
    }

    /// 祝寿词
    pub fn birthday_greetings(&self) -> Vec<&'static str> {
        vec![
            "福如东海，寿比南山",
            "日月昌明，松鹤长春",
            "笑口常开，天伦永享",
            "身体健康，长命百岁",
            "生日快乐，后福无疆",
            "岁岁平安，年年有余",
            "福寿安康，喜气盈门",
            "寿比天高，福比海深",
        ]
    }

    /// 家族责任
    pub fn family_duties(&self) -> Vec<&'static str> {
        vec![
            "长子主事，统筹安排",
            "子女分担费用",
            "孙辈献艺助兴",
            "媳妇操持后勤",
            "外嫁女儿携礼回门",
            "家族长辈主持仪式",
        ]
    }
}

impl Rule for ChineseBirthdayEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_birthday_etiquette")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国寿礼礼仪",
            &[
                ("寿礼等级", &self.birthday_levels()),
                ("寿礼筹备", &self.preparation()),
                ("祝寿仪式流程", &self.ceremony_procedure()),
                ("寿礼禁忌", &self.taboos()),
                ("传统寿礼礼品", &self.traditional_gifts()),
                ("祝寿词", &self.birthday_greetings()),
                ("家族责任", &self.family_duties()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birthday_etiquette_rules() {
        let rules = ChineseBirthdayEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "中国寿礼礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_birthday_levels() {
        let rules = ChineseBirthdayEtiquetteRules::new();
        let levels = rules.birthday_levels();
        assert!(levels.iter().any(|l| l.contains("花甲")));
        assert!(levels.iter().any(|l| l.contains("古稀")));
        assert!(levels.len() >= 5);
    }

    #[test]
    fn test_ceremony_procedure() {
        let rules = ChineseBirthdayEtiquetteRules::new();
        let procedure = rules.ceremony_procedure();
        assert!(procedure.iter().any(|p| p.contains("寿星")));
        assert!(procedure.len() >= 6);
    }

    #[test]
    fn test_taboos() {
        let rules = ChineseBirthdayEtiquetteRules::new();
        let taboos = rules.taboos();
        assert!(taboos.iter().any(|t| t.contains("钟")));
        assert!(taboos.len() >= 6);
    }

    #[test]
    fn test_traditional_gifts() {
        let rules = ChineseBirthdayEtiquetteRules::new();
        let gifts = rules.traditional_gifts();
        assert!(gifts.iter().any(|g| g.contains("寿桃")));
        assert!(gifts.len() >= 6);
    }
}
