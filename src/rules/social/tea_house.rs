//! 茶馆礼仪
//!
//! 涵盖茶馆的礼仪规范，包括入座、品茶、交流、赏艺等礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TeaHouseRules,
    name: "茶馆礼仪",
    desc: "茶馆礼仪规范，包括入座、品茶、交流、赏艺等",
    origin: "中国",
    tags: ["社交", "礼仪", "餐饮", "茶"]
}

impl TeaHouseRules {
    /// 入座礼仪
    pub fn seating(&self) -> Vec<&'static str> {
        vec![
            "选择合适的茶座",
            "尊重传统座次安排",
            "不要占用过多空间",
            "保持座位整洁",
            "等待引导入座",
            "不要随意换座位",
            "座次讲究礼仪",
            "入座后保持安静",
        ]
    }

    /// 品茶礼仪
    pub fn tea_tasting(&self) -> Vec<&'static str> {
        vec![
            "观茶色、闻茶香、品茶味",
            "小口品味，体会茶韵",
            "不要一口喝完",
            "可以多次冲泡",
            "欣赏茶艺表演",
            "品茶时保持安静",
            "杯子轻放不碰撞",
            "感谢茶艺师",
        ]
    }

    /// 茶具礼仪
    pub fn tea_utensils(&self) -> Vec<&'static str> {
        vec![
            "轻拿轻放茶具",
            "不要敲击茶杯",
            "杯盖正确摆放",
            "茶巾保持整洁",
            "不要用手触碰杯口",
            "茶匙使用得当",
            "茶壶倾倒礼仪",
            "茶具是艺术品",
        ]
    }

    /// 茶艺礼仪
    pub fn tea_art(&self) -> Vec<&'static str> {
        vec![
            "欣赏茶艺师表演",
            "保持安静专注",
            "不要打扰表演",
            "适时鼓掌致谢",
            "可以拍照留念",
            "了解茶艺文化",
            "体会茶道精神",
            "尊重传统礼仪",
        ]
    }

    /// 交谈礼仪
    pub fn conversation(&self) -> Vec<&'static str> {
        vec![
            "选择优雅话题",
            "音量适中",
            "不要大声喧哗",
            "讨论茶文化",
            "分享品茶心得",
            "不要谈论俗事",
            "保持文人气质",
            "轮流发言",
        ]
    }

    /// 敬茶礼仪
    pub fn serving_tea(&self) -> Vec<&'static str> {
        vec![
            "双手奉茶",
            "茶杯面向客人",
            "敬茶顺序有讲究",
            "长辈先敬",
            "客人优先",
            "敬茶时说敬语",
            "客人接茶双手",
            "表示感谢",
        ]
    }

    /// 茶食礼仪
    pub fn tea_snacks(&self) -> Vec<&'static str> {
        vec![
            "茶点搭配得当",
            "小口食用",
            "不要用手抓取",
            "茶食不影响品茶",
            "可以分享茶食",
            "保持桌面整洁",
            "茶食精致品尝",
            "感谢主人准备",
        ]
    }

    /// 离座礼仪
    pub fn departure(&self) -> Vec<&'static str> {
        vec![
            "感谢茶艺师",
            "感谢主人款待",
            "收拾个人物品",
            "清理座位",
            "有序离座",
            "茶具归位",
            "保持茶馆整洁",
            "可以赠送小礼",
        ]
    }

    /// 禁忌事项
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要大声喧哗",
            "不要粗鲁对待茶具",
            "不要浪费茶叶",
            "不要谈论粗俗话题",
            "不要在茶馆吸烟",
            "不要占用座位太久",
            "不要打扰茶艺表演",
            "不要带走茶馆物品",
        ]
    }

    /// 文化传承
    pub fn culture(&self) -> Vec<&'static str> {
        vec![
            "了解茶文化历史",
            "体会茶道精神",
            "传承传统礼仪",
            "尊重茶艺传统",
            "欣赏茶艺表演",
            "学习茶艺知识",
            "推广茶文化",
            "保持文人气质",
        ]
    }
}

impl Rule for TeaHouseRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("dining")
    }

    fn explain(&self) -> String {
        format!(
            "【茶馆礼仪】\n\n\
            入座礼仪：\n{}\n\n\
            品茶礼仪：\n{}\n\n\
            茶具礼仪：\n{}\n\n\
            茶艺礼仪：\n{}\n\n\
            交谈礼仪：\n{}\n\n\
            敬茶礼仪：\n{}\n\n\
            茶食礼仪：\n{}\n\n\
            离座礼仪：\n{}\n\n\
            禁忌事项：\n{}\n\n\
            文化传承：\n{}",
            self.seating()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tea_tasting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tea_utensils()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tea_art()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.conversation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.serving_tea()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tea_snacks()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.departure()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.culture()
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
    fn test_tea_house_rules() {
        let rules = TeaHouseRules::new();
        assert_eq!(rules.metadata().name, "茶馆礼仪");
        assert!(!rules.seating().is_empty());
        assert!(!rules.tea_tasting().is_empty());
        assert!(!rules.tea_utensils().is_empty());
        assert!(!rules.tea_art().is_empty());
        assert!(!rules.conversation().is_empty());
        assert!(!rules.serving_tea().is_empty());
        assert!(!rules.tea_snacks().is_empty());
        assert!(!rules.departure().is_empty());
        assert!(!rules.taboos().is_empty());
        assert!(!rules.culture().is_empty());
    }

    #[test]
    fn test_tea_house_validation() {
        let rules = TeaHouseRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("dining"));
    }

    #[test]
    fn test_tea_house_explain() {
        let rules = TeaHouseRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("入座礼仪"));
        assert!(explanation.contains("品茶礼仪"));
        assert!(explanation.contains("茶艺礼仪"));
        assert!(explanation.contains("禁忌事项"));
    }
}
