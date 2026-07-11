//! 商务陪同礼仪
//!
//! 涵盖商务陪同的礼仪规范，包括陪同参观、陪同用餐、陪同出行等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: AccompanyEtiquetteRules,
    name: "商务陪同礼仪",
    desc: "商务陪同礼仪规范，包括陪同参观、陪同用餐、陪同出行等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "陪同", "参观"]
}

impl AccompanyEtiquetteRules {
    /// 陪同基本原则
    pub fn basic_principles(&self) -> Vec<&'static str> {
        vec![
            "尊重访客需求和意愿",
            "保持专业和友好态度",
            "主动但不强迫",
            "注意访客舒适度",
            "提供必要信息和帮助",
            "保持适当交谈距离",
            "避免过度打扰",
            "随时关注访客状态",
        ]
    }

    /// 陪同参观礼仪
    pub fn tour_guide(&self) -> Vec<&'static str> {
        vec![
            "提前规划参观路线",
            "走在访客左前方半步",
            "适时介绍环境和设施",
            "控制参观节奏和时间",
            "注意访客安全和舒适",
            "介绍相关工作人员",
            "允许拍照（必要时确认）",
            "回答访客提问耐心",
            "避免进入敏感区域",
            "参观结束总结感谢",
        ]
    }

    /// 陪同用餐礼仪
    pub fn dining_accompany(&self) -> Vec<&'static str> {
        vec![
            "提前了解访客饮食偏好",
            "引导座位安排",
            "介绍菜品和特色",
            "关注访客用餐需求",
            "适时提供茶水和服务",
            "避免强迫饮酒",
            "聊天话题轻松适宜",
            "注意用餐速度同步",
            "协助处理用餐问题",
            "用餐结束安排后续",
        ]
    }

    /// 陪同出行礼仪
    pub fn travel_accompany(&self) -> Vec<&'static str> {
        vec![
            "提前安排交通路线",
            "确认访客出行偏好",
            "车辆准备整洁舒适",
            "开车平稳安全",
            "介绍沿途景观",
            "避免拥堵路线",
            "预留充足时间",
            "提供天气信息",
            "确认返程安排",
            "安全送达目的地",
        ]
    }

    /// 陪同住宿礼仪
    pub fn accommodation(&self) -> Vec<&'static str> {
        vec![
            "提前预订合适酒店",
            "确认房间类型和要求",
            "协助办理入住手续",
            "介绍酒店设施和周边",
            "确认访客作息安排",
            "提供必要联系方式",
            "关注访客住宿体验",
            "协助解决住宿问题",
            "提前安排退房时间",
            "协助行李搬运",
        ]
    }

    /// 陪同购物礼仪
    pub fn shopping_accompany(&self) -> Vec<&'static str> {
        vec![
            "了解访客购物需求",
            "推荐合适的购物地点",
            "提供翻译协助（必要时）",
            "避免过度推销",
            "尊重访客购买决定",
            "协助处理支付方式",
            "帮助携带物品",
            "注意购物时间控制",
            "提供退换货信息",
            "确认购物满意度",
        ]
    }

    /// 陪同注意事项
    pub fn important_notes(&self) -> Vec<&'static str> {
        vec![
            "避免私人行程干扰",
            "保护访客隐私",
            "避免过度热情或冷淡",
            "尊重访客文化差异",
            "准备应急方案",
            "保持通讯畅通",
            "及时汇报重要情况",
            "记录访客偏好和反馈",
        ]
    }

    /// 文化差异处理
    pub fn cultural_handling(&self) -> Vec<&'static str> {
        vec![
            "了解访客文化背景",
            "尊重宗教和习俗",
            "避免禁忌话题和行为",
            "提供符合习惯的饮食",
            "注意性别互动差异",
            "适应访客沟通风格",
            "提供必要的翻译服务",
            "学习基本问候语言",
        ]
    }
}

impl Rule for AccompanyEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【商务陪同礼仪】\n\n\
            陪同基本原则：\n{}\n\n\
            陪同参观礼仪：\n{}\n\n\
            陪同用餐礼仪：\n{}\n\n\
            陪同出行礼仪：\n{}\n\n\
            陪同住宿礼仪：\n{}\n\n\
            陪同购物礼仪：\n{}\n\n\
            陪同注意事项：\n{}\n\n\
            文化差异处理：\n{}",
            self.basic_principles()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.tour_guide()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dining_accompany()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.travel_accompany()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.accommodation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.shopping_accompany()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.important_notes()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_handling()
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
    fn test_accompany_etiquette_rules() {
        let rules = AccompanyEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "商务陪同礼仪");
        assert!(!rules.basic_principles().is_empty());
        assert!(!rules.tour_guide().is_empty());
        assert!(!rules.dining_accompany().is_empty());
        assert!(!rules.travel_accompany().is_empty());
        assert!(!rules.accommodation().is_empty());
        assert!(!rules.shopping_accompany().is_empty());
        assert!(!rules.important_notes().is_empty());
        assert!(!rules.cultural_handling().is_empty());
    }

    #[test]
    fn test_accompany_etiquette_validation() {
        let rules = AccompanyEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_accompany_etiquette_explain() {
        let rules = AccompanyEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("陪同基本原则"));
        assert!(explanation.contains("陪同参观礼仪"));
        assert!(explanation.contains("文化差异处理"));
    }
}
