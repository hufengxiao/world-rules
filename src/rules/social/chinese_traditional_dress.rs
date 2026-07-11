//! 中国传统服饰礼仪
//!
//! 中国传统服饰礼仪涵盖衣着规范、服饰等级、穿戴礼节等内容。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseTraditionalDressRules,
    name: "中国传统服饰礼仪",
    desc: "传统服饰穿着规范与礼节",
    origin: "中国",
    tags: ["社交", "礼仪", "服饰", "传统"]
}

impl ChineseTraditionalDressRules {
    /// 传统服饰类型
    pub fn dress_types(&self) -> Vec<&'static str> {
        vec![
            "深衣 - 上下连体，衣长至足",
            "袍服 - 长袍，男女通用",
            "襦裙 - 上衣下裙，女子的常服",
            "曲裾 - 曲线裁剪的深衣",
            "直裾 - 直线裁剪的深衣",
            "圆领袍 - 圆领窄袖长袍",
            "褙子 - 对襟长衫，宋代流行",
            "马面裙 - 明代女裙，两侧打褶",
        ]
    }

    /// 服饰等级
    pub fn dress_hierarchy(&self) -> Vec<&'static str> {
        vec![
            "帝王 - 龙袍、冕服",
            "官员 - 朝服、公服、常服",
            "命妇 - 凤冠霞帔、诰命服",
            "士人 - 章服、儒服",
            "庶民 - 布衣、短褐",
            "僧道 - 袈裟、道袍",
            "颜色等级：黄 > 紫 > 朱 > 绿 > 青 > 蓝 > 黑",
        ]
    }

    /// 场合着装
    pub fn occasion_dress(&self) -> Vec<&'static str> {
        vec![
            "朝会 - 朝服、官服",
            "祭祀 - 祭服、礼服",
            "婚礼 - 凤冠霞帔、大红喜服",
            "丧礼 - 斩衰、齐衰、功服",
            "日常 - 常服、便服",
            "居家 - 中衣、寝衣",
            "出行 - 行服、骑装",
        ]
    }

    /// 配饰礼仪
    pub fn accessories_etiquette(&self) -> Vec<&'static str> {
        vec![
            "冠帽 - 男子成年加冠",
            "发簪 - 女子挽发、笄礼",
            "玉佩 - 君子无故玉不去身",
            "香囊 - 随身佩戴、驱邪避秽",
            "腰带 - 身份象征，带钩精美",
            "荷包 - 随身小袋，装零物",
            "扇子 - 文人雅士必备",
            "手帕 - 男女皆用，礼仪所需",
        ]
    }

    /// 穿戴规范
    pub fn wearing_rules(&self) -> Vec<&'static str> {
        vec![
            "衣不露体 - 穿着端庄，不过分暴露",
            "冠不正不立 - 帽子戴正才能站立",
            "衣整带束 - 衣服整齐，腰带束紧",
            "袜履整齐 - 鞋袜整洁，不可赤足",
            "内衣不外露 - 内衣不可外露",
            "颜色协调 - 上下衣颜色搭配和谐",
            "季节适宜 - 穿着符合季节特点",
        ]
    }

    /// 禁忌规范
    pub fn dress_taboos(&self) -> Vec<&'static str> {
        vec![
            "不可穿奇装异服",
            "不可衣冠不整见客",
            "不可赤膊赤足待客",
            "不可穿丧服入喜庆场合",
            "不可穿婚服入丧礼",
            "不可穿着过于华丽赴丧礼",
            "不可反穿衣（不祥之兆）",
            "不可穿着脏污见长辈",
        ]
    }

    /// 颜色象征
    pub fn color_symbolism(&self) -> Vec<&'static str> {
        vec![
            "黄色 - 皇家专用，尊贵象征",
            "红色 - 喜庆吉祥，婚嫁首选",
            "白色 - 丧葬之色，纯洁哀悼",
            "黑色 - 庄重肃穆，官员朝服",
            "紫色 - 高贵典雅，三品以上",
            "绿色 - 生机盎然，普通百姓",
            "蓝色 - 沉稳内敛，文人雅士",
            "五色对应五行：青赤黄白黑",
        ]
    }

    /// 现代传承
    pub fn modern_inheritance(&self) -> Vec<&'static str> {
        vec![
            "汉服复兴 - 传承传统服饰文化",
            "旗袍传承 - 展现东方女性之美",
            "唐装流行 - 现代改良传统服饰",
            "传统婚礼 - 穿汉服/秀禾服结婚",
            "节日着装 - 春节穿传统服饰",
            "文化展示 - 传统文化活动中展示",
        ]
    }
}

impl Rule for ChineseTraditionalDressRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_traditional_dress")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国传统服饰礼仪",
            &[
                ("传统服饰类型", &self.dress_types()),
                ("服饰等级", &self.dress_hierarchy()),
                ("场合着装", &self.occasion_dress()),
                ("配饰礼仪", &self.accessories_etiquette()),
                ("穿戴规范", &self.wearing_rules()),
                ("禁忌规范", &self.dress_taboos()),
                ("颜色象征", &self.color_symbolism()),
                ("现代传承", &self.modern_inheritance()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traditional_dress_rules() {
        let rules = ChineseTraditionalDressRules::new();
        assert_eq!(rules.metadata().name, "中国传统服饰礼仪");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_dress_types() {
        let rules = ChineseTraditionalDressRules::new();
        let types = rules.dress_types();
        assert!(types.iter().any(|t| t.contains("深衣")));
        assert!(types.iter().any(|t| t.contains("袍服")));
        assert!(types.len() >= 6);
    }

    #[test]
    fn test_dress_hierarchy() {
        let rules = ChineseTraditionalDressRules::new();
        let hierarchy = rules.dress_hierarchy();
        assert!(hierarchy.iter().any(|h| h.contains("帝王")));
        assert!(hierarchy.iter().any(|h| h.contains("颜色")));
        assert!(hierarchy.len() >= 5);
    }

    #[test]
    fn test_color_symbolism() {
        let rules = ChineseTraditionalDressRules::new();
        let colors = rules.color_symbolism();
        assert!(colors.iter().any(|c| c.contains("黄色")));
        assert!(colors.iter().any(|c| c.contains("红色")));
        assert!(colors.len() >= 6);
    }
}
