//! 酒店住宿礼仪
//!
//! 酒店入住、安静卫生与退房的礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: HotelStayMannerRules,
    name: "酒店住宿礼仪",
    desc: "酒店入住、安静卫生与退房的礼仪",
    origin: "国际",
    tags: ["社交", "礼仪", "酒店", "住宿"]
}

impl HotelStayMannerRules {
    /// 入住有序
    pub fn checkin(&self) -> Vec<&'static str> {
        vec![
            "礼貌登记办理入住",
            "妥善保管房卡",
            "按约定入住",
            "贵重物品注意",
        ]
    }

    /// 安静卫生
    pub fn quiet(&self) -> Vec<&'static str> {
        vec![
            "控制音量不扰邻",
            "镜面保整洁",
            "不吸烟按规定",
            "垃圾放置得当",
        ]
    }

    /// 公共区域
    pub fn lobby(&self) -> Vec<&'static str> {
        vec!["大厅勿喧哗", "泳池餐厅守规", "礼让他人", "衣着得体"]
    }

    /// 退房结算
    pub fn checkout(&self) -> Vec<&'static str> {
        vec![
            "按时退房结账",
            "核对账单",
            "遗留物品及时处理",
            "感谢服务人员",
        ]
    }
}

impl Rule for HotelStayMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("hotel")
    }

    fn explain(&self) -> String {
        format!(
            "【酒店住宿礼仪】\n{}",
            [
                format!(
                    "入住有序：\\n{}",
                    self.checkin()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安静卫生：\\n{}",
                    self.quiet()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共区域：\\n{}",
                    self.lobby()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "退房结算：\\n{}",
                    self.checkout()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
            ]
            .join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_hotelstaymannerrules_basic() {
        let rules = HotelStayMannerRules::new();
        assert_eq!(rules.metadata().name, "酒店住宿礼仪");
        assert!(!rules.checkin().is_empty());
        assert!(!rules.quiet().is_empty());
        assert!(!rules.lobby().is_empty());
        assert!(!rules.checkout().is_empty());
    }

    #[test]
    fn test_hotelstaymannerrules_validation() {
        let rules = HotelStayMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("hotel"));
    }

    #[test]
    fn test_hotelstaymannerrules_explain() {
        let rules = HotelStayMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("入住有序"));
        assert!(e.contains("安静卫生"));
        assert!(e.contains("公共区域"));
    }
}
