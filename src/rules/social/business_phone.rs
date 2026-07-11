//! 商务电话礼仪
//!
//! 涵盖商务电话沟通规范，包括接打电话礼仪、留言规则、会议电话礼仪等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BusinessPhoneRules,
    name: "商务电话礼仪",
    desc: "商务电话沟通规范，包括接打电话礼仪、留言规则、会议电话礼仪等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "电话", "沟通"]
}

impl BusinessPhoneRules {
    /// 拨打电话礼仪
    pub fn calling_rules(&self) -> Vec<&'static str> {
        vec![
            "选择合适的时间（工作时间：9:00-17:00）",
            "避开午休时间（12:00-14:00）",
            "提前准备好通话要点",
            "准备好相关文件和资料",
            "确认号码无误后再拨打",
            "电话接通后自报家门",
            "询问对方是否方便通话",
            "控制通话时间，简洁明了",
            "结束前确认关键信息",
            "礼貌结束通话",
        ]
    }

    /// 接听电话礼仪
    pub fn answering_rules(&self) -> Vec<&'static str> {
        vec![
            "电话铃响三声内接听",
            "清晰问候并自报部门",
            "语音清晰，语气友好",
            "准备记录工具",
            "认真倾听，不随意打断",
            "重要信息复述确认",
            "无法立即回答时说明",
            "转接电话说明原因",
            "通话结束等待对方先挂",
            "记录重要通话内容",
        ]
    }

    /// 留言礼仪
    pub fn voicemail_rules(&self) -> Vec<&'static str> {
        vec![
            "简明扼要说明身份和目的",
            "留下回电号码",
            "说明最佳回电时间",
            "控制时长（30秒内）",
            "语速适中，吐字清晰",
            "避免复杂或敏感信息",
            "结束前再次确认联系方式",
            "设置专业的语音信箱问候语",
        ]
    }

    /// 会议电话礼仪
    pub fn conference_call(&self) -> Vec<&'static str> {
        vec![
            "提前测试设备连接",
            "选择安静的环境",
            "按时加入会议",
            "加入时自报姓名",
            "不发言时静音",
            "发言前说明姓名",
            "避免打断他人发言",
            "注意语速和音量",
            "不进行私下交谈",
            "结束前感谢主持人",
        ]
    }

    /// 手机使用礼仪
    pub fn mobile_phone(&self) -> Vec<&'static str> {
        vec![
            "会议中手机静音或关机",
            "重要场合不接打电话",
            "公共场合控制音量",
            "避免在电梯内通话",
            "开车时不接打电话",
            "避免餐桌使用手机",
            "及时回复重要电话",
            "设置专业铃声",
        ]
    }

    /// 国际通话礼仪
    pub fn international_calls(&self) -> Vec<&'static str> {
        vec![
            "注意时差，选择合适时间",
            "了解对方工作时间",
            "使用清晰的英语或翻译",
            "避免俚语和方言",
            "语速放慢，吐字清晰",
            "重要信息书面确认",
            "尊重文化差异",
            "考虑网络质量影响",
        ]
    }

    /// 常见禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "一边吃饭一边打电话",
            "使用免提时不告知对方",
            "通话时做其他事情",
            "挂断电话前不礼貌结束",
            "对方说话时打哈欠",
            "通话中突然消失",
            "使用不专业语气",
            "过早挂断电话",
        ]
    }

    /// 紧急情况处理
    pub fn emergency_handling(&self) -> Vec<&'static str> {
        vec![
            "紧急情况说明优先级",
            "保持冷静和专业",
            "清晰说明问题和需求",
            "提供必要的背景信息",
            "确认下一步行动",
            "记录处理过程",
            "及时跟进结果",
            "必要时升级处理",
        ]
    }
}

impl Rule for BusinessPhoneRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【商务电话礼仪】\n\n\
            拨打电话礼仪：\n{}\n\n\
            接听电话礼仪：\n{}\n\n\
            留言礼仪：\n{}\n\n\
            会议电话礼仪：\n{}\n\n\
            手机使用礼仪：\n{}\n\n\
            国际通话礼仪：\n{}\n\n\
            常见禁忌：\n{}\n\n\
            紧急情况处理：\n{}",
            self.calling_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.answering_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.voicemail_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.conference_call()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.mobile_phone()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.international_calls()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.emergency_handling()
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
    fn test_business_phone_rules() {
        let rules = BusinessPhoneRules::new();
        assert_eq!(rules.metadata().name, "商务电话礼仪");
        assert!(!rules.calling_rules().is_empty());
        assert!(!rules.answering_rules().is_empty());
        assert!(!rules.voicemail_rules().is_empty());
        assert!(!rules.conference_call().is_empty());
        assert!(!rules.mobile_phone().is_empty());
        assert!(!rules.international_calls().is_empty());
        assert!(!rules.taboos().is_empty());
        assert!(!rules.emergency_handling().is_empty());
    }

    #[test]
    fn test_business_phone_validation() {
        let rules = BusinessPhoneRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_business_phone_explain() {
        let rules = BusinessPhoneRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("拨打电话礼仪"));
        assert!(explanation.contains("接听电话礼仪"));
        assert!(explanation.contains("常见禁忌"));
    }
}
