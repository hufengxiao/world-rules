//! 面试礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 面试礼仪规则
pub struct InterviewEtiquette {
    metadata: RuleMetadata,
}

impl InterviewEtiquette {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "面试礼仪",
                "求职面试礼仪规范"
            )
            .with_origin("通用")
            .with_tags(vec!["社交".into(), "面试".into(), "求职".into()]),
        }
    }

    /// 面试前准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "研究公司背景和职位要求",
            "准备简历和相关材料",
            "了解面试流程",
            "准备常见问题回答",
            "确认面试时间和地点",
            "准备好着装",
        ]
    }

    /// 着装要求
    pub fn dress_code(&self) -> Vec<&'static str> {
        vec![
            "正装为主 (西装/衬衫)",
            "颜色稳重 (深色系)",
            "整洁干净",
            "避免过于花哨",
            "发型整洁",
            "鞋子干净",
        ]
    }

    /// 面试中礼仪
    pub fn during_interview(&self) -> Vec<&'static str> {
        vec![
            "提前10-15分钟到达",
            "敲门后进入",
            "主动问候面试官",
            "坐姿端正",
            "眼神交流",
            "回答问题清晰简洁",
            "保持微笑",
            "不要打断面试官",
            "不要嚼口香糖",
        ]
    }

    /// 面试后礼仪
    pub fn after_interview(&self) -> Vec<&'static str> {
        vec![
            "感谢面试官",
            "询问后续流程",
            "发送感谢邮件",
            "保持耐心等待",
            "做好其他准备",
        ]
    }

    /// 常见错误
    pub fn common_mistakes(&self) -> Vec<&'static str> {
        vec![
            "迟到",
            "着装不当",
            "手机未静音",
            "回答过于简短或冗长",
            "贬低前雇主",
            "态度傲慢或过于紧张",
            "隐瞒重要信息",
            "对职位不了解",
        ]
    }
}

impl Default for InterviewEtiquette {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for InterviewEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("interview")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【面试礼仪】\n\n\
            面试前准备:\n{}\n\n\
            着装要求:\n{}\n\n\
            面试中礼仪:\n{}\n\n\
            面试后礼仪:\n{}\n\n\
            常见错误:\n{}\n",
            self.preparation().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.dress_code().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.during_interview().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.after_interview().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n"),
            self.common_mistakes().iter().map(|s| format!("  • {}", s)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interview_etiquette() {
        let etiquette = InterviewEtiquette::new();
        assert!(etiquette.during_interview().contains(&"提前10-15分钟到达"));
    }
}