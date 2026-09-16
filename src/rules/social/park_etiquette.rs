//! 公园公共礼仪
//!
//! 公园、绿地等公共休闲空间的共享、环保与礼仪规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ParkEtiquetteRules,
    name: "公园公共礼仪",
    desc: "公园、绿地等公共休闲空间的共享、环保与礼仪规范",
    origin: "公共",
    tags: ["社交", "礼仪", "公园", "绿地", "环保"]
}

impl ParkEtiquetteRules {
    /// 共享空间
    pub fn share(&self) -> Vec<&'static str> {
        vec![
            "不长时间独占整张长凳",
            "运动器材轮换使用不霸占",
            "放风筝等避开他人与电线",
            "照顾老弱不占开阔步道",
        ]
    }

    /// 噪音与宠物
    pub fn noise(&self) -> Vec<&'static str> {
        vec![
            "控制外放音响的音量",
            "宠物牵绳并及时清理粪便",
            "儿童嬉闹看护好不撞人",
            "广场舞等音量不影响他人",
        ]
    }

    /// 环境卫生
    pub fn cleanliness(&self) -> Vec<&'static str> {
        vec![
            "垃圾按分类投放桶内",
            "不践踏花坛与禁入草坪",
            "不在非吸烟区吸烟",
            "爱护花草树木不折枝",
        ]
    }

    /// 公共设施
    pub fn facilities(&self) -> Vec<&'static str> {
        vec![
            "洗手间接序使用",
            "游乐设施排队依序",
            "不损坏雕塑喷泉等设施",
            "用后归还原位的公共物品",
        ]
    }
}

impl Rule for ParkEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("park")
    }

    fn explain(&self) -> String {
        format!(
            "【公园公共礼仪】\n{}",
            [
                format!(
                    "共享空间：\\n{}",
                    self.share()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "噪音与宠物：\\n{}",
                    self.noise()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "环境卫生：\\n{}",
                    self.cleanliness()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "公共设施：\\n{}",
                    self.facilities()
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
    fn test_parketiquetterules_basic() {
        let rules = ParkEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "公园公共礼仪");
        assert!(!rules.share().is_empty());
        assert!(!rules.noise().is_empty());
        assert!(!rules.cleanliness().is_empty());
        assert!(!rules.facilities().is_empty());
    }

    #[test]
    fn test_parketiquetterules_validation() {
        let rules = ParkEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("park"));
    }

    #[test]
    fn test_parketiquetterules_explain() {
        let rules = ParkEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("共享空间"));
        assert!(e.contains("噪音与宠物"));
        assert!(e.contains("环境卫生"));
    }
}
