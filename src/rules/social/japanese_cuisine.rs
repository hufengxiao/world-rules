//! 日式料理礼仪
//!
//! 涵盖日本料理的礼仪规范，包括寿司、拉面、怀石料理等用餐礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: JapaneseCuisineRules,
    name: "日式料理礼仪",
    desc: "日本料理用餐礼仪规范，包括寿司、拉面、怀石料理等",
    origin: "日本",
    tags: ["社交", "礼仪", "餐饮", "日料"]
}

impl JapaneseCuisineRules {
    /// 用餐基本礼仪
    pub fn basic_manners(&self) -> Vec<&'static str> {
        vec![
            "餐前说「いただきます」（我开动了）",
            "餐后说「ごちそうさまでした」（谢谢款待）",
            "双手合十表示感谢",
            "等待主人说「どうぞ」后开始用餐",
            "不要在餐桌上整理发型或补妆",
            "手机放在包里，不要放在桌上",
            "用餐时保持正确坐姿",
            "感谢厨师和服务人员",
        ]
    }

    /// 筷子礼仪
    pub fn chopstick_etiquette(&self) -> Vec<&'static str> {
        vec![
            "不要用筷子传递食物（渡し箸）",
            "不要用筷子指人或物（指し箸）",
            "不要舔筷子（ねぶり箸）",
            "不要用筷子插食物（刺し箸）",
            "不要在菜肴中翻找（探り箸）",
            "不要用筷子拉盘子（寄せ箸）",
            "不要拿着筷子犹豫不决（迷い箸）",
            "筷子不用时放在筷子架上",
        ]
    }

    /// 寿司礼仪
    pub fn sushi_etiquette(&self) -> Vec<&'static str> {
        vec![
            "用手或筷子都可以吃寿司",
            "鱼生面蘸酱油，不要蘸米饭",
            "一口吃完，不要分两半",
            "不要将芥末混入酱油中",
            "姜片用于清口，不是主菜",
            "按味道从轻到重的顺序享用",
            "可以用手吃寿司卷",
            "吃完后清空盘子表示满意",
        ]
    }

    /// 拉面礼仪
    pub fn ramen_etiquette(&self) -> Vec<&'static str> {
        vec![
            "吃面时发出吸溜声是正常的",
            "用筷子夹起面条，低头靠近碗",
            "不要用勺子吃面条",
            "喝汤可以用勺子或直接端碗",
            "配菜和面条一起吃",
            "吃完面条后可以端碗喝汤",
            "辣味拉面可以点额外的辣",
            "不要将面条咬断放回碗中",
        ]
    }

    /// 怀石料理礼仪
    pub fn kaiseki_etiquette(&self) -> Vec<&'static str> {
        vec![
            "欣赏每道菜的摆盘艺术",
            "按照上菜顺序享用",
            "小份食物一次吃完",
            "不要留下残羹剩饭",
            "碗盖取下后放在桌上",
            "汤碗可以端起来喝",
            "感谢厨师的精心准备",
            "体会季节感和意境",
        ]
    }

    /// 刺身礼仪
    pub fn sashimi_etiquette(&self) -> Vec<&'static str> {
        vec![
            "芥末放在刺身上，不是酱油中",
            "蘸酱油要轻，不要浸透",
            "一次取适量食用",
            "配萝卜丝和紫苏叶",
            "生姜用于清口",
            "按味道从淡到浓的顺序",
            "不要一次切太多",
            "体会刺身的鲜味",
        ]
    }

    /// 天妇罗礼仪
    pub fn tempura_etiquette(&self) -> Vec<&'static str> {
        vec![
            "趁热食用，不要久放",
            "蘸天妇罗汁或盐",
            "蘸汁时轻蘸即可",
            "一次吃一块",
            "可以挤柠檬汁提味",
            "按味道从淡到浓的顺序",
            "不要让天妇罗变软",
            "炸制食物要趁热享用",
        ]
    }

    /// 居酒屋礼仪
    pub fn izakaya_etiquette(&self) -> Vec<&'static str> {
        vec![
            "入座后先点饮料",
            "等待所有人饮料到齐再干杯",
            "「干杯」时碰杯并说「干杯」",
            "自己斟酒时要先为他人斟酒",
            "别人斟酒时要双手持杯",
            "不要让他人杯子空着",
            "可以共享小菜",
            "最后点餐要适量",
        ]
    }

    /// 茶道礼仪
    pub fn tea_ceremony(&self) -> Vec<&'static str> {
        vec![
            "穿着得体，避免强烈香水",
            "进入茶室前洗手漱口",
            "按指定位置就座",
            "欣赏茶具和挂轴",
            "主人奉茶时行礼致谢",
            "茶碗要转动后饮用",
            "喝完后擦拭茶碗边缘",
            "感谢主人的款待",
        ]
    }

    /// 饮食禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要把筷子插在米饭上",
            "不要将食物残渣放回盘中",
            "不要用手抓取食物（寿司除外）",
            "不要在餐桌上谈论不适话题",
            "不要边走边吃",
            "不要大声咀嚼",
            "不要将碗端到嘴边用筷子扒饭",
            "不要浪费食物",
        ]
    }
}

impl Rule for JapaneseCuisineRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining")
    }

    fn explain(&self) -> String {
        format!(
            "【日式料理礼仪】\n\n\
            用餐基本礼仪：\n{}\n\n\
            筷子礼仪：\n{}\n\n\
            寿司礼仪：\n{}\n\n\
            拉面礼仪：\n{}\n\n\
            怀石料理礼仪：\n{}\n\n\
            刺身礼仪：\n{}\n\n\
            天妇罗礼仪：\n{}\n\n\
            居酒屋礼仪：\n{}\n\n\
            茶道礼仪：\n{}\n\n\
            饮食禁忌：\n{}",
            self.basic_manners()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.chopstick_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.sushi_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.ramen_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.kaiseki_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.sashimi_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tempura_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.izakaya_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tea_ceremony()
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
    fn test_japanese_cuisine_rules() {
        let rules = JapaneseCuisineRules::new();
        assert_eq!(rules.metadata().name, "日式料理礼仪");
        assert!(!rules.basic_manners().is_empty());
        assert!(!rules.chopstick_etiquette().is_empty());
        assert!(!rules.sushi_etiquette().is_empty());
        assert!(!rules.ramen_etiquette().is_empty());
        assert!(!rules.kaiseki_etiquette().is_empty());
        assert!(!rules.sashimi_etiquette().is_empty());
        assert!(!rules.tempura_etiquette().is_empty());
        assert!(!rules.izakaya_etiquette().is_empty());
        assert!(!rules.tea_ceremony().is_empty());
        assert!(!rules.taboos().is_empty());
    }

    #[test]
    fn test_japanese_cuisine_validation() {
        let rules = JapaneseCuisineRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("dining"));
    }

    #[test]
    fn test_japanese_cuisine_explain() {
        let rules = JapaneseCuisineRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("用餐基本礼仪"));
        assert!(explanation.contains("筷子礼仪"));
        assert!(explanation.contains("寿司礼仪"));
        assert!(explanation.contains("饮食禁忌"));
    }
}
