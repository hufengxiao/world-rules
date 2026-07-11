//! 适应性游泳规则
//!
//! 针对不同残疾类型的游泳适应性规则，涵盖分级、起跳方式、转身等。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 适应性游泳规则
pub struct AdaptiveSwimmingRules {
    metadata: RuleMetadata,
}

impl AdaptiveSwimmingRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("适应性游泳规则", "残疾人游泳适应性规则")
                .with_origin("IPC/WPS")
                .with_tags(vec![
                    "体育".into(),
                    "游泳".into(),
                    "残奥".into(),
                    "适应性".into(),
                ]),
        }
    }

    /// 运动分级
    pub fn classification(&self) -> Vec<&'static str> {
        vec![
            "S级: 自由泳/仰泳/蝶泳",
            "S1-S10: 肢体残疾程度递减",
            "SB级: 蛙泳",
            "SB1-SB9: 肢体残疾程度递减",
            "SM级: 个人混合泳",
            "SM1-SM10: 肢体残疾程度递减",
            "S11-S13: 视力残疾（S11最重）",
            "S14: 智力残疾",
        ]
    }

    /// 起跳适应性
    pub fn starting_adaptations(&self) -> Vec<&'static str> {
        vec![
            "水中出发: 站立困难的运动员",
            "平台出发: 单腿起跳允许",
            "辅助出发: 使用毛巾/绳索",
            "信号适应: 视力残疾使用敲击信号",
            "起跳器改装: 截肢运动员假肢固定",
            "教练辅助: 智力残疾运动员允许",
        ]
    }

    /// 转身规则
    pub fn turn_rules(&self) -> Vec<&'static str> {
        vec![
            "视力残疾: 教练敲击提醒",
            "截肢运动员: 单手触壁允许",
            "麻痹运动员: 身体任何部位触壁即可",
            "转身时间: 不做特殊限制",
            "禁止: 推蹬池壁辅助",
            "允许: 滑行调整姿势",
        ]
    }

    /// 装备适应性
    pub fn equipment_adaptations(&self) -> Vec<&'static str> {
        vec![
            "假肢: 比赛中必须取下",
            "义眼: 可以佩戴",
            "助听器: 允许使用",
            "泳镜: 视力残疾可使用特殊泳镜",
            "泳帽: 必须佩戴，标明分级",
            "禁止: 浮力辅助设备",
            "禁止: 电子速度辅助",
        ]
    }

    /// 接力规则
    pub fn relay_rules(&self) -> Vec<&'static str> {
        vec![
            "4×100米自由泳接力",
            "4×100米混合泳接力",
            "总分限制: 34分（S1-10级）",
            "至少两名S级运动员",
            "视力残疾接力: 至少2名S11-13级",
            "混合接力: 男女混合编队允许",
            "交接棒: 水中接棒规则",
        ]
    }

    /// 犯规规则
    pub fn fouls(&self) -> Vec<&'static str> {
        vec![
            "非法辅助出发",
            "假肢未取下",
            "转身违规",
            "使用禁止装备",
            "干扰其他运动员",
            "分级不符",
            "接受场外协助",
        ]
    }
}

impl Default for AdaptiveSwimmingRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for AdaptiveSwimmingRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("adaptive_swimming")
    }

    fn explain(&self) -> String {
        format!(
            "【适应性游泳规则】\n\n\
            运动分级:\n{}\n\n\
            起跳适应性:\n{}\n\n\
            转身规则:\n{}\n\n\
            装备适应性:\n{}\n\n\
            接力规则:\n{}",
            self.classification()
                .iter()
                .map(|c| format!("  • {}", c))
                .collect::<Vec<_>>()
                .join("\n"),
            self.starting_adaptations()
                .iter()
                .map(|s| format!("  • {}", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.turn_rules()
                .iter()
                .map(|t| format!("  • {}", t))
                .collect::<Vec<_>>()
                .join("\n"),
            self.equipment_adaptations()
                .iter()
                .map(|e| format!("  • {}", e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.relay_rules()
                .iter()
                .map(|r| format!("  • {}", r))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_swimming_rules_basic() {
        let rules = AdaptiveSwimmingRules::new();
        assert_eq!(rules.metadata().name, "适应性游泳规则");
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_adaptive_swimming_classification() {
        let rules = AdaptiveSwimmingRules::new();
        let classification = rules.classification();
        assert!(classification.iter().any(|c| c.contains("S级")));
        assert!(classification.iter().any(|c| c.contains("视力残疾")));
        assert!(classification.len() >= 6);
    }

    #[test]
    fn test_adaptive_swimming_starting() {
        let rules = AdaptiveSwimmingRules::new();
        let starting = rules.starting_adaptations();
        assert!(starting.iter().any(|s| s.contains("水中出发")));
        assert!(starting.iter().any(|s| s.contains("视力残疾")));
        assert!(starting.len() >= 4);
    }

    #[test]
    fn test_adaptive_swimming_equipment() {
        let rules = AdaptiveSwimmingRules::new();
        let equipment = rules.equipment_adaptations();
        assert!(equipment.iter().any(|e| e.contains("假肢")));
        assert!(equipment.iter().any(|e| e.contains("禁止")));
        assert!(equipment.len() >= 5);
    }

    #[test]
    fn test_adaptive_swimming_category() {
        let rules = AdaptiveSwimmingRules::new();
        assert!(matches!(rules.category(), RuleCategory::Sports(_)));
    }
}
