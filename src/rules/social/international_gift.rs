//! 国际礼物礼仪
//!
//! 涵盖国际礼物赠送的详细规范，包括礼物选择、包装、赠送时机等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InternationalGiftRules,
    name: "国际礼物礼仪",
    desc: "国际礼物礼仪详细规范，包括礼物选择、包装、赠送时机等",
    origin: "国际",
    tags: ["社交", "礼仪", "国际", "礼物"]
}

impl InternationalGiftRules {
    /// 礼物选择礼仪
    pub fn gift_selection(&self) -> Vec<&'static str> {
        vec![
            "选择具有本国特色的礼物",
            "了解受礼者的文化背景",
            "避免赠送禁忌物品",
            "礼物价值适中，不宜过贵",
            "考虑受礼者的个人喜好",
            "选择有纪念意义的礼物",
            "避免送钟表（中国忌讳）",
            "避免送刀具（象征断交）",
        ]
    }

    /// 礼物包装礼仪
    pub fn gift_wrapping(&self) -> Vec<&'static str> {
        vec![
            "礼物应精心包装",
            "包装纸颜色符合文化习俗",
            "中国避免白色和黑色包装",
            "日本避免黑白色，偏好红色",
            "西方国家常用彩色包装",
            "蝴蝶结装饰体现心意",
            "附上礼品卡写明祝福",
            "包装整洁，不显破损",
        ]
    }

    /// 赠送时机礼仪
    pub fn giving_timing(&self) -> Vec<&'static str> {
        vec![
            "正式场合当面赠送礼物",
            "初次见面可赠送小礼品",
            "节日或纪念日是合适的时机",
            "访问时在入座后赠送",
            "离别时赠送纪念品",
            "商务场合在会议结束时赠送",
            "私人拜访在进门时赠送",
            "不宜在公共场合大肆宣扬",
        ]
    }

    /// 赠送方式礼仪
    pub fn giving_manner(&self) -> Vec<&'static str> {
        vec![
            "双手递送礼物",
            "面带微笑，表达祝福",
            "简要说明礼物含义",
            "不要强调礼物价值",
            "不期望立即回赠",
            "接受对方婉拒时不过于坚持",
            "不要催促对方打开礼物",
            "尊重对方是否当面打开",
        ]
    }

    /// 受礼礼仪
    pub fn receiving(&self) -> Vec<&'static str> {
        vec![
            "双手接过礼物",
            "当面表示感谢",
            "询问是否可以打开礼物",
            "当面打开时表达欣赏",
            "不要评论礼物价值",
            "即使不喜欢也礼貌接受",
            "适当时候回赠礼物",
            "礼品卡或信件表示感谢",
        ]
    }

    /// 文化禁忌礼仪
    pub fn cultural_taboos(&self) -> Vec<&'static str> {
        vec![
            "中国：不送钟、伞、鞋、梨",
            "日本：不送梳子、手帕",
            "伊斯兰国家：不送酒类、猪肉",
            "印度：不送牛皮制品",
            "西方：不送奇数花束",
            "俄罗斯：不送偶数花束",
            "韩国：不送绿色包装纸",
            "泰国：避免尖锐物品",
        ]
    }

    /// 商务礼物礼仪
    pub fn business_gifts(&self) -> Vec<&'static str> {
        vec![
            "商务礼物应体现专业性",
            "避免过于私人的礼物",
            "价值符合公司政策",
            "不赠送可能引起误解的礼物",
            "了解对方公司的礼品政策",
            "商务礼品附上名片",
            "群访时可赠送集体礼物",
            "保持礼品的商务风格",
        ]
    }

    /// 回礼礼仪
    pub fn reciprocal_gifts(&self) -> Vec<&'static str> {
        vec![
            "收到礼物后适时回赠",
            "回礼价值相当",
            "回礼体现对等原则",
            "不必过于强调礼尚往来",
            "回礼时机选择恰当",
            "回礼附上感谢语",
            "可以服务或帮助作为回礼",
            "保持礼尚往来的友好关系",
        ]
    }
}

impl Rule for InternationalGiftRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("international")
    }

    fn explain(&self) -> String {
        format!(
            "【国际礼物礼仪】\n\n\
            礼物选择礼仪：\n{}\n\n\
            礼物包装礼仪：\n{}\n\n\
            赠送时机礼仪：\n{}\n\n\
            赠送方式礼仪：\n{}\n\n\
            受礼礼仪：\n{}\n\n\
            文化禁忌礼仪：\n{}\n\n\
            商务礼物礼仪：\n{}\n\n\
            回礼礼仪：\n{}",
            self.gift_selection()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.gift_wrapping()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.giving_timing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.giving_manner()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.receiving()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.business_gifts()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.reciprocal_gifts()
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
    fn test_international_gift_rules() {
        let rules = InternationalGiftRules::new();
        assert_eq!(rules.metadata().name, "国际礼物礼仪");
        assert!(!rules.gift_selection().is_empty());
        assert!(!rules.gift_wrapping().is_empty());
        assert!(!rules.giving_timing().is_empty());
        assert!(!rules.giving_manner().is_empty());
        assert!(!rules.receiving().is_empty());
        assert!(!rules.cultural_taboos().is_empty());
        assert!(!rules.business_gifts().is_empty());
        assert!(!rules.reciprocal_gifts().is_empty());
    }

    #[test]
    fn test_international_gift_validation() {
        let rules = InternationalGiftRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("international"));
    }

    #[test]
    fn test_international_gift_explain() {
        let rules = InternationalGiftRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("礼物选择礼仪"));
        assert!(explanation.contains("礼物包装礼仪"));
        assert!(explanation.contains("赠送时机礼仪"));
        assert!(explanation.contains("受礼礼仪"));
    }
}