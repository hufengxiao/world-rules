//! 一级方程式赛车规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 一级方程式赛车规则
pub struct F1Rules {
    metadata: RuleMetadata,
}

impl F1Rules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "一级方程式赛车规则",
                "F1赛车比赛基本规则"
            )
            .with_origin("欧洲")
            .with_tags(vec!["体育".into(), "汽车".into()]),
        }
    }

    /// 比赛周末安排
    pub fn race_weekend(&self) -> Vec<&'static str> {
        vec![
            "周五: 练习赛",
            "周六: 练习赛和排位赛",
            "周日: 正赛",
            "正赛距离约305公里",
            "时间限制2小时",
        ]
    }

    /// 排位赛规则
    pub fn qualifying_rules(&self) -> Vec<&'static str> {
        vec![
            "Q1: 18分钟淘汰5人",
            "Q2: 15分钟再淘汰5人",
            "Q3: 12分钟决定前10名",
            "杆位: 第一名发车位置",
            "罚退规则",
        ]
    }

    /// 正赛规则
    pub fn race_rules(&self) -> Vec<&'static str> {
        vec![
            "静态发车",
            "发车灯控制",
            "超车规则",
            "安全车",
            "红旗中断",
        ]
    }

    /// 积分系统
    pub fn points_system(&self) -> Vec<&'static str> {
        vec![
            "第一名: 25分",
            "第二名: 18分",
            "第三名: 15分",
            "第四名: 12分",
            "第五名: 10分",
            "最快圈速: 1分(前10名内)",
        ]
    }

    /// 车辆规格
    pub fn car_specifications(&self) -> Vec<&'static str> {
        vec![
            "混合动力系统",
            "1.6升V6涡轮增压",
            "电池能量回收",
            "重量限制",
            "空气动力学规则",
        ]
    }

    /// 轮胎规则
    pub fn tire_rules(&self) -> Vec<&'static str> {
        vec![
            "三种干地轮胎配方",
            "每场比赛使用两种",
            "比赛中必须使用两种",
            "轮胎数量限制",
            "湿地轮胎选择",
        ]
    }

    /// 进站规则
    pub fn pit_stop_rules(&self) -> Vec<&'static str> {
        vec![
            "换胎时间约2-3秒",
            "进站次数不限",
            "安全限制",
            "维修区限速",
            "释放安全检查",
        ]
    }

    /// 安全规则
    pub fn safety_rules(&self) -> Vec<&'static str> {
        vec![
            "Halo保护装置",
            "防火服",
            "头盔标准",
            "医疗车和救护车",
            "赛道安全设施",
        ]
    }
}

impl Default for F1Rules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for F1Rules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::sports("f1")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【一级方程式赛车规则】\n\n\
            比赛周末安排:\n{}\n\n\
            积分系统:\n{}\n\n\
            车辆规格:\n{}\n\n\
            安全规则:\n{}\n",
            self.race_weekend().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.points_system().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.car_specifications().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n"),
            self.safety_rules().iter().map(|r| format!("  • {}", r)).collect::<Vec<_>>().join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f1_rules() {
        let rules = F1Rules::new();
        assert!(!rules.race_weekend().is_empty());
    }
}