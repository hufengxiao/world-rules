//! 中国传统祭祀礼仪
//!
//! 祭祀是中国传统文化中最重要的礼仪活动之一，
//! 用于表达对天地、祖先、神灵的崇敬与感恩。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseRitualSacrificeRules,
    name: "中国祭祀礼仪",
    desc: "传统祭祀仪式规范",
    origin: "中国",
    tags: ["社交", "礼仪", "祭祀", "传统"]
}

impl ChineseRitualSacrificeRules {
    /// 祭祀对象
    pub fn sacrifice_objects(&self) -> Vec<&'static str> {
        vec![
            "天神祭祀 - 祭天、祭日、祭月",
            "地祇祭祀 - 祭地、祭山川、祭社稷",
            "祖先祭祀 - 祭祖、祠堂祭祀",
            "圣贤祭祀 - 祭孔、祭关羽",
            "行业祖师祭祀 - 各行业祭祖师",
            "家神祭祀 - 灶神、门神、财神",
        ]
    }

    /// 祭品类型
    pub fn offering_types(&self) -> Vec<&'static str> {
        vec![
            "太牢 - 牛羊猪三牲（最高规格）",
            "少牢 - 羊猪二牲（次高规格）",
            "特牲 - 单一牲畜",
            "五谷 - 稻、黍、稷、麦、菽",
            "果蔬 - 时令水果、蔬菜",
            "酒醴 - 清酒、甜酒",
            "币帛 - 丝绸、布帛",
            "香烛 - 线香、蜡烛",
        ]
    }

    /// 祭祀程序
    pub fn sacrifice_procedure(&self) -> Vec<&'static str> {
        vec![
            "沐浴斋戒 - 祭前三日斋戒",
            "设坛陈器 - 布置祭坛、祭器",
            "迎神降神 - 恭迎神灵降临",
            "进馔进酒 - 献祭品、奠酒",
            "读祝文 - 宣读祝祷之文",
            "焚香祭拜 - 上香、行礼",
            "送神望燎 - 送神、焚烧祭品",
            "分胙受福 - 分享祭肉、祈福",
        ]
    }

    /// 祭祖礼仪
    pub fn ancestor_worship(&self) -> Vec<&'static str> {
        vec![
            "设神位、供桌、香炉",
            "摆放供品：三牲、水果、糕点",
            "点烛、上香（三柱）",
            "献酒三杯，依次洒于地",
            "读祭文，表达哀思与感恩",
            "行三跪九叩大礼",
            "焚烧纸钱、元宝",
            "全家依次祭拜",
            "撤供、分胙",
        ]
    }

    /// 清明祭扫
    pub fn qingming_sacrifice(&self) -> Vec<&'static str> {
        vec![
            "修整坟墓、清除杂草",
            "培土加固，整理墓碑",
            "摆放供品：鲜花、水果、糕点",
            "上香、点烛",
            "祭拜、叩首",
            "焚烧纸钱",
            "撒酒祭奠",
            "追思先人功德",
        ]
    }

    /// 祭祀禁忌
    pub fn sacrifice_taboos(&self) -> Vec<&'static str> {
        vec![
            "祭祀前需沐浴斋戒",
            "不可用不洁之物祭祀",
            "祭祀时不可嬉笑喧哗",
            "女子经期不宜参与祭祀",
            "孕妇不宜参与祭祀",
            "祭祀不可穿红戴绿",
            "祭品不可事先品尝",
            "祭祀期间不可说不吉利的话",
        ]
    }

    /// 祭祀祝文格式
    pub fn prayer_format(&self) -> Vec<&'static str> {
        vec![
            "开头：维某年某月某日",
            "主祭者：孝孙/孝子某某",
            "祭拜对象：致祭于显祖/先考",
            "正文：叙述先人功德、表达哀思",
            "结尾：尚飨（敬请享用）",
            "署名：子孙某某敬以此文",
        ]
    }

    /// 现代简化祭祀
    pub fn modern_sacrifice(&self) -> Vec<&'static str> {
        vec![
            "鲜花祭祀 - 用鲜花代替纸钱",
            "植树祭祀 - 植树纪念先人",
            "网上祭祀 - 网上祭扫平台",
            "家庭追思会 - 家庭内部追思",
            "代客祭祀 - 委托机构代祭",
            "集体公祭 - 政府组织的公祭活动",
        ]
    }
}

impl Rule for ChineseRitualSacrificeRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_ritual_sacrifice")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国祭祀礼仪",
            &[
                ("祭祀对象", &self.sacrifice_objects()),
                ("祭品类型", &self.offering_types()),
                ("祭祀程序", &self.sacrifice_procedure()),
                ("祭祖礼仪", &self.ancestor_worship()),
                ("清明祭扫", &self.qingming_sacrifice()),
                ("祭祀禁忌", &self.sacrifice_taboos()),
                ("祭祀祝文格式", &self.prayer_format()),
                ("现代简化祭祀", &self.modern_sacrifice()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ritual_sacrifice_rules() {
        let rules = ChineseRitualSacrificeRules::new();
        assert_eq!(rules.metadata().name, "中国祭祀礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_sacrifice_objects() {
        let rules = ChineseRitualSacrificeRules::new();
        let objects = rules.sacrifice_objects();
        assert!(objects.iter().any(|o| o.contains("天神")));
        assert!(objects.iter().any(|o| o.contains("祖先")));
        assert!(objects.len() >= 5);
    }

    #[test]
    fn test_offering_types() {
        let rules = ChineseRitualSacrificeRules::new();
        let offerings = rules.offering_types();
        assert!(offerings.iter().any(|o| o.contains("太牢")));
        assert!(offerings.iter().any(|o| o.contains("五谷")));
        assert!(offerings.len() >= 6);
    }

    #[test]
    fn test_sacrifice_procedure() {
        let rules = ChineseRitualSacrificeRules::new();
        let procedure = rules.sacrifice_procedure();
        assert!(procedure.iter().any(|p| p.contains("斋戒")));
        assert!(procedure.iter().any(|p| p.contains("上香")));
        assert!(procedure.len() >= 6);
    }
}