//! 电梯礼仪

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 电梯礼仪规则
pub struct ElevatorEtiquette {
    metadata: RuleMetadata,
}

impl ElevatorEtiquette {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "电梯礼仪",
                "乘坐电梯礼仪规范"
            )
            .with_origin("通用")
            .with_tags(vec!["社交".into(), "电梯".into()]),
        }
    }

    /// 乘梯规则
    pub fn boarding_rules(&self) -> Vec<&'static str> {
        vec![
            "先下后上",
            "让老人、小孩、孕妇优先",
            "不拥挤、不推搡",
            "站在电梯门两侧等候",
        ]
    }

    /// 站立位置
    pub fn standing_positions(&self) -> Vec<&'static str> {
        vec![
            "先进入者往里站",
            "后进入者靠门站",
            "高层者站里面",
            "低层者靠门站",
        ]
    }

    /// 操作礼仪
    pub fn operation_etiquette(&self) -> Vec<&'static str> {
        vec![
            "主动按楼层按钮",
            "帮他人按住开门键",
            "不乱按按钮",
            "到达时让他人先出",
        ]
    }

    /// 禁止行为
    pub fn prohibited_behaviors(&self) -> Vec<&'static str> {
        vec![
            "不要在电梯内大声喧哗",
            "不要在电梯内吃东西",
            "不要强行挤入超载电梯",
            "不要让小孩在电梯内玩耍",
            "不要长时间挡住电梯门",
        ]
    }

    /// 特殊情况
    pub fn special_situations(&self) -> Vec<&'static str> {
        vec![
            "电梯故障: 按紧急按钮求助",
            "火灾: 禁止乘坐电梯",
            "搬运物品: 注意不要超载",
            "带宠物: 需要装入宠物包",
        ]
    }
}

impl Default for ElevatorEtiquette {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ElevatorEtiquette {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::social("elevator")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【电梯礼仪】\n\n\
            乘梯规则:\n{}\n\n\
            站立位置:\n{}\n\n\
            操作礼仪:\n{}\n\n\
            禁止行为:\n{}\n",
            self.boarding_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.standing_positions().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.operation_etiquette().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.prohibited_behaviors().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elevator_etiquette() {
        let etiquette = ElevatorEtiquette::new();
        assert!(etiquette.boarding_rules().contains(&"先下后上"));
    }
}