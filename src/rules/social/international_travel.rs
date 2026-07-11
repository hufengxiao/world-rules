//! 国际旅行礼仪
//!
//! 涵盖国际旅行的详细规范，包括出入境、海关、酒店住宿等礼仪。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: InternationalTravelRules,
    name: "国际旅行礼仪",
    desc: "国际旅行礼仪详细规范，包括出入境、海关、酒店住宿等礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "国际", "旅行"]
}

impl InternationalTravelRules {
    /// 护照签证礼仪
    pub fn passport_visa(&self) -> Vec<&'static str> {
        vec![
            "确保护照有效期超过六个月",
            "提前办理目的地国家签证",
            "护照照片与本人相符",
            "保留护照复印件备用",
            "签证申请材料真实完整",
            "诚实回答签证官提问",
            "了解签证有效期和停留天数",
            "不逾期停留",
        ]
    }

    /// 出入境礼仪
    pub fn border_control(&self) -> Vec<&'static str> {
        vec![
            "在边境检查时保持礼貌",
            "准备好护照和签证",
            "如实填写入境卡",
            "回答官员问题时诚实",
            "不携带违禁物品",
            "配合安检和检查",
            "不拍摄边境检查区域",
            "尊重边检人员的决定",
        ]
    }

    /// 海关礼仪
    pub fn customs(&self) -> Vec<&'static str> {
        vec![
            "如实申报携带物品",
            "了解免税额度和限制",
            "不携带违禁品入境",
            "保留购物凭证",
            "配合海关检查",
            "礼貌回答海关人员询问",
            "了解现金携带限制",
            "申报超过限额的货币",
        ]
    }

    /// 机场礼仪
    pub fn airport_etiquette(&self) -> Vec<&'static str> {
        vec![
            "提前到达机场办理手续",
            "遵守航空公司行李规定",
            "安检时配合工作人员",
            "在候机区保持安静",
            "登机时排队等候",
            "尊重机组人员",
            "飞机起降时关闭电子设备",
            "下飞机时有序离开",
        ]
    }

    /// 酒店住宿礼仪
    pub fn hotel_etiquette(&self) -> Vec<&'static str> {
        vec![
            "办理入住时出示证件",
            "核对预订信息",
            "了解酒店设施和服务",
            "保持房间整洁",
            "遵守酒店规定",
            "晚上保持安静",
            "退房时检查遗留物品",
            "对服务人员表示感谢",
        ]
    }

    /// 文化禁忌礼仪
    pub fn cultural_taboos(&self) -> Vec<&'static str> {
        vec![
            "了解目的地文化禁忌",
            "尊重当地宗教习俗",
            "着装符合当地规范",
            "不拍摄禁止拍照的区域",
            "不在宗教场所大声喧哗",
            "遵守当地饮食习惯",
            "尊重当地节假日和活动",
            "避免敏感话题讨论",
        ]
    }

    /// 紧急情况礼仪
    pub fn emergency(&self) -> Vec<&'static str> {
        vec![
            "记录紧急联系电话",
            "了解最近使馆位置",
            "购买旅行保险",
            "携带常用药品",
            "遇到问题及时求助",
            "保留重要文件复印件",
            "与家人保持联系",
            "遵守当地法律和规定",
        ]
    }

    /// 公共场所礼仪
    pub fn public_places(&self) -> Vec<&'static str> {
        vec![
            "遵守当地公共秩序",
            "排队时不插队",
            "保持公共场所清洁",
            "不大声喧哗",
            "尊重他人隐私",
            "遵守禁烟规定",
            "不随地吐痰",
            "爱护公共设施",
        ]
    }
}

impl Rule for InternationalTravelRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("international")
    }

    fn explain(&self) -> String {
        format!(
            "【国际旅行礼仪】\n\n\
            护照签证礼仪：\n{}\n\n\
            出入境礼仪：\n{}\n\n\
            海关礼仪：\n{}\n\n\
            机场礼仪：\n{}\n\n\
            酒店住宿礼仪：\n{}\n\n\
            文化禁忌礼仪：\n{}\n\n\
            紧急情况礼仪：\n{}\n\n\
            公共场所礼仪：\n{}",
            self.passport_visa()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.border_control()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.customs()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.airport_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.hotel_etiquette()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.cultural_taboos()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.emergency()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.public_places()
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
    fn test_international_travel_rules() {
        let rules = InternationalTravelRules::new();
        assert_eq!(rules.metadata().name, "国际旅行礼仪");
        assert!(!rules.passport_visa().is_empty());
        assert!(!rules.border_control().is_empty());
        assert!(!rules.customs().is_empty());
        assert!(!rules.airport_etiquette().is_empty());
        assert!(!rules.hotel_etiquette().is_empty());
        assert!(!rules.cultural_taboos().is_empty());
        assert!(!rules.emergency().is_empty());
        assert!(!rules.public_places().is_empty());
    }

    #[test]
    fn test_international_travel_validation() {
        let rules = InternationalTravelRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("international"));
    }

    #[test]
    fn test_international_travel_explain() {
        let rules = InternationalTravelRules::new();
        let explanation = rules.explain();
        assert!(explanation.contains("护照签证礼仪"));
        assert!(explanation.contains("出入境礼仪"));
        assert!(explanation.contains("海关礼仪"));
        assert!(explanation.contains("机场礼仪"));
    }
}
