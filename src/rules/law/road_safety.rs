//! 道路交通安全法规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 道路交通安全法规则
pub struct RoadSafetyRules {
    metadata: RuleMetadata,
}

impl RoadSafetyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "道路交通安全法",
                "中国道路交通安全基本规则"
            )
            .with_origin("中国")
            .with_tags(vec!["法律".into(), "交通".into(), "安全".into()]),
        }
    }

    /// 行人规则
    pub fn pedestrian_rules(&self) -> Vec<&'static str> {
        vec![
            "走人行道或靠路边行走",
            "过马路走人行横道",
            "遵守交通信号灯",
            "不得翻越道路隔离设施",
            "不得在道路上追逐打闹",
        ]
    }

    /// 非机动车规则
    pub fn non_motor_vehicle_rules(&self) -> Vec<&'static str> {
        vec![
            "自行车上路年龄: 12周岁以上",
            "电动自行车上路年龄: 16周岁以上",
            "在非机动车道内行驶",
            "不得逆向行驶",
            "载物宽度不得超出车把0.15米",
        ]
    }

    /// 机动车规则
    pub fn motor_vehicle_rules(&self) -> Vec<&'static str> {
        vec![
            "驾驶年龄: 18周岁以上",
            "必须取得驾驶证",
            "行驶证、车牌齐全",
            "前排乘员系安全带",
            "不得拨打接听电话",
        ]
    }

    /// 超速处罚
    pub fn speeding_penalties(&self) -> Vec<&'static str> {
        vec![
            "超速20%以下: 警告或罚款200元",
            "超速20%-50%: 罚款200元，记6分",
            "超速50%以上: 罚款200-2000元，记12分",
            "超速100%以上: 可吊销驾驶证",
        ]
    }

    /// 酒驾处罚
    pub fn drunk_driving_penalties(&self) -> Vec<&'static str> {
        vec![
            "酒驾(20-80mg/100ml): 暂扣驾照6个月，罚款1000-2000元",
            "醉驾(80mg/100ml以上): 吊销驾照，5年内不得重考",
            "醉驾营运车辆: 吊销驾照，10年内不得重考",
            "醉驾发生重大事故: 终身禁驾",
        ]
    }

    /// 交通事故处理
    pub fn accident_handling(&self) -> Vec<&'static str> {
        vec![
            "立即停车保护现场",
            "有人员伤亡先救人",
            "及时报警(122)",
            "轻微事故可协商处理",
            "保留现场证据",
        ]
    }

    /// 紧急电话
    pub fn emergency_numbers(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("交通事故报警", "122"),
            ("医疗急救", "120"),
            ("火警", "119"),
            ("报警", "110"),
        ]
    }
}

impl Default for RoadSafetyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for RoadSafetyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::law("road_safety")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        let numbers = self.emergency_numbers();
        format!(
            "【道路交通安全法】\n\n\
            行人规则:\n{}\n\n\
            非机动车规则:\n{}\n\n\
            机动车规则:\n{}\n\n\
            超速处罚:\n{}\n\n\
            酒驾处罚:\n{}\n\n\
            紧急电话:\n{}\n",
            self.pedestrian_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.non_motor_vehicle_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.motor_vehicle_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.speeding_penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.drunk_driving_penalties().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            numbers.iter().map(|(n, t)| format!("  • {}: {}", n, t)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_road_safety_rules() {
        let rules = RoadSafetyRules::new();
        assert!(!rules.pedestrian_rules().is_empty());
    }
}