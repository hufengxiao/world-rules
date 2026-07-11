//! 国际商务会议礼仪
//!
//! 涵盖各类商务会议的礼仪规范，包括会议准备、参会礼仪、发言规则等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: MeetingEtiquetteRules,
    name: "国际商务会议礼仪",
    desc: "商务会议礼仪规范，包括会议准备、参会礼仪、发言规则等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "会议"]
}

impl MeetingEtiquetteRules {
    /// 会议前准备
    pub fn before_meeting(&self) -> Vec<&'static str> {
        vec![
            "提前确认会议时间和地点",
            "阅读会议议程和相关材料",
            "准备发言要点和问题",
            "检查技术设备是否正常",
            "提前5-10分钟到达会场",
            "关闭或静音手机",
            "准备笔记本和笔",
            "穿着符合商务礼仪要求",
        ]
    }

    /// 会议中礼仪
    pub fn during_meeting(&self) -> Vec<&'static str> {
        vec![
            "坐在指定或合适的位置",
            "保持专注，认真聆听",
            "适时做笔记",
            "发言前举手或示意",
            "发言简洁明了，不偏离主题",
            "尊重他人发言，不打断",
            "避免使用手机或做与会议无关的事",
            "保持良好的身体语言",
            "与发言者保持眼神交流",
            "避免私下交谈",
        ]
    }

    /// 发言礼仪
    pub fn speaking_rules(&self) -> Vec<&'static str> {
        vec![
            "发言前说明自己的姓名和职位",
            "声音清晰，语速适中",
            "用词专业，避免口语化",
            "结构清晰：开场、内容、总结",
            "控制时间，不超时",
            "引用数据需准确",
            "对不同意见保持尊重",
            "避免情绪化表达",
        ]
    }

    /// 视频会议礼仪
    pub fn video_meeting(&self) -> Vec<&'static str> {
        vec![
            "提前测试网络和设备",
            "选择安静、整洁的背景环境",
            "摄像头与眼睛平齐",
            "保持良好的光线",
            "关闭麦克风除非发言",
            "发言时注视摄像头",
            "避免背景噪音",
            "不随意离开座位",
            "共享屏幕前关闭无关内容",
            "会议结束前等待主持人示意",
        ]
    }

    /// 会议后跟进
    pub fn after_meeting(&self) -> Vec<&'static str> {
        vec![
            "及时发送会议纪要",
            "确认后续行动项和负责人",
            "按时完成分配的任务",
            "有问题及时沟通",
            "归档会议记录",
        ]
    }

    /// 不同文化会议礼仪差异
    pub fn cultural_differences(&self) -> Vec<&'static str> {
        vec![
            "美国：准时，直接表达，高效",
            "日本：提前到达，等级分明，沉默不急于发言",
            "德国：严格守时，准备充分，正式",
            "法国：可能迟到，重视关系，灵活",
            "中国：重视座次，先寒暄后正题，集体决策",
            "英国：正式但不失幽默，礼貌优先",
            "中东：关系优先，可能推迟开始，茶饮待客",
        ]
    }
}

impl Rule for MeetingEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【国际商务会议礼仪】\n\n\
            会议前准备：\n{}\n\n\
            会议中礼仪：\n{}\n\n\
            发言礼仪：\n{}\n\n\
            视频会议礼仪：\n{}\n\n\
            会议后跟进：\n{}\n\n\
            文化差异：\n{}",
            self.before_meeting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.during_meeting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.speaking_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.video_meeting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.after_meeting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_differences()
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
    fn test_meeting_etiquette_rules() {
        let rules = MeetingEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "国际商务会议礼仪");
        assert!(!rules.before_meeting().is_empty());
        assert!(!rules.during_meeting().is_empty());
        assert!(!rules.speaking_rules().is_empty());
        assert!(!rules.video_meeting().is_empty());
        assert!(!rules.after_meeting().is_empty());
        assert!(!rules.cultural_differences().is_empty());
    }

    #[test]
    fn test_meeting_etiquette_validation() {
        let rules = MeetingEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_meeting_etiquette_explain() {
        let rules = MeetingEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("会议前准备"));
        assert!(explanation.contains("会议中礼仪"));
        assert!(explanation.contains("发言礼仪"));
        assert!(explanation.contains("视频会议礼仪"));
        assert!(explanation.contains("文化差异"));
    }
}
