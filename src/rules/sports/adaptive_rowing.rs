//! 适应性划船规则
//!
//! 针对不同残疾类型的划船适应性规则，包括固定座位、手臂划船等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 适应性划船规则
pub struct AdaptiveRowingRules {
    metadata: RuleMetadata,
}

impl AdaptiveRowingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("适应性划船规则", "残疾人划船适应性规则")
                .with_origin("FISA/IPC")
                .with_tags(vec![
                    "体育".into(),
                    "划船".into(),
                    "残奥".into(),
                    "适应性".into(),
                ]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "PR1级: 手臂和肩膀功能（AS）",
            "PR1-M1x: 男子单人双桨固定座位",
            "PR1-W1x: 女子单人双桨固定座位",
            "PR2级: 躯干和手臂功能（TA）",
            "PR2-M1x: 男子单人双桨躯干功能",
            "PR2-W1x: 女子单人双桨躯干功能",
            "PR2-Mix2x: 混合双人双桨",
            "PR3级: 腿部、躯干和手臂功能（LTA）",
            "PR3-Mix4+: 混合四人单桨有舵手",
            "PR3-Mix2x: 混合双人双桨",
            "视力残疾: PR3级允许参赛",
        ]
    }

    /// 船艇适应性
    pub fn boat_adaptations(&self) -> Vec<&'static str> {
        vec![
            "PR1船艇: 固定座椅系统",
            "PR2船艇: 支撑性座椅",
            "PR3船艇: 标准赛艇允许改装",
            "座椅固定: 必须牢固固定",
            "脚踏板: 可改装或移除",
            "靠背支撑: PR1/PR2允许",
            "船体稳定: 可增加稳定器",
            "禁止: 动力辅助装置",
            "禁止: 电子控制系统",
        ]
    }

    /// 划桨适应性
    pub fn rowing_adaptations(&self) -> Vec<&'static str> {
        vec![
            "固定座位划桨: 仅使用手臂和肩膀",
            "支撑座位划桨: 允许躯干运动",
            "标准划桨: 全身协调划桨",
            "单手划桨: 允许固定桨位",
            "视力残疾: 允许舵手引导",
            "假肢固定: 允许假肢固定桨",
            "手套绑带: 允许使用握持辅助",
            "桨杆改装: 允许长度调整",
        ]
    }

    /// 比赛项目
    pub fn events(&self) -> Vec<&'static str> {
        vec![
            "PR1-M1x: 男子单人双桨（1000米）",
            "PR1-W1x: 女子单人双桨（1000米）",
            "PR2-M1x: 男子单人双桨（2000米）",
            "PR2-W1x: 女子单人双桨（2000米）",
            "PR2-Mix2x: 混合双人双桨（2000米）",
            "PR3-Mix2x: 混合双人双桨（2000米）",
            "PR3-Mix4+: 混合四人单桨有舵手（2000米）",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "救生衣: 所有运动员必须佩戴",
            "翻船程序: 赛前必须演练",
            "温度监控: 水温低于12°C限制",
            "医疗艇: 必须跟随救援艇",
            "分级卡: 必须随身携带",
            "赛前检查: 船艇和固定装置",
            "紧急信号: 必须了解紧急程序",
            "运动员声明: 健康状况声明",
        ]
    }

    /// 参赛资格
    pub fn eligibility(&self) -> Vec<&'static str> {
        vec![
            "必须通过FISA分级认证",
            "最低残疾标准",
            "国际赛艇联合会执照",
            "国家队注册要求",
            "达标成绩要求",
            "医疗证明: 健康状况证明",
            "年龄限制: 最低18岁",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "船艇规格违规",
            "座椅固定不牢",
            "使用禁止装备",
            "航道违规",
            "干扰其他船艇",
            "接受非法场外协助",
            "分级不符",
            "安全装置失效",
        ]
    }
}

impl Default for AdaptiveRowingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AdaptiveRowingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("adaptive_rowing")
    }

    fn explain(&self) -> String {
        format!(
            "【适应性划船规则】\n\n\
            运动分级:\n{}\n\n\
            船艇适应性:\n{}\n\n\
            划桨适应性:\n{}\n\n\
            比赛项目:\n{}\n\n\
            安全规则:\n{}",
            self.classification()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.boat_adaptations()
                .iter()
                .map(|b| format!("  • {}", b))
                .collect::<Vec<_>>()
                .join("\n"),
            self.rowing_adaptations()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.events()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_rules()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_rowing_rules_basic() {
        let rules = AdaptiveRowingRules::new();
        assert_eq!(rules.metadata().name, "适应性划船规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_adaptive_rowing_classification() {
        let rules = AdaptiveRowingRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("PR1")));
        assert!(classification.iter().any(|c| c.contains("PR2")));
        assert!(classification.iter().any(|c| c.contains("PR3")));
        assert!(classification.len() >= 8);
    }

    #[test]
    fn test_adaptive_rowing_boat() {
        let rules = AdaptiveRowingRules::new();
        let boat = rules.boat_adaptations();
        assert!(boat.iter().any(|b| b.contains("座椅")));
        assert!(boat.iter().any(|b| b.contains("禁止")));
        assert!(boat.len() >= 6);
    }

    #[test]
    fn test_adaptive_rowing_events() {
        let rules = AdaptiveRowingRules::new();
        let events = rules.events();
        assert!(events.iter().any(|e| e.contains("PR1")));
        assert!(events.iter().any(|e| e.contains("Mix")));
        assert!(events.len() >= 5);
    }

    #[test]
    fn test_adaptive_rowing_category() {
        let rules = AdaptiveRowingRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
