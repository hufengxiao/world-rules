//! 商务接待礼仪
//!
//! 涵盖商务接待的详细礼仪规范，包括预约、接待流程、陪同参观等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ReceptionEtiquetteRules,
    name: "商务接待礼仪",
    desc: "商务接待详细礼仪规范，包括预约、接待流程、陪同参观等",
    origin: "国际通用",
    tags: ["社交", "礼仪", "商务", "接待", "访客"]
}

impl ReceptionEtiquetteRules {
    /// 接待前准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "确认访客身份和来访目的",
            "了解访客数量和级别",
            "安排接待时间和会议室",
            "准备接待人员和分工",
            "通知相关部门和人员",
            "准备必要的文件和资料",
            "安排车辆和停车位",
            "准备茶水和点心",
            "检查会议室环境和设备",
            "安排翻译人员（必要时）",
        ]
    }

    /// 前台接待礼仪
    pub fn front_desk(&self) -> Vec<&'static str> {
        vec![
            "保持微笑和礼貌态度",
            "主动问候访客",
            "确认访客身份和预约",
            "引导访客登记",
            "通知相关人员迎接",
            "提供等候区域和茶水",
            "保持前台整洁",
            "处理访客咨询耐心",
            "避免让访客长时间等待",
            "提供清晰的指引",
        ]
    }

    /// 陪同参观礼仪
    pub fn accompanying(&self) -> Vec<&'static str> {
        vec![
            "走在访客稍前方引导",
            "适时介绍公司环境",
            "注意访客安全和舒适",
            "控制参观时间和路线",
            "介绍相关人员",
            "允许拍照（必要时确认）",
            "回答访客问题",
            "避免访问敏感区域",
            "尊重访客隐私需求",
            "结束时总结并感谢",
        ]
    }

    /// 会议接待礼仪
    pub fn meeting_reception(&self) -> Vec<&'static str> {
        vec![
            "提前布置会议室",
            "座位安排遵循礼仪",
            "提供饮用水和茶点",
            "准备投影和演示设备",
            "及时提供会议材料",
            "控制会议时间",
            "记录会议要点",
            "提供舒适的温度",
            "避免打断会议",
            "会议后整理会议室",
        ]
    }

    /// 餐饮接待礼仪
    pub fn dining_reception(&self) -> Vec<&'static str> {
        vec![
            "提前预订餐厅",
            "了解访客饮食偏好",
            "座位安排尊重礼仪",
            "菜单选择兼顾各方",
            "主人主动引导用餐",
            "注意用餐礼仪示范",
            "避免过度饮酒",
            "买单时低调处理",
            "感谢访客光临",
            "安排返程车辆",
        ]
    }

    /// 送客礼仪
    pub fn farewell(&self) -> Vec<&'static str> {
        vec![
            "感谢访客到来",
            "确认后续安排",
            "赠送纪念品（必要时）",
            "陪同到出口或车辆",
            "帮助开门或搬运",
            "挥手告别或握手",
            "目送访客离开",
            "发送感谢邮件",
            "记录接待总结",
            "跟进后续事项",
        ]
    }

    /// VIP接待礼仪
    pub fn vip_reception(&self) -> Vec<&'static str> {
        vec![
            "安排专人全程陪同",
            "提供VIP接待室",
            "准备高质量茶点和餐饮",
            "高层领导亲自接待",
            "提供专属停车位",
            "安排舒适的休息环境",
            "提供个性化服务",
            "准备礼品和纪念品",
            "确保隐私和安保",
            "高层领导陪同送别",
        ]
    }

    /// 文化差异注意
    pub fn cultural_considerations(&self) -> Vec<&'static str> {
        vec![
            "日本：高度仪式化，重视细节和等级",
            "中国：热情周到，宴请重要，座次严格",
            "美国：高效务实，时间观念强",
            "欧洲：正式友好，重视午餐安排",
            "中东：热情待客，茶饮文化，男性主导",
            "印度：尊重等级，避免左手，素食者多",
            "韩国：等级分明，双手递交，鞠躬致意",
            "东南亚：微笑友好，谦虚低调",
        ]
    }
}

impl Rule for ReceptionEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("business")
    }

    fn explain(&self) -> String {
        format!(
            "【商务接待礼仪】\n\n\
            接待前准备：\n{}\n\n\
            前台接待礼仪：\n{}\n\n\
            陪同参观礼仪：\n{}\n\n\
            会议接待礼仪：\n{}\n\n\
            餐饮接待礼仪：\n{}\n\n\
            送客礼仪：\n{}\n\n\
            VIP接待礼仪：\n{}\n\n\
            文化差异注意：\n{}",
            self.preparation()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.front_desk()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.accompanying()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.meeting_reception()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dining_reception()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.farewell()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.vip_reception()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_considerations()
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
    fn test_reception_etiquette_rules() {
        let rules = ReceptionEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "商务接待礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.front_desk().is_empty());
        assert!(!rules.accompanying().is_empty());
        assert!(!rules.meeting_reception().is_empty());
        assert!(!rules.dining_reception().is_empty());
        assert!(!rules.farewell().is_empty());
        assert!(!rules.vip_reception().is_empty());
        assert!(!rules.cultural_considerations().is_empty());
    }

    #[test]
    fn test_reception_etiquette_validation() {
        let rules = ReceptionEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("business"));
    }

    #[test]
    fn test_reception_etiquette_explain() {
        let rules = ReceptionEtiquetteRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("接待前准备"));
        assert!(explanation.contains("前台接待礼仪"));
        assert!(explanation.contains("VIP接待礼仪"));
    }
}
