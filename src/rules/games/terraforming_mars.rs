//! 殖民火星
//!
//! 改造火星环境的卡牌引擎构筑策略桌游

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};
use crate::simple_rule;

simple_rule! {
    struct: TerraformingMarsRules,
    name: "殖民火星",
    desc: "改造火星环境的卡牌引擎构筑策略桌游",
    origin: "美国",
    tags: ["桌游", "策略", "引擎"]
}

impl TerraformingMarsRules {
    /// 开局布置
    pub fn setup(&self) -> Vec<&'static str> {
        vec![
            "玩家扮演不同企业",
            "各获起始资金与牌",
            "摆放火星地图板块",
            "设定氧气温度海洋指标",
        ]
    }

    /// 回合流程
    pub fn turn(&self) -> Vec<&'static str> {
        vec![
            "研究阶段购卡",
            "行动阶段执行行动",
            "生产阶段结算资源",
            "世代循环推进",
        ]
    }

    /// 改造指标
    pub fn terraforming(&self) -> Vec<&'static str> {
        vec![
            "提升氧气浓度",
            "升高星球温度",
            "放置海洋板块",
            "三项达标即完成改造",
        ]
    }

    /// 资源生产
    pub fn production(&self) -> Vec<&'static str> {
        vec![
            "资金钢铁钛植物能源",
            "生产值决定收益",
            "卡牌改变生产",
            "合理规划资源",
        ]
    }

    /// 胜负结束
    pub fn ending(&self) -> Vec<&'static str> {
        vec![
            "三项指标全满即终局",
            "统计改造分与卡牌分",
            "含里程碑与奖励分",
            "总分最高者获胜",
        ]
    }
}

impl Rule for TerraformingMarsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("terraforming_mars")
    }

    fn explain(&self) -> String {
        format!(
            "【殖民火星】\n{}",
            [
                format!(
                    "开局布置：\\n{}",
                    self.setup()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "回合流程：\\n{}",
                    self.turn()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "改造指标：\\n{}",
                    self.terraforming()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "资源生产：\\n{}",
                    self.production()
                        .iter()
                        .map(|s| format!("  • {}", s))
                        .collect::<Vec<_>>()
                        .join("\\n")
                ),
                format!(
                    "胜负结束：\\n{}",
                    self.ending()
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
    fn test_terraformingmarsrules_basic() {
        let rules = TerraformingMarsRules::new();
        assert_eq!(rules.metadata().name, "殖民火星");
        assert!(!rules.setup().is_empty());
        assert!(!rules.turn().is_empty());
        assert!(!rules.terraforming().is_empty());
        assert!(!rules.production().is_empty());
        assert!(!rules.ending().is_empty());
    }

    #[test]
    fn test_terraformingmarsrules_validation() {
        let rules = TerraformingMarsRules::new();
        assert!(rules
            .validate(&ValidateContext::Generic("test".to_string()))
            .is_ok());
        assert_eq!(rules.category(), RuleCategory::games("terraforming_mars"));
    }

    #[test]
    fn test_terraformingmarsrules_explain() {
        let rules = TerraformingMarsRules::new();
        let e = rules.explain();
        assert!(e.contains("开局布置"));
        assert!(e.contains("回合流程"));
        assert!(e.contains("改造指标"));
    }
}
