//! 邮件礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 邮件礼仪
pub struct EmailEtiquette {
    metadata: RuleMetadata,
}

impl EmailEtiquette {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "邮件礼仪",
                "电子邮件沟通礼仪规范"
            )
            .with_origin("现代办公")
            .with_tags(vec!["社交".into(), "办公".into()]),
        }
    }

    /// 邮件主题
    pub fn subject_line(&self) -> Vec<&'static str> {
        vec![
            "必须填写主题",
            "主题简洁明了",
            "主题反映邮件内容",
            "紧急邮件标注\"紧急\"",
            "回复时保持原主题或修改",
            "避免空白或无意义主题",
        ]
    }

    /// 邮件开头
    pub fn email_opening(&self) -> Vec<&'static str> {
        vec![
            "适当称呼收件人",
            "商务邮件使用正式称呼",
            "亲友邮件可轻松称呼",
            "说明邮件目的",
            "简短引入正文",
        ]
    }

    /// 正文格式
    pub fn body_format(&self) -> Vec<&'static str> {
        vec![
            "分段清晰",
            "段落简短",
            "重点突出",
            "语言简洁",
            "避免长篇大论",
            "重要信息可加粗或标注",
            "附件需在正文说明",
        ]
    }

    /// 邮件结尾
    pub fn email_ending(&self) -> Vec<&'static str> {
        vec![
            "总结要点",
            "说明后续期望",
            "感谢对方",
            "正式结束语",
            "签名档包含联系信息",
        ]
    }

    /// 回复礼仪
    pub fn reply_etiquette(&self) -> Vec<&'static str> {
        vec![
            "及时回复(24小时内)",
            "回复相关邮件",
            "引用原邮件关键内容",
            "回复所有人时谨慎",
            "不在回复中发表不当言论",
            "转发时征得同意",
        ]
    }

    /// 附件礼仪
    pub fn attachment_rules(&self) -> Vec<&'static str> {
        vec![
            "附件命名清晰",
            "说明附件内容",
            "控制附件大小",
            "压缩大文件",
            "使用通用格式",
            "重要附件备份",
        ]
    }

    /// 常见错误
    pub fn common_mistakes(&self) -> Vec<&'static str> {
        vec![
            "收件人地址错误",
            "忘记添加附件",
            "主题空白或不相关",
            "语气不当",
            "过度使用回复所有人",
            "格式混乱",
            "错别字和语法错误",
        ]
    }
}

impl Default for EmailEtiquette {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for EmailEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("email")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【邮件礼仪】\n\n\
            邮件主题:\n{}\n\n\
            正文格式:\n{}\n\n\
            回复礼仪:\n{}\n\n\
            附件礼仪:\n{}\n",
            self.subject_line().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.body_format().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.reply_etiquette().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.attachment_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_etiquette() {
        let rules = EmailEtiquette::new();
        assert!(!rules.subject_line().is_empty());
    }
}