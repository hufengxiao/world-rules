//! 见面礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 见面礼仪
pub struct GreetingEtiquette {
    metadata: RuleMetadata,
}

impl GreetingEtiquette {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "见面礼仪",
                "日常见面问候礼仪规范"
            )
            .with_origin("中国")
            .with_tags(vec!["社交".into(), "礼仪".into()]),
        }
    }

    /// 握手礼仪
    pub fn handshake_rules(&self) -> Vec<&'static str> {
        vec![
            "伸出右手",
            "力度适中",
            "握手时间约2-3秒",
            "眼睛注视对方",
            "上级/长辈先伸手",
            "女士先伸手",
            "客人到达时主人先伸手",
            "客人离开时客人先伸手",
        ]
    }

    /// 介绍礼仪
    pub fn introduction_rules(&self) -> Vec<&'static str> {
        vec![
            "先介绍晚辈给长辈",
            "先介绍下级给上级",
            "先介绍男士给女士",
            "先介绍主人给客人",
            "先介绍内部人员给外部人员",
            "介绍时使用完整称呼",
            "简要介绍对方身份背景",
        ]
    }

    /// 名片礼仪
    pub fn business_card_rules(&self) -> Vec<&'static str> {
        vec![
            "双手递出名片",
            "名片正面朝向对方",
            "名片高度适中便于阅读",
            "双手接过名片",
            "仔细阅读名片内容",
            "妥善放置名片",
            "不可将名片放入裤兜",
            "不可在名片上写字(未经同意)",
        ]
    }

    /// 称呼礼仪
    pub fn addressing_rules(&self) -> Vec<&'static str> {
        vec![
            "使用适当称呼",
            "职场用职务称呼",
            "长辈用尊称",
            "平辈可用姓名",
            "不可随意称呼",
            "询问对方称呼方式",
            "避免使用绰号(除非关系亲密)",
        ]
    }

    /// 问候语言
    pub fn greeting_words(&self) -> Vec<&'static str> {
        vec![
            "\"您好\" - 通用问候",
            "\"早上好/下午好/晚上好\"",
            "\"好久不见\"",
            "\"近来如何\"",
            "\"欢迎光临\"",
            "\"欢迎再次来访\"",
        ]
    }

    /// 鞠躬礼仪
    pub fn bowing_rules(&self) -> Vec<&'static str> {
        vec![
            "脱帽鞠躬",
            "视线向下",
            "背部保持挺直",
            "鞠躬角度15-90度不等",
            "鞠躬深度表示敬意程度",
            "日本文化中鞠躬更普遍",
        ]
    }

    /// 问候时机
    pub fn greeting_timing(&self) -> Vec<&'static str> {
        vec![
            "见面时主动问候",
            "进入场所时问候在场人员",
            "离开时道别",
            "每天初次见面问候",
            "节日问候",
            "重要场合问候",
        ]
    }
}

impl Default for GreetingEtiquette {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GreetingEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("greeting")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【见面礼仪】\n\n\
            握手规则:\n{}\n\n\
            介绍规则:\n{}\n\n\
            名片礼仪:\n{}\n\n\
            称呼礼仪:\n{}\n",
            self.handshake_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.introduction_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.business_card_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.addressing_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting_etiquette() {
        let rules = GreetingEtiquette::new();
        assert!(!rules.handshake_rules().is_empty());
    }
}