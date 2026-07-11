//! 国际会议礼仪
//!
//! 涵盖国际会议的详细规范，包括会议筹备、参与礼仪、发言规则等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InternationalConferenceRules,
    name: "国际会议礼仪",
    desc: "国际会议礼仪详细规范，包括会议筹备、参与礼仪、发言规则等",
    origin: "国际",
    tags: ["社交", "礼仪", "国际", "会议"]
}

impl InternationalConferenceRules {
    /// 会议筹备礼仪
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前发送正式邀请函",
            "明确会议议程和时间安排",
            "提供会议资料和背景文件",
            "安排翻译和同声传译服务",
            "准备会议设施和设备",
            "安排会场布置和座位",
            "准备茶歇和餐饮服务",
            "制定应急预案",
        ]
    }

    /// 报到注册礼仪
    pub fn registration(&self) -> Vec<&'static str> {
        vec![
            "提前到达会场报到",
            "携带有效证件和邀请函",
            "领取会议资料和胸牌",
            "确认座位安排和分组",
            "了解会场设施和紧急出口",
            "熟悉同声传译设备使用",
            "保存好会议资料和证件",
            "遵守保密规定",
        ]
    }

    /// 座次礼仪
    pub fn seating_protocol(&self) -> Vec<&'static str> {
        vec![
            "按礼宾顺序安排座位",
            "主席台座位按职位高低排列",
            "主宾位于主席右侧",
            "代表团按国名英文字母排列",
            "圆桌会议体现平等原则",
            "同声传译室安排在后方",
            "媒体席安排在会场后方",
            "观察员席位按国际惯例",
        ]
    }

    /// 发言礼仪
    pub fn speaking(&self) -> Vec<&'static str> {
        vec![
            "发言前向主席请求发言权",
            "发言时站立，面向主席",
            "发言简明扼要，不超时",
            "使用正式外交语言",
            "避免使用攻击性言辞",
            "引用资料需注明来源",
            "发言结束向主席致谢",
            "回答提问时态度友善",
        ]
    }

    /// 同声传译礼仪
    pub fn interpretation(&self) -> Vec<&'static str> {
        vec![
            "发言速度适中，便于翻译",
            "使用同声传译设备",
            "选择所需语言的频道",
            "避免使用难以翻译的俚语",
            "技术术语提前提供译法",
            "配合翻译人员的节奏",
            "如需停顿可提前说明",
            "感谢翻译人员的付出",
        ]
    }

    /// 投票表决礼仪
    pub fn voting(&self) -> Vec<&'static str> {
        vec![
            "投票前认真阅读决议草案",
            "不清楚时可请求解释",
            "投票时保持安静",
            "举手或电子投票按规定",
            "尊重投票结果",
            "可要求记录投票立场",
            "弃权需说明理由",
            "投票后不发表争议言论",
        ]
    }

    /// 会间交流礼仪
    pub fn networking(&self) -> Vec<&'static str> {
        vec![
            "茶歇时主动与其他代表交流",
            "交换名片并妥善保存",
            "讨论议题时保持专业",
            "避免私下达成未公开协议",
            "尊重不同观点和文化",
            "不泄露会议敏感信息",
            "建立专业人际关系",
            "会后保持联系和沟通",
        ]
    }

    /// 会议着装礼仪
    pub fn dress_code(&self) -> Vec<&'static str> {
        vec![
            "正式会议着正装",
            "男性穿深色西装，配领带",
            "女性着职业套装或正装",
            "可着民族服装",
            "避免过于鲜艳的颜色",
            "饰品简洁大方",
            "保持个人形象整洁",
            "着装符合会议主题",
        ]
    }
}

impl Rule for InternationalConferenceRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("international")
    }

    fn explain(&self) -> String {
        format!(
            "【国际会议礼仪】\n\n\
            会议筹备礼仪：\n{}\n\n\
            报到注册礼仪：\n{}\n\n\
            座次礼仪：\n{}\n\n\
            发言礼仪：\n{}\n\n\
            同声传译礼仪：\n{}\n\n\
            投票表决礼仪：\n{}\n\n\
            会间交流礼仪：\n{}\n\n\
            会议着装礼仪：\n{}",
            self.preparation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.registration()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.seating_protocol()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.speaking()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.interpretation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.voting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.networking()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dress_code()
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
    fn test_international_conference_rules() {
        let rules = InternationalConferenceRules::new();
        assert_eq!(rules.metadata().name, "国际会议礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.registration().is_empty());
        assert!(!rules.seating_protocol().is_empty());
        assert!(!rules.speaking().is_empty());
        assert!(!rules.interpretation().is_empty());
        assert!(!rules.voting().is_empty());
        assert!(!rules.networking().is_empty());
        assert!(!rules.dress_code().is_empty());
    }

    #[test]
    fn test_international_conference_validation() {
        let rules = InternationalConferenceRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("international"));
    }

    #[test]
    fn test_international_conference_explain() {
        let rules = InternationalConferenceRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("会议筹备礼仪"));
        assert!(explanation.contains("报到注册礼仪"));
        assert!(explanation.contains("座次礼仪"));
        assert!(explanation.contains("发言礼仪"));
    }
}
