//! 商务社交礼仪
//!
//! 涵盖商务社交场合的礼仪规范，包括接待、拜访、商务宴请等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: BusinessNetworkingRules,
    name: "商务社交礼仪",
    desc: "商务社交场合礼仪规范，包括接待、拜访、商务宴请等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "接待", "拜访"]
}

impl BusinessNetworkingRules {
    /// 客户接待礼仪
    pub fn client_reception(&self) -> Vec<&'static str> {
        vec![
            "提前准备接待方案",
            "确认接待时间和地点",
            "安排接待人员和车辆",
            "准备会议场所和茶水",
            "热情迎接客人",
            "引导路线清晰",
            "介绍接待人员",
            "确保会议室整洁",
            "提供舒适的座位安排",
            "准备必要的展示材料",
        ]
    }

    /// 拜访礼仪
    pub fn visiting(&self) -> Vec<&'static str> {
        vec![
            "提前预约拜访时间",
            "确认拜访对象和目的",
            "准时到达约定地点",
            "穿着得体整洁",
            "携带必要文件和名片",
            "遵循对方公司的规定",
            "进入时问候前台",
            "等待对方引导",
            "尊重对方时间安排",
            "拜访结束时表示感谢",
        ]
    }

    /// 商务宴请礼仪
    pub fn business_dining(&self) -> Vec<&'static str> {
        vec![
            "选择合适的餐厅和座位",
            "提前了解客人饮食偏好",
            "安排菜单兼顾各方需求",
            "座位安排遵循礼仪规则",
            "主人主动开场和买单",
            "避免过度饮酒",
            "交谈话题积极正面",
            "尊重各国饮食习惯",
            "不要在餐桌上处理商务",
            "结束时感谢客人光临",
        ]
    }

    /// 商务握手礼仪
    pub fn handshake(&self) -> Vec<&'static str> {
        vec![
            "站姿端正，面带微笑",
            "握手力度适中",
            "握手时间2-3秒",
            "眼神交流，表达诚意",
            "伸出右手（避免左手）",
            "上级或女士先伸手",
            "握手时自报姓名",
            "避免同时与多人握手",
            "手套应脱下（户外可保留）",
            "握手后保持适当距离",
        ]
    }

    /// 商务介绍礼仪
    pub fn introduction(&self) -> Vec<&'static str> {
        vec![
            "先介绍级别较低者给级别较高者",
            "先介绍主人给客人",
            "先介绍男士给女士",
            "先介绍年轻者给年长者",
            "清晰说明姓名和职位",
            "介绍后引导问候和握手",
            "避免过多介绍以免混乱",
            "提供简短背景信息",
        ]
    }

    /// 商务场合交谈礼仪
    pub fn conversation(&self) -> Vec<&'static str> {
        vec![
            "保持微笑和友好态度",
            "话题积极，避免争议",
            "注意倾听，适时回应",
            "不打断对方发言",
            "避免过度私人话题",
            "尊重对方观点",
            "避免涉及敏感话题（政治、宗教）",
            "使用恰当的幽默",
            "控制说话音量",
            "注意身体语言",
        ]
    }

    /// 文化差异注意事项
    pub fn cultural_notes(&self) -> Vec<&'static str> {
        vec![
            "日本：鞠躬比握手重要，名片交换仪式化",
            "中国：重视宴请，座次严格，主人买单",
            "美国：直接热情，时间观念强",
            "欧洲：正式但友好，重视午餐会",
            "中东：重视关系建立，男性主导",
            "印度：尊重等级，避免左手使用",
            "拉美：热情亲切，可能迟到",
            "东南亚：谦逊礼貌，避免直接冲突",
        ]
    }

    /// 商务旅行礼仪
    pub fn business_travel(&self) -> Vec<&'static str> {
        vec![
            "遵守当地习俗和法律",
            "了解基本礼仪和文化",
            "尊重宗教禁忌",
            "穿着符合当地要求",
            "学习基本问候语",
            "注意时差和作息",
            "避免敏感话题讨论",
            "保持专业形象",
        ]
    }
}

impl Rule for BusinessNetworkingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【商务社交礼仪】\n\n\
            客户接待礼仪：\n{}\n\n\
            拜访礼仪：\n{}\n\n\
            商务宴请礼仪：\n{}\n\n\
            商务握手礼仪：\n{}\n\n\
            商务介绍礼仪：\n{}\n\n\
            商务场合交谈礼仪：\n{}\n\n\
            文化差异注意事项：\n{}\n\n\
            商务旅行礼仪：\n{}",
            self.client_reception()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.visiting()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.business_dining()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.handshake()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.introduction()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.conversation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_notes()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.business_travel()
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
    fn test_business_networking_rules() {
        let rules = BusinessNetworkingRules::new();
        assert_eq!(rules.metadata().name, "商务社交礼仪");
        assert!(!rules.client_reception().is_empty());
        assert!(!rules.visiting().is_empty());
        assert!(!rules.business_dining().is_empty());
        assert!(!rules.handshake().is_empty());
        assert!(!rules.introduction().is_empty());
        assert!(!rules.conversation().is_empty());
        assert!(!rules.cultural_notes().is_empty());
        assert!(!rules.business_travel().is_empty());
    }

    #[test]
    fn test_business_networking_validation() {
        let rules = BusinessNetworkingRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_business_networking_explain() {
        let rules = BusinessNetworkingRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("客户接待礼仪"));
        assert!(explanation.contains("商务宴请礼仪"));
        assert!(explanation.contains("文化差异"));
    }
}
