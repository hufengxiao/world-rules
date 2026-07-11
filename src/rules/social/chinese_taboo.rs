//! 中国传统禁忌礼仪
//!
//! 中国传统禁忌是社会行为规范的重要组成部分，
//! 涉及日常生活的方方面面。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseTabooRules,
    name: "中国传统禁忌",
    desc: "传统社会行为禁忌规范",
    origin: "中国",
    tags: ["社交", "礼仪", "禁忌", "传统"]
}

impl ChineseTabooRules {
    /// 语言禁忌
    pub fn language_taboos(&self) -> Vec<&'static str> {
        vec![
            "过年忌说'死'、'病'、'穷'、'光'等字",
            "船上忌说'翻'、'沉'、'漏'等字",
            "商家忌说'关门'、'亏本'",
            "婚嫁忌说'离'、'散'、'断'",
            "对长辈直呼其名为不敬",
            "名字避讳 - 避祖先、尊者之名",
            "不吉利的谐音要避免",
            "不说'四'（谐音'死'），尤其送礼",
        ]
    }

    /// 饮食禁忌
    pub fn dietary_taboos(&self) -> Vec<&'static str> {
        vec![
            "筷子不可竖插饭中（形似祭亡人）",
            "不可用筷子敲碗（乞丐之象）",
            "饭碗不可留剩饭（浪费粮食）",
            "倒茶不可满杯（'茶七酒满'）",
            "鱼不可翻面（渔民忌讳）",
            "梨不可分食（谐音'分离'）",
            "药渣不可倒在门口",
            "正月不剃头（'剃头死舅舅'）",
        ]
    }

    /// 行为禁忌
    pub fn behavioral_taboos(&self) -> Vec<&'static str> {
        vec![
            "不可用手指指月亮（耳朵会缺）",
            "夜间不可吹口哨（招鬼）",
            "小孩不可玩火（夜间尿床）",
            "不可踩门槛（得罪门神）",
            "不可对着人打喷嚏",
            "不可用红笔写人名（死刑犯）",
            "晾晒的衣裤不可从下穿过",
            "不可坐在枕头上",
        ]
    }

    /// 礼仪禁忌
    pub fn etiquette_taboos(&self) -> Vec<&'static str> {
        vec![
            "送礼忌送钟（谐音'送终'）",
            "送礼忌送伞（谐音'散'）",
            "送礼忌送鞋（寓意'走人'）",
            "送礼忌送绿帽（寓妻不贞）",
            "探病忌在下午或晚上",
            "戴孝者不进他人家门",
            "孕妇不参加婚礼丧礼",
            "筷子掉落要说'筷落'（快乐）",
        ]
    }

    /// 节日禁忌
    pub fn festival_taboos(&self) -> Vec<&'static str> {
        vec![
            "春节初一不扫地倒垃圾（扫走财气）",
            "春节初一不说不吉利的话",
            "春节初二回娘家带礼物成双",
            "正月不理发（'死舅舅'）",
            "清明节不穿红戴绿",
            "端午节不祝'快乐'只祝'安康'",
            "中元节夜间不出门",
            "中秋月饼要分食不独享",
        ]
    }

    /// 数字禁忌
    pub fn number_taboos(&self) -> Vec<&'static str> {
        vec![
            "四（谐音'死'）为不吉",
            "七十三、八十四为'坎年'",
            "送礼成双不成单",
            "本命年穿红避凶",
            "六、八为吉利数字",
            "九（谐音'久'）为吉利",
            "七为凶数（头七、做七）",
            "十八为吉利（'要发'）",
        ]
    }

    /// 婚丧禁忌
    pub fn wedding_funeral_taboos(&self) -> Vec<&'static str> {
        vec![
            "婚车不可走回头路",
            "新娘出门不可踩门槛",
            "丧礼不可穿红戴绿",
            "戴孝期间不参加喜事",
            "孕妇不送葬",
            "本命年不宜结婚",
            "相冲生肖不宜做伴郎伴娘",
            "头七之前家属不外出",
        ]
    }

    /// 居家禁忌
    pub fn household_taboos(&self) -> Vec<&'static str> {
        vec![
            "镜子不对床",
            "床头不对门",
            "梁下不设床",
            "大门不对电梯",
            "厕所不居中宫",
            "厨房门不对卧室门",
            "卧室内不设鱼缸",
            "床头不挂巨画",
        ]
    }
}

impl Rule for ChineseTabooRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_taboo")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国传统禁忌",
            &[
                ("语言禁忌", &self.language_taboos()),
                ("饮食禁忌", &self.dietary_taboos()),
                ("行为禁忌", &self.behavioral_taboos()),
                ("礼仪禁忌", &self.etiquette_taboos()),
                ("节日禁忌", &self.festival_taboos()),
                ("数字禁忌", &self.number_taboos()),
                ("婚丧禁忌", &self.wedding_funeral_taboos()),
                ("居家禁忌", &self.household_taboos()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taboo_rules() {
        let rules = ChineseTabooRules::new();
        assert_eq!(rules.metadata().name, "中国传统禁忌");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_language_taboos() {
        let rules = ChineseTabooRules::new();
        let taboos = rules.language_taboos();
        assert!(taboos.iter().any(|t| t.contains("过年")));
        assert!(taboos.len() >= 6);
    }

    #[test]
    fn test_dietary_taboos() {
        let rules = ChineseTabooRules::new();
        let taboos = rules.dietary_taboos();
        assert!(taboos.iter().any(|t| t.contains("筷子")));
        assert!(taboos.len() >= 6);
    }

    #[test]
    fn test_festival_taboos() {
        let rules = ChineseTabooRules::new();
        let taboos = rules.festival_taboos();
        assert!(taboos.iter().any(|t| t.contains("春节")));
        assert!(taboos.len() >= 6);
    }

    #[test]
    fn test_all_sections() {
        let rules = ChineseTabooRules::new();
        assert!(rules.language_taboos().len() >= 6);
        assert!(rules.dietary_taboos().len() >= 6);
        assert!(rules.behavioral_taboos().len() >= 6);
        assert!(rules.etiquette_taboos().len() >= 6);
    }
}
