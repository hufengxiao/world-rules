//! 商务谈判礼仪
//!
//! 涵盖商务谈判各阶段的礼仪规范，包括准备、开场、谈判过程和后续跟进。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: NegotiationEtiquetteRules,
    name: "商务谈判礼仪",
    desc: "商务谈判礼仪规范，包括谈判准备、过程礼仪、文化差异等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "谈判"]
}

impl NegotiationEtiquetteRules {
    /// 谈判前准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "研究对方公司背景和决策者",
            "了解谈判对象的文化习俗",
            "明确己方目标和底线",
            "准备多种可行方案",
            "收集相关数据和证据",
            "制定谈判策略和时间表",
            "选择合适的谈判地点",
            "确认参会人员和角色分工",
        ]
    }

    /// 开场礼仪
    pub fn opening(&self) -> Vec<&'static str> {
        vec![
            "准时到达，着装得体",
            "面带微笑，主动问候",
            "按礼仪顺序介绍团队成员",
            "交换名片并认真查看",
            "选择合适的座位安排",
            "简短寒暄建立氛围",
            "说明会议议程和预期时长",
            "确认对方授权范围",
        ]
    }

    /// 谈判过程礼仪
    pub fn during_negotiation(&self) -> Vec<&'static str> {
        vec![
            "保持专业和礼貌的态度",
            "专注倾听，不随意打断",
            "提出问题清晰具体",
            "回答问题诚实谨慎",
            "避免情绪化表达",
            "记录要点和承诺",
            "尊重对方立场和观点",
            "适时总结和确认",
            "控制谈判节奏",
            "遇到僵局保持冷静",
        ]
    }

    /// 谈判语言技巧
    pub fn language_skills(&self) -> Vec<&'static str> {
        vec![
            "使用积极正面的语言",
            "避免绝对化表述",
            "用'我们'代替'你'",
            "提出建议而非命令",
            "肯定对方观点后再表达异议",
            "避免威胁性语言",
            "用数据支持观点",
            "提问代替直接反对",
        ]
    }

    /// 报价和让步礼仪
    pub fn offer_concession(&self) -> Vec<&'static str> {
        vec![
            "首次报价留有协商空间",
            "让步要循序渐进",
            "每次让步都应获取回报",
            "解释让步原因和困难",
            "不要急于接受首次报价",
            "使用条件性让步",
            "记录所有让步和承诺",
            "避免在核心利益上轻易让步",
        ]
    }

    /// 谈判结束礼仪
    pub fn closing(&self) -> Vec<&'static str> {
        vec![
            "确认所有条款和细节",
            "书面记录达成的一致意见",
            "明确后续步骤和责任",
            "感谢对方的合作",
            "握手表示诚意",
            "无论结果如何都保持专业",
            "及时发送会议纪要",
            "跟进履行承诺",
        ]
    }

    /// 文化差异注意事项
    pub fn cultural_notes(&self) -> Vec<&'static str> {
        vec![
            "美国：直接表达，效率优先，合同至上",
            "日本：重视关系，避免直接冲突，集体决策",
            "中国：注重面子，先交朋友后谈生意，灵活处理",
            "德国：严谨认真，准备充分，尊重规则",
            "法国：重视人际关系，可能迂回，强调逻辑",
            "中东：建立信任需要时间，重视待客之道",
            "印度：关系和等级重要，需要耐心",
            "拉美：热情友好，时间观念相对灵活",
        ]
    }
}

impl Rule for NegotiationEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【商务谈判礼仪】\n\n\
            谈判前准备：\n{}\n\n\
            开场礼仪：\n{}\n\n\
            谈判过程礼仪：\n{}\n\n\
            谈判语言技巧：\n{}\n\n\
            报价和让步礼仪：\n{}\n\n\
            谈判结束礼仪：\n{}\n\n\
            文化差异注意事项：\n{}",
            self.preparation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.opening()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.during_negotiation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.language_skills()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.offer_concession()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.closing()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_notes()
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
    fn test_negotiation_etiquette_rules() {
        let rules = NegotiationEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "商务谈判礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.opening().is_empty());
        assert!(!rules.during_negotiation().is_empty());
        assert!(!rules.language_skills().is_empty());
        assert!(!rules.offer_concession().is_empty());
        assert!(!rules.closing().is_empty());
        assert!(!rules.cultural_notes().is_empty());
    }

    #[test]
    fn test_negotiation_etiquette_validation() {
        let rules = NegotiationEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_negotiation_etiquette_explain() {
        let rules = NegotiationEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("谈判前准备"));
        assert!(explanation.contains("开场礼仪"));
        assert!(explanation.contains("谈判过程礼仪"));
        assert!(explanation.contains("文化差异"));
    }
}
