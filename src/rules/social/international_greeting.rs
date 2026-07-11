//! 国际问候礼仪
//!
//! 涵盖国际问候的详细规范，包括握手、鞠躬、亲吻礼等各国的问候习俗。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InternationalGreetingRules,
    name: "国际问候礼仪",
    desc: "国际问候礼仪详细规范，包括握手、鞠躬、亲吻礼等各国的问候习俗",
    origin: "国际",
    tags: ["社交", "礼仪", "国际", "问候"]
}

impl InternationalGreetingRules {
    /// 握手礼仪
    pub fn handshake(&self) -> Vec<&'static str> {
        vec![
            "握手力度适中，不过重或过轻",
            "目光注视对方眼睛",
            "握手时间约2-3秒",
            "手掌垂直，表示平等",
            "避免湿手握手",
            "主动伸手表示尊重",
            "女士优先伸手",
            "上级优先伸手",
        ]
    }

    /// 鞠躬礼仪
    pub fn bowing(&self) -> Vec<&'static str> {
        vec![
            "日本：鞠躬角度越深越尊重",
            "普通问候15度鞠躬",
            "正式问候30度鞠躬",
            "最高敬意45度鞠躬",
            "韩国：鞠躬与握手结合",
            "中国：微微鞠躬表示尊重",
            "鞠躬时保持背部挺直",
            "眼神适当下垂，不直视",
        ]
    }

    /// 亲吻礼礼仪
    pub fn kissing(&self) -> Vec<&'static str> {
        vec![
            "法国：脸颊轻触，左右各一次",
            "比利时：一次或三次轻触",
            "意大利：熟人之间两次轻触",
            "西班牙：两次轻触较常见",
            "俄罗斯：熟悉的朋友间亲吻",
            "中东地区：男性间亲吻额头",
            "只适用于熟人之间",
            "避免亲吻不熟悉的人",
        ]
    }

    /// 名片交换礼仪
    pub fn business_card_exchange(&self) -> Vec<&'static str> {
        vec![
            "双手递送名片",
            "名片正面朝向接收方",
            "双手接过对方名片",
            "认真阅读对方名片",
            "名片放在名片夹中",
            "不随意在名片上写字",
            "日本：名片表示身份，应尊重",
            "交换名片时鞠躬致意",
        ]
    }

    /// 称呼礼仪
    pub fn addressing(&self) -> Vec<&'static str> {
        vec![
            "使用正确的称呼和头衔",
            "西方：Mr./Ms. + 姓氏",
            "日本：姓氏 + さん（San）",
            "中国：职位 + 姓氏",
            "韩国：职位或先生/女士",
            "不确定时使用尊称",
            "注意学位和专业头衔",
            "熟悉后可使用名字",
        ]
    }

    /// 各国问候习俗
    pub fn country_customs(&self) -> Vec<&'static str> {
        vec![
            "美国：热情握手，直呼其名",
            "英国：握手，正式场合用头衔",
            "法国：亲吻礼（朋友间）",
            "德国：握手，正式问候",
            "日本：鞠躬，交换名片",
            "中国：握手，微微点头",
            "印度：合十礼（Namaste）",
            "泰国：合十礼，手放胸前",
        ]
    }

    /// 特殊场合问候
    pub fn special_occasions(&self) -> Vec<&'static str> {
        vec![
            "正式场合：使用正式问候语",
            "商务场合：握手并交换名片",
            "宴会场合：微笑点头致意",
            "宗教场所：遵守当地习俗",
            "皇室场合：遵守皇家礼仪",
            "军事场合：敬礼或站立致意",
            "学术场合：学位和头衔称呼",
            "体育场合：友好握手或拥抱",
        ]
    }

    /// 禁忌礼仪
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "伊斯兰国家：异性间避免肢体接触",
            "日本：不要过度握手或拥抱",
            "泰国：不要摸头（包括儿童）",
            "印度：左手被认为不洁",
            "穆斯林：同性间问候不过于亲密",
            "欧洲：不熟悉的人不亲吻",
            "中国：不熟悉的异性不过于亲密",
            "韩国：晚辈先问候长辈",
        ]
    }
}

impl Rule for InternationalGreetingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("international")
    }

    fn explain(&self) -> String {
        format!(
            "【国际问候礼仪】\n\n\
            握手礼仪：\n{}\n\n\
            鞠躬礼仪：\n{}\n\n\
            亲吻礼礼仪：\n{}\n\n\
            名片交换礼仪：\n{}\n\n\
            称呼礼仪：\n{}\n\n\
            各国问候习俗：\n{}\n\n\
            特殊场合问候：\n{}\n\n\
            禁忌礼仪：\n{}",
            self.handshake()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.bowing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.kissing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.business_card_exchange()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.addressing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.country_customs()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.special_occasions()
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
    fn test_international_greeting_rules() {
        let rules = InternationalGreetingRules::new();
        assert_eq!(rules.metadata().name, "国际问候礼仪");
        assert!(!rules.handshake().is_empty());
        assert!(!rules.bowing().is_empty());
        assert!(!rules.kissing().is_empty());
        assert!(!rules.business_card_exchange().is_empty());
        assert!(!rules.addressing().is_empty());
        assert!(!rules.country_customs().is_empty());
        assert!(!rules.special_occasions().is_empty());
        assert!(!rules.taboos().is_empty());
    }

    #[test]
    fn test_international_greeting_validation() {
        let rules = InternationalGreetingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("international"));
    }

    #[test]
    fn test_international_greeting_explain() {
        let rules = InternationalGreetingRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("握手礼仪"));
        assert!(explanation.contains("鞠躬礼仪"));
        assert!(explanation.contains("亲吻礼礼仪"));
        assert!(explanation.contains("名片交换礼仪"));
    }
}
