//! 中国传统礼仪基础 - 三礼五常
//!
//! 中华礼仪文化的核心思想，包括三纲五常、四维八德等基本伦理规范。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ChineseEtiquetteBasicsRules,
    name: "中国传统礼仪基础",
    desc: "三纲五常、四维八德等传统伦理规范",
    origin: "中国",
    tags: ["社交", "礼仪", "传统", "文化"]
}

impl ChineseEtiquetteBasicsRules {
    /// 五常
    pub fn five_constants(&self) -> Vec<&'static str> {
        vec![
            "仁 - 仁爱之心，恻隐为怀",
            "义 - 正义公正，见义勇为",
            "礼 - 恭敬谦让，礼尚往来",
            "智 - 明辨是非，通达事理",
            "信 - 诚实守信，言出必行",
        ]
    }

    /// 三纲
    pub fn three_guidelines(&self) -> Vec<&'static str> {
        vec![
            "君为臣纲 - 臣下服从君主",
            "父为子纲 - 子女服从父母",
            "夫为妻纲 - 妻子服从丈夫",
            "注：现代应理解为上下级、亲子、夫妻间的责任关系",
        ]
    }

    /// 四维
    pub fn four_pillars(&self) -> Vec<&'static str> {
        vec![
            "礼 - 尊卑有序，进退有度",
            "义 - 合乎道义，公正无私",
            "廉 - 廉洁奉公，不贪污腐败",
            "耻 - 知耻而后勇，荣辱分明",
        ]
    }

    /// 八德
    pub fn eight_virtues(&self) -> Vec<&'static str> {
        vec![
            "孝 - 孝顺父母，赡养老人",
            "悌 - 兄弟友爱，手足情深",
            "忠 - 忠诚报国，忠于职守",
            "信 - 诚实守信，言必信行必果",
            "礼 - 恭敬谦让，彬彬有礼",
            "义 - 见义勇为，义不容辞",
            "廉 - 廉洁自律，清白做人",
            "耻 - 知耻明理，自省自律",
        ]
    }

    /// 五伦
    pub fn five_relations(&self) -> Vec<&'static str> {
        vec![
            "父子有亲 - 父慈子孝",
            "君臣有义 - 君仁臣忠",
            "夫妇有别 - 夫唱妇随",
            "长幼有序 - 兄友弟恭",
            "朋友有信 - 一诺千金",
        ]
    }

    /// 六艺
    pub fn six_arts(&self) -> Vec<&'static str> {
        vec![
            "礼 - 礼仪规范",
            "乐 - 音乐舞蹈",
            "射 - 射箭技艺",
            "御 - 驾车技术",
            "书 - 书法文字",
            "数 - 数学计算",
        ]
    }

    /// 基本礼仪
    pub fn basic_etiquette(&self) -> Vec<&'static str> {
        vec![
            "站如松 - 站姿挺拔，精神饱满",
            "坐如钟 - 坐姿端正，稳重端庄",
            "行如风 - 行走轻快，步伐稳健",
            "卧如弓 - 睡姿舒展，安眠养神",
            "言有度 - 言语有度，不妄言",
            "行有礼 - 行为有礼，不鲁莽",
        ]
    }

    /// 现代诠释
    pub fn modern_interpretation(&self) -> Vec<&'static str> {
        vec![
            "五常是做人的基本准则",
            "八德是处世的道德规范",
            "取其精华，去其糟粕",
            "与社会主义核心价值观相结合",
            "传承中华优秀传统文化",
            "建立现代文明礼仪体系",
        ]
    }
}

impl Rule for ChineseEtiquetteBasicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("chinese_etiquette_basics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "中国传统礼仪基础",
            &[
                ("五常", &self.five_constants()),
                ("三纲", &self.three_guidelines()),
                ("四维", &self.four_pillars()),
                ("八德", &self.eight_virtues()),
                ("五伦", &self.five_relations()),
                ("六艺", &self.six_arts()),
                ("基本礼仪", &self.basic_etiquette()),
                ("现代诠释", &self.modern_interpretation()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_etiquette_basics_rules() {
        let rules = ChineseEtiquetteBasicsRules::new();
        assert_eq!(rules.metadata().name, "中国传统礼仪基础");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_five_constants() {
        let rules = ChineseEtiquetteBasicsRules::new();
        let constants = rules.five_constants();
        assert!(constants.iter().any(|c| c.contains("仁")));
        assert!(constants.iter().any(|c| c.contains("义")));
        assert!(constants.iter().any(|c| c.contains("礼")));
        assert_eq!(constants.len(), 5);
    }

    #[test]
    fn test_eight_virtues() {
        let rules = ChineseEtiquetteBasicsRules::new();
        let virtues = rules.eight_virtues();
        assert!(virtues.iter().any(|v| v.contains("孝")));
        assert!(virtues.iter().any(|v| v.contains("悌")));
        assert_eq!(virtues.len(), 8);
    }

    #[test]
    fn test_six_arts() {
        let rules = ChineseEtiquetteBasicsRules::new();
        let arts = rules.six_arts();
        assert!(arts.iter().any(|a| a.contains("礼")));
        assert!(arts.iter().any(|a| a.contains("乐")));
        assert_eq!(arts.len(), 6);
    }
}