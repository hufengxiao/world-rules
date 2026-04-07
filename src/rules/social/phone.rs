//! 电话礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 电话礼仪
pub struct PhoneEtiquette {
    metadata: RuleMetadata,
}

impl PhoneEtiquette {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "电话礼仪",
                "电话沟通礼仪规范"
            )
            .with_origin("现代社交")
            .with_tags(vec!["社交".into(), "沟通".into()]),
        }
    }

    /// 接电话礼仪
    pub fn answering_calls(&self) -> Vec<&'static str> {
        vec![
            "尽快接听(铃响三声内)",
            "先问候再自报身份",
            "语气亲切友善",
            "认真倾听不打断",
            "做好记录准备",
            "公务电话要专业",
        ]
    }

    /// 打电话礼仪
    pub fn making_calls(&self) -> Vec<&'static str> {
        vec![
            "选择合适时间",
            "避开休息时间和用餐时间",
            "先问候再说明目的",
            "简洁明了表达",
            "确认对方理解",
            "礼貌结束通话",
        ]
    }

    /// 结束通话
    pub fn ending_calls(&self) -> Vec<&'static str> {
        vec![
            "确认交流完成",
            "感谢对方",
            "道别",
            "上级/长辈先挂电话",
            "客人先挂电话",
            "打电话方先挂电话",
        ]
    }

    /// 手机使用礼仪
    pub fn mobile_phone_usage(&self) -> Vec<&'static str> {
        vec![
            "会议中关闭或静音",
            "公共场所不大声通话",
            "不在他人面前玩手机",
            "走路时不看手机",
            "开车时不使用手机",
            "图书馆/影院等场所不使用",
        ]
    }

    /// 转接电话
    pub fn transferring_calls(&self) -> Vec<&'static str> {
        vec![
            "告知对方将要转接",
            "说明转接原因",
            "告知转接对象",
            "确认对方同意",
            "转接后确认接通",
            "无法转接时说明原因",
        ]
    }

    /// 留言礼仪
    pub fn leaving_messages(&self) -> Vec<&'static str> {
        vec![
            "留言简洁明了",
            "说明身份和联系方式",
            "说明留言目的",
            "留言时间",
            "请求回电时间",
            "感谢对方",
        ]
    }

    /// 视频通话礼仪
    pub fn video_call_rules(&self) -> Vec<&'static str> {
        vec![
            "确保环境整洁",
            "注意着装",
            "保持适当距离",
            "保持摄像头稳定",
            "注意背景内容",
            "保持眼神接触",
            "避免干扰",
        ]
    }
}

impl Default for PhoneEtiquette {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for PhoneEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("phone")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【电话礼仪】\n\n\
            接电话礼仪:\n{}\n\n\
            打电话礼仪:\n{}\n\n\
            手机使用礼仪:\n{}\n\n\
            视频通话礼仪:\n{}\n",
            self.answering_calls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.making_calls().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.mobile_phone_usage().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.video_call_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_etiquette() {
        let rules = PhoneEtiquette::new();
        assert!(!rules.answering_calls().is_empty());
    }
}