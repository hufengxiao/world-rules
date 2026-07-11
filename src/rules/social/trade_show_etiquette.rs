//! 展会礼仪
//!
//! 涵盖商务展会参与相关的礼仪规范，包括展位布置、客户接待、展会交流等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TradeShowEtiquetteRules,
    name: "展会礼仪",
    desc: "商务展会参与礼仪规范，包括展位布置、客户接待、展会交流等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "展会", "展览"]
}

impl TradeShowEtiquetteRules {
    /// 展前准备礼仪
    pub fn before_show(&self) -> Vec<&'static str> {
        vec![
            "设计专业展位形象",
            "准备充足的宣传材料",
            "培训展位工作人员",
            "制定展会目标计划",
            "预约重要客户会面",
            "准备演示产品设备",
            "确认展位位置和规格",
            "安排展会后勤支持",
        ]
    }

    /// 展位布置礼仪
    pub fn booth_setup(&self) -> Vec<&'static str> {
        vec![
            "展位设计专业美观",
            "品牌标识清晰醒目",
            "产品陈列井然有序",
            "宣传材料摆放整齐",
            "预留客户交谈空间",
            "提供舒适的座位",
            "保持展位整洁干净",
            "照明和音响效果适当",
        ]
    }

    /// 展位人员礼仪
    pub fn staff_behavior(&self) -> Vec<&'static str> {
        vec![
            "着装统一整洁",
            "站姿端正面带微笑",
            "主动但不过度推销",
            "熟悉产品和服务知识",
            "准备好名片和资料",
            "避免在展位内吃东西",
            "不玩手机或闲聊",
            "保持展位始终有人值守",
        ]
    }

    /// 客户接待礼仪
    pub fn client_reception(&self) -> Vec<&'static str> {
        vec![
            "主动问候驻足客户",
            "简短介绍吸引兴趣",
            "倾听客户需求和问题",
            "提供专业解答和建议",
            "展示产品特点优势",
            "收集客户联系方式",
            "安排后续详细洽谈",
            "感谢客户光临展位",
        ]
    }

    /// 展会交流礼仪
    pub fn networking(&self) -> Vec<&'static str> {
        vec![
            "主动与同行友好交流",
            "尊重竞争对手展位",
            "参加展会相关活动",
            "参与行业论坛讲座",
            "建立行业人脉关系",
            "交换名片保持联系",
            "学习行业最新动态",
            "遵守展会规则秩序",
        ]
    }

    /// 展会后跟进礼仪
    pub fn after_show(&self) -> Vec<&'static str> {
        vec![
            "及时整理客户信息",
            "尽快联系意向客户",
            "发送感谢邮件",
            "提供展会承诺的资料",
            "安排后续商务洽谈",
            "评估展会效果",
            "总结展会经验教训",
            "更新潜在客户数据库",
        ]
    }

    /// 国际展会礼仪
    pub fn international(&self) -> Vec<&'static str> {
        vec![
            "准备多语言宣传材料",
            "安排翻译人员在场",
            "了解当地文化习俗",
            "遵守当地展会规定",
            "尊重国际客户差异",
            "注意时区和语言安排",
            "了解国际商务礼仪",
            "准备跨境合作方案",
        ]
    }

    /// 展会禁忌
    pub fn taboos(&self) -> Vec<&'static str> {
        vec![
            "不要强行拉扯客户",
            "不要贬低竞争对手",
            "不要在展位内大声喧哗",
            "不要忽视来访客户",
            "不要过早离开展位",
            "不要乱扔宣传材料",
            "不要占用他人展位空间",
            "不要在展位内处理私事",
        ]
    }
}

impl Rule for TradeShowEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【展会礼仪】\n\n\
            展前准备礼仪：\n{}\n\n\
            展位布置礼仪：\n{}\n\n\
            展位人员礼仪：\n{}\n\n\
            客户接待礼仪：\n{}\n\n\
            展会交流礼仪：\n{}\n\n\
            展会后跟进礼仪：\n{}\n\n\
            国际展会礼仪：\n{}\n\n\
            展会禁忌：\n{}",
            self.before_show()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.booth_setup()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.staff_behavior()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.client_reception()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.networking()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.after_show()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.international()
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
    fn test_trade_show_rules() {
        let rules = TradeShowEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "展会礼仪");
        assert!(!rules.before_show().is_empty());
        assert!(!rules.booth_setup().is_empty());
        assert!(!rules.staff_behavior().is_empty());
        assert!(!rules.client_reception().is_empty());
        assert!(!rules.networking().is_empty());
        assert!(!rules.after_show().is_empty());
        assert!(!rules.international().is_empty());
        assert!(!rules.taboos().is_empty());
    }

    #[test]
    fn test_trade_show_validation() {
        let rules = TradeShowEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_trade_show_explain() {
        let rules = TradeShowEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("展前准备礼仪"));
        assert!(explanation.contains("展位布置礼仪"));
        assert!(explanation.contains("客户接待礼仪"));
        assert!(explanation.contains("展会禁忌"));
    }
}