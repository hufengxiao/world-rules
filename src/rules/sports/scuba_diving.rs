//! 水肺潜水规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 水肺潜水规则 (PADI标准)
pub struct ScubaDivingRules {
    metadata: RuleMetadata,
}

impl ScubaDivingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("水肺潜水规则", "PADI水肺潜水标准规则")
                .with_origin("国际")
                .with_tags(vec!["体育".into(), "水上".into(), "潜水".into()]),
        }
    }

    /// 认证等级
    pub fn certification_levels(&self) -> Vec<&'static str> {
        vec![
            "开放水域潜水员(OW): 18米深度限制",
            "进阶开放水域潜水员(AOW): 30米深度限制",
            "救援潜水员: 应急救援能力",
            "潜水长(Divemaster): 专业级别",
            "教练等级: 教学认证",
        ]
    }

    /// 潜水规则
    pub fn diving_rules(&self) -> Vec<&'static str> {
        vec![
            "潜伴制度: 必须与潜伴共同潜水",
            "安全停留: 5米3分钟安全停留",
            "减压限制: 遵守减压表或潜水电脑",
            "上升速度: 不超过每分钟18米",
            "气源管理: 保留50bar备用气",
        ]
    }

    /// 装备要求
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "气瓶: 压缩空气供给",
            "调节器: 一级头和二级头",
            "BCD: 浮力控制装置",
            "潜水服: 保暖和保护",
            "面镜脚蹼: 视力和推进",
        ]
    }

    /// 潜水类型
    pub fn diving_types(&self) -> Vec<&'static str> {
        vec![
            "休闲潜水: 观光和摄影",
            "夜潜: 夜间探索",
            "深潜: 30-40米深度",
            "沉船潜水: 历史探索",
            "漂流潜水: 随流漂流",
        ]
    }

    /// 安全程序
    pub fn safety_procedures(&self) -> Vec<&'static str> {
        vec![
            "潜水计划: 确定深度、时间、路线",
            "潜伴检查: BWRAF检查流程",
            "手势沟通: 水下交流信号",
            "应急程序: 气源共享、紧急上浮",
            "潜水日志: 记录潜水经历",
        ]
    }

    /// 手势信号
    pub fn hand_signals(&self) -> Vec<&'static str> {
        vec![
            "OK信号: 一切正常",
            "上升信号: 准备上浮",
            "下潜信号: 准备下潜",
            "气量不足: 提醒注意",
            "危险信号: 立即关注",
        ]
    }

    /// 深度限制
    pub fn depth_limits(&self) -> Vec<&'static str> {
        vec![
            "开放水域: 18米",
            "进阶潜水: 30米",
            "深潜专长: 40米",
            "减压潜水: 需要特殊训练",
            "休闲极限: 40米(免减压)",
        ]
    }

    /// 环境保护
    pub fn environmental_rules(&self) -> Vec<&'static str> {
        vec![
            "不触摸珊瑚: 保护海洋生物",
            "不收集生物: 禁止带走海洋生物",
            "保持浮力: 避免触碰海底",
            "垃圾带走: 不留任何垃圾",
            "尊重海洋生物: 保持安全距离",
        ]
    }
}

impl Default for ScubaDivingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ScubaDivingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("scuba_diving")
    }

    fn explain(&self) -> String {
        format!(
            "【水肺潜水规则】\n\n\
            认证等级:\n{}\n\n\
            潜水规则:\n{}\n\n\
            装备要求:\n{}\n\n\
            安全程序:\n{}",
            self.certification_levels()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.diving_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_procedures()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scuba_diving_rules() {
        let rules = ScubaDivingRules::new();
        assert_eq!(rules.metadata().name, "水肺潜水规则");
        assert!(!rules.explain().is_empty());
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }

    #[test]
    fn test_certification_levels() {
        let rules = ScubaDivingRules::new();
        let levels = rules.certification_levels();
        assert!(levels.iter().any(|l| l.contains("开放水域")));
        assert!(levels.iter().any(|l| l.contains("进阶")));
        assert!(levels.len() >= 5);
    }

    #[test]
    fn test_diving_rules() {
        let rules = ScubaDivingRules::new();
        let rules_list = rules.diving_rules();
        assert!(rules_list.iter().any(|r| r.contains("潜伴")));
        assert!(rules_list.iter().any(|r| r.contains("安全停留")));
    }

    #[test]
    fn test_equipment() {
        let rules = ScubaDivingRules::new();
        let equipment = rules.equipment();
        assert!(equipment.iter().any(|e| e.contains("气瓶")));
        assert!(equipment.iter().any(|e| e.contains("调节器")));
    }

    #[test]
    fn test_diving_types() {
        let rules = ScubaDivingRules::new();
        let types = rules.diving_types();
        assert!(types.iter().any(|t| t.contains("休闲潜水")));
        assert!(types.iter().any(|t| t.contains("深潜")));
    }

    #[test]
    fn test_safety_procedures() {
        let rules = ScubaDivingRules::new();
        let procedures = rules.safety_procedures();
        assert!(procedures.iter().any(|p| p.contains("潜水计划")));
        assert!(procedures.iter().any(|p| p.contains("BWRAF")));
    }

    #[test]
    fn test_hand_signals() {
        let rules = ScubaDivingRules::new();
        let signals = rules.hand_signals();
        assert!(signals.iter().any(|s| s.contains("OK")));
        assert!(signals.iter().any(|s| s.contains("上升")));
        assert!(signals.len() >= 5);
    }

    #[test]
    fn test_depth_limits() {
        let rules = ScubaDivingRules::new();
        let limits = rules.depth_limits();
        assert!(limits.iter().any(|l| l.contains("18米")));
        assert!(limits.iter().any(|l| l.contains("40米")));
    }

    #[test]
    fn test_environmental_rules() {
        let rules = ScubaDivingRules::new();
        let env_rules = rules.environmental_rules();
        assert!(env_rules.iter().any(|r| r.contains("珊瑚")));
        assert!(env_rules.iter().any(|r| r.contains("海洋生物")));
    }
}
