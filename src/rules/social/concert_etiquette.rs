//! 音乐会与演出礼仪
//!
//! 音乐会、演唱会、戏剧等现场表演的观演行为规范

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: ConcertEtiquetteRules,
    name: "音乐会与演出礼仪",
    desc: "音乐会、演唱会、戏剧等现场表演的观演行为规范",
    origin: "国际",
    tags: ["社交", "礼仪", "音乐会", "演出", "观演"]
}

impl ConcertEtiquetteRules {
    /// 观演准备
    pub fn preparation(&self) -> Vec<&'static str> {
        vec![
            "提前购票确认演出时间与场馆",
            "迟到观众通常需在曲目间隙入场",
            "了解演出是否允许摄影摄像",
            "交响音乐会建议着正装或商务休闲",
            "将手机调至静音或关机",
            "演出前提前如厕减少中途离席",
            "携带儿童前确认是否适合年龄",
            "香味不宜过浓以免影响他人",
        ]
    }

    /// 入场与就座
    pub fn seating(&self) -> Vec<&'static str> {
        vec![
            "按票面座位号有序就座",
            "请他人让身时轻声致谢",
            "大衣挎包遵循场馆存放指引",
            "入场后尽快就座减少走动",
            "经过他人座位时背向已坐观众",
            "尊长或嘉宾优先入座",
            "临时换座先征得相邻观众同意",
        ]
    }

    /// 演出进行中
    pub fn during_performance(&self) -> Vec<&'static str> {
        vec![
            "保持安静，关闭手机提示音",
            "不与他人交谈评论剧情",
            "未经许可不拍照录像",
            "不在场内饮食",
            "不频繁翻动节目单制造响动",
            "尊重演员不给倒彩",
            "带儿童观演应能保持安静",
            "咳嗽喷嚏用手肘遮挡降低声响",
        ]
    }

    /// 鼓掌与互动
    pub fn applause(&self) -> Vec<&'static str> {
        vec![
            "交响乐在多乐章演完后鼓掌",
            "独奏乐感可在独奏结束处鼓掌",
            "乐章之间避免零散掌声",
            "主讲人开场与退场时鼓掌",
            "谢幕时应起立鼓掌表达敬意",
            "演出完全退场后再离席",
            "用口哨或跺脚过激表达不合适",
            "安可演奏被唤起后重新安静",
        ]
    }

    /// 退场
    pub fn exit(&self) -> Vec<&'static str> {
        vec![
            "结束有序离场不拥挤抢先",
            "尊重演员谢幕不急于离开",
            "带走随身垃圾遵守卫生",
            "听从工作人员指引从指定出口",
            "不在通道或门口滞留聊天",
        ]
    }

    /// 常见误区
    pub fn common_missteps(&self) -> Vec<&'static str> {
        vec![
            "演出中高声谈笑被视为无礼",
            "擅自拍摄会被场馆制止",
            "带孩子选择适合剧目",
            "确需离场应待曲目或幕间",
        ]
    }
}

impl Rule for ConcertEtiquetteRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("concert")
    }

    fn explain(&self) -> String {
        let parts = vec![
            format!(
                "观演准备：\\n{}",
                self.preparation()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "入场与就座：\\n{}",
                self.seating()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "演出进行中：\\n{}",
                self.during_performance()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "鼓掌与互动：\\n{}",
                self.applause()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "退场：\\n{}",
                self.exit()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
            format!(
                "常见误区：\\n{}",
                self.common_missteps()
                    .iter()
                    .map(|s| format!("  • {}", s))
                    .collect::<Vec<_>>()
                    .join("\\n")
            ),
        ];
        format!("【音乐会与演出礼仪】\n{}", parts.join("\n\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::core::ValidateContext;

    #[test]
    fn test_concertetiquetterules_basic() {
        let rules = ConcertEtiquetteRules::new();
        assert_eq!(rules.metadata().name, "音乐会与演出礼仪");
        assert!(!rules.preparation().is_empty());
        assert!(!rules.seating().is_empty());
        assert!(!rules.during_performance().is_empty());
        assert!(!rules.applause().is_empty());
        assert!(!rules.exit().is_empty());
        assert!(!rules.common_missteps().is_empty());
    }

    #[test]
    fn test_concertetiquetterules_validation() {
        let rules = ConcertEtiquetteRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::social("concert"));
    }

    #[test]
    fn test_concertetiquetterules_explain() {
        let rules = ConcertEtiquetteRules::new();
        let e = rules.explain();
        assert!(e.contains("观演准备"));
        assert!(e.contains("入场与就座"));
        assert!(e.contains("演出进行中"));
    }
}
