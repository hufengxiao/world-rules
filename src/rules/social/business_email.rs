//! 商务邮件礼仪
//!
//! 涵盖商务邮件写作规范，包括格式、语言、回复规则等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BusinessEmailRules,
    name: "商务邮件礼仪",
    desc: "商务邮件写作规范，包括格式、语言、回复规则等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "邮件", "沟通"]
}

impl BusinessEmailRules {
    /// 邮件格式规范
    pub fn format_rules(&self) -> Vec<&'static str> {
        vec![
            "使用专业的电子邮件地址",
            "主题行简洁明确，不超过50字符",
            "称呼得体，根据关系选择",
            "正文段落清晰，逻辑分明",
            "使用要点列表提高可读性",
            "结尾礼貌，如'此致敬礼'",
            "签名包含完整联系信息",
            "附件命名规范，大小适中",
        ]
    }

    /// 主题行规范
    pub fn subject_line(&self) -> Vec<&'static str> {
        vec![
            "包含核心主题和关键信息",
            "使用项目名称或编号便于检索",
            "紧急邮件可标注[紧急]",
            "回复时保持主题连贯，可添加'回复：'",
            "避免使用全部大写",
            "避免空白主题行",
            "避免过度使用感叹号",
            "简洁但不要过于简略",
        ]
    }

    /// 称呼和结尾
    pub fn salutations(&self) -> Vec<&'static str> {
        vec![
            "首次联系：尊敬的[姓]先生/女士",
            "熟悉后：您好[名]或[姓名]",
            "群发：尊敬的各位/大家好",
            "结尾：此致敬礼/顺颂商祺",
            "回复：谢谢/感谢您的回复",
            "英文：Dear Mr./Ms. [姓], Best regards",
            "避免过于随意的称呼",
            "注意职位和职称的正确使用",
        ]
    }

    /// 正文写作规范
    pub fn body_writing(&self) -> Vec<&'static str> {
        vec![
            "开篇说明目的",
            "内容简洁，避免冗长",
            "重点信息放在开头",
            "使用段落分隔不同主题",
            "数字列表清晰明了",
            "语气专业但不过于生硬",
            "避免使用俚语和网络用语",
            "检查拼写和语法错误",
            "避免全部大写或过多感叹号",
            "重要信息加粗但不过度使用",
        ]
    }

    /// 回复规则
    pub fn reply_rules(&self) -> Vec<&'static str> {
        vec![
            "24小时内回复重要邮件",
            "优先处理紧急和重要邮件",
            "回复时引用相关内容",
            "避免不必要的'全部回复'",
            "回复内容针对问题",
            "收到邮件后简短确认",
            "无法立即解决时说明预计时间",
            "假期设置自动回复",
        ]
    }

    /// 抄送和密送规则
    pub fn cc_bcc_rules(&self) -> Vec<&'static str> {
        vec![
            "抄送需要知情的相关人员",
            "避免抄送无关人员",
            "密送用于保护隐私",
            "避免滥用密送",
            "注意密送回复只回复发件人",
            "抄送时考虑收件人感受",
            "谨慎选择抄送对象",
            "大群体邮件可使用密送保护地址",
        ]
    }

    /// 附件礼仪
    pub fn attachment_rules(&self) -> Vec<&'static str> {
        vec![
            "文件命名清晰明确",
            "单个附件不超过10MB",
            "大文件使用云存储链接",
            "正文中说明附件内容",
            "常见格式（PDF、DOC、XLS）",
            "病毒扫描后再发送",
            "避免过多附件",
            "压缩多个相关文件",
        ]
    }

    /// 文化差异注意
    pub fn cultural_differences(&self) -> Vec<&'static str> {
        vec![
            "美国：简洁直接，效率优先",
            "英国：礼貌正式，委婉表达",
            "法国：正式开头，结构严谨",
            "德国：精确详尽，注重细节",
            "日本：高度礼貌，谦逊表达",
            "中国：注重关系，委婉礼貌",
            "中东：问候和祝福语重要",
            "拉美：友好热情，可能较长",
        ]
    }

    /// 常见错误
    pub fn common_mistakes(&self) -> Vec<&'static str> {
        vec![
            "主题不明确或空白",
            "称呼不当或缺失",
            "正文冗长无重点",
            "语气过于随意",
            "语法和拼写错误",
            "附件忘记添加",
            "收件人地址错误",
            "情绪化回复",
            "过度使用'紧急'标记",
            "回复过慢或忽略邮件",
        ]
    }
}

impl Rule for BusinessEmailRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【商务邮件礼仪】\n\n\
            邮件格式规范：\n{}\n\n\
            主题行规范：\n{}\n\n\
            称呼和结尾：\n{}\n\n\
            正文写作规范：\n{}\n\n\
            回复规则：\n{}\n\n\
            抄送和密送规则：\n{}\n\n\
            附件礼仪：\n{}\n\n\
            文化差异注意：\n{}\n\n\
            常见错误：\n{}",
            self.format_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.subject_line()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.salutations()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.body_writing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.reply_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cc_bcc_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.attachment_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_differences()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.common_mistakes()
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
    fn test_business_email_rules() {
        let rules = BusinessEmailRules::new();
        assert_eq!(rules.metadata().name, "商务邮件礼仪");
        assert!(!rules.format_rules().is_empty());
        assert!(!rules.subject_line().is_empty());
        assert!(!rules.salutations().is_empty());
        assert!(!rules.body_writing().is_empty());
        assert!(!rules.reply_rules().is_empty());
        assert!(!rules.cc_bcc_rules().is_empty());
        assert!(!rules.attachment_rules().is_empty());
        assert!(!rules.cultural_differences().is_empty());
        assert!(!rules.common_mistakes().is_empty());
    }

    #[test]
    fn test_business_email_validation() {
        let rules = BusinessEmailRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_business_email_explain() {
        let rules = BusinessEmailRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("邮件格式规范"));
        assert!(explanation.contains("正文写作规范"));
        assert!(explanation.contains("常见错误"));
    }
}
