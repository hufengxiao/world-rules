//! 位置共享礼仪
//!
//! 分享实时位置与打卡的隐私与安全分寸

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: LocationSharingMannerRules,
    name: "位置共享礼仪",
    desc: "分享实时位置与打卡的隐私与安全分寸",
    origin: "网络通用",
    tags: ["线上", "隐私", "安全"]
}

impl LocationSharingMannerRules {
    /// 分享场景
    pub fn scenarios(&self) -> Vec<&'static str> {
        vec![
            "约见面时共享位置",
            "接送亲友时使用",
            "紧急情况求助",
            "团队活动便于集合",
        ]
    }

    /// 隐私保护
    pub fn privacy(&self) -> Vec<&'static str> {
        vec![
            "不随意公开实时位置",
            "谨慎对陌生人共享",
            "关闭长期后台定位",
            "及时结束位置共享",
        ]
    }

    /// 社交打卡
    pub fn checkin(&self) -> Vec<&'static str> {
        vec![
            "避免暴露家庭住址",
            "外出打卡勿实时暴露",
            "延后发布行程动态",
            "家中照片注意背景",
        ]
    }

    /// 安全提醒
    pub fn safety(&self) -> Vec<&'static str> {
        vec![
            "仅向信任者共享",
            "注意儿童位置隐私",
            "定期检查定位权限",
            "发现异常及时处置",
        ]
    }
}

impl Rule for LocationSharingMannerRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("location_sharing_manner")
    }

    fn explain(&self) -> String {
        format!(
            "【位置共享礼仪】\n{}",
            [
                format!(
                    "分享场景：\\n{}",
                    self.scenarios()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "隐私保护：\\n{}",
                    self.privacy()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "社交打卡：\\n{}",
                    self.checkin()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "安全提醒：\\n{}",
                    self.safety()
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
    fn test_locationsharingmannerrules_basic() {
        let rules = LocationSharingMannerRules::new();
        assert_eq!(rules.metadata().name, "位置共享礼仪");
        assert!(!rules.scenarios().is_empty());
        assert!(!rules.privacy().is_empty());
        assert!(!rules.checkin().is_empty());
        assert!(!rules.safety().is_empty());
    }

    #[test]
    fn test_locationsharingmannerrules_validation() {
        let rules = LocationSharingMannerRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(
            rules.category(),
            RuleCategory::social("location_sharing_manner")
        );
    }

    #[test]
    fn test_locationsharingmannerrules_explain() {
        let rules = LocationSharingMannerRules::new();
        let e = rules.explain();
        assert!(e.contains("分享场景"));
        assert!(e.contains("隐私保护"));
        assert!(e.contains("社交打卡"));
    }
}
