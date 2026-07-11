//! 韩餐礼仪
//!
//! 涵盖韩国料理的礼仪规范，包括泡菜、烤肉、石锅拌饭等用餐礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: KoreanCuisineRules,
    name: "韩餐礼仪",
    desc: "韩国料理用餐礼仪规范，包括泡菜、烤肉、石锅拌饭等",
    origin: "韩国",
    tags: ["社交", "礼仪", "餐饮", "韩餐"]
}

impl KoreanCuisineRules {
    /// 用餐基本礼仪
    pub fn basic_manners(&self) -> Vec<&'static str> {
        vec![
            "长辈先开始用餐",
            "双手持碗表示尊敬",
            "不要把碗端到嘴边",
            "不要用筷子指人",
            "用餐时不要大声说话",
            "等长辈放下餐具后再放下",
            "餐后感谢主人款待",
            "不要浪费食物",
        ]
    }

    /// 餐具使用礼仪
    pub fn utensil_etiquette(&self) -> Vec<&'static str> {
        vec![
            "勺子用于吃饭和喝汤",
            "筷子用于夹菜",
            "不要同时使用勺子和筷子",
            "餐具不用时放在桌上",
            "不要敲击餐具",
            "勺子不要放在饭碗里",
            "金属餐具要轻拿轻放",
            "用完后餐具整齐摆放",
        ]
    }

    /// 小菜礼仪
    pub fn banchan_etiquette(&self) -> Vec<&'static str> {
        vec![
            "小菜可以共享",
            "用公筷夹取小菜",
            "一次取适量",
            "不要翻动小菜",
            "泡菜是主食配菜",
            "可以添加小菜",
            "不同小菜按顺序品尝",
            "吃完后尽量清空盘子",
        ]
    }

    /// 烤肉礼仪
    pub fn bbq_etiquette(&self) -> Vec<&'static str> {
        vec![
            "等待肉烤熟再吃",
            "用剪刀剪肉时注意安全",
            "用生菜包肉和配菜",
            "一次包一口大小",
            "可以添加大蒜和泡菜",
            "蘸酱适量",
            "不要浪费烤好的肉",
            "与同桌分享烤肉",
        ]
    }

    /// 石锅拌饭礼仪
    pub fn bibimbap_etiquette(&self) -> Vec<&'static str> {
        vec![
            "石锅很烫，小心触碰",
            "将酱料拌匀",
            "喜欢锅巴可以让饭多放一会",
            "勺子和筷子配合使用",
            "可以添加额外的酱",
            "拌匀后开始享用",
            "注意石锅温度",
            "不要急于倒入冷水",
        ]
    }

    /// 汤品礼仪
    pub fn soup_etiquette(&self) -> Vec<&'static str> {
        vec![
            "用勺子喝汤",
            "汤可以就着饭吃",
            "不要吹汤降温",
            "可以添汤",
            "汤里的料用筷子夹取",
            "汤碗不要端起来",
            "喝汤不要出声",
            "喝完表示满意",
        ]
    }

    /// 饮酒礼仪
    pub fn drinking_etiquette(&self) -> Vec<&'static str> {
        vec![
            "长辈倒酒时要双手持杯",
            "晚辈倒酒时要双手倒",
            "喝酒时侧身避开长辈目光",
            "不要给自己倒酒",
            "碰杯时杯沿低于长辈",
            "不要拒绝长辈敬酒",
            "保持适度饮酒",
            "空杯时主动为他人斟酒",
        ]
    }

    /// 年糕礼仪
    pub fn tteok_etiquette(&self) -> Vec<&'static str> {
        vec![
            "年糕粘糯，小口食用",
            "不要一次吃太多",
            "搭配酱料食用",
            "分享年糕菜肴",
            "注意年糕温度",
            "不要用手抓年糕",
            "用筷子夹取",
            "细嚼慢咽",
        ]
    }

    /// 韩定食礼仪
    pub fn hanjeongsik_etiquette(&self) -> Vec<&'static str> {
        vec![
            "按照上菜顺序享用",
            "小菜可以自由取用",
            "主食和配菜搭配",
            "体会传统韩餐的丰富",
            "不要浪费食物",
            "感谢主人的准备",
            "用餐时保持优雅",
            "按需添加小菜",
        ]
    }

    /// 用餐禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要把筷子插在饭里",
            "不要翻动菜肴",
            "不要在长辈面前失礼",
            "不要在餐桌上整理仪容",
            "不要边走边吃",
            "不要大声打嗝",
            "不要剩饭",
            "不要用筷子指人",
        ]
    }
}

impl Rule for KoreanCuisineRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining")
    }

    fn explain(&self) -> String {
        format!(
            "【韩餐礼仪】\n\n\
            用餐基本礼仪：\n{}\n\n\
            餐具使用礼仪：\n{}\n\n\
            小菜礼仪：\n{}\n\n\
            烤肉礼仪：\n{}\n\n\
            石锅拌饭礼仪：\n{}\n\n\
            汤品礼仪：\n{}\n\n\
            饮酒礼仪：\n{}\n\n\
            年糕礼仪：\n{}\n\n\
            韩定食礼仪：\n{}\n\n\
            用餐禁忌：\n{}",
            self.basic_manners()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.utensil_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.banchan_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bibimbap_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bibimbap_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.soup_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.drinking_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tteok_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hanjeongsik_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_korean_cuisine_rules() {
        let rules = KoreanCuisineRules::new();
        assert_eq!(rules.metadata().name, "韩餐礼仪");
        assert!(!rules.basic_manners().is_empty());
        assert!(!rules.utensil_etiquette().is_empty());
        assert!(!rules.banchan_etiquette().is_empty());
        assert!(!rules.bibimbap_etiquette().is_empty());
        assert!(!rules.soup_etiquette().is_empty());
        assert!(!rules.drinking_etiquette().is_empty());
        assert!(!rules.tteok_etiquette().is_empty());
        assert!(!rules.hanjeongsik_etiquette().is_empty());
        assert!(!rules.taboos().is_empty());
    }

    #[test]
    fn test_korean_cuisine_validation() {
        let rules = KoreanCuisineRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("dining"));
    }

    #[test]
    fn test_korean_cuisine_explain() {
        let rules = KoreanCuisineRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("用餐基本礼仪"));
        assert!(explanation.contains("餐具使用礼仪"));
        assert!(explanation.contains("烤肉礼仪"));
        assert!(explanation.contains("用餐禁忌"));
    }
}