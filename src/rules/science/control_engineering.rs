//! 控制工程定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 控制工程定律集合
pub struct ControlEngineeringLaws {
    metadata: RuleMetadata,
}

impl ControlEngineeringLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "控制工程定律",
                "控制工程基本定律"
            )
            .with_origin("工程")
            .with_tags(vec!["科学".into(), "工程".into(), "控制".into()]),
        }
    }

    /// 控制理论定律
    pub fn theory_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("反馈定律", "闭环控制", "反馈控制原理"),
            ("开环定律", "开环控制", "开环控制系统"),
            ("稳定性定律", "稳定条件", "系统稳定性分析"),
            ("响应定律", "响应特性", "系统响应特性"),
            ("传递函数定律", "数学模型", "传递函数模型"),
            ("状态空间定律", "状态方程", "状态空间模型"),
            ("频率响应定律", "频域分析", "频率特性分析"),
            ("时域定律", "时域分析", "时域响应分析"),
        ]
    }

    /// 控制方法定律
    pub fn method_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("PID控制定律", "PID算法", "PID控制原理"),
            ("模糊控制定律", "模糊逻辑", "模糊控制方法"),
            ("自适应控制定律", "自适应调整", "自适应控制"),
            ("最优控制定律", "最优策略", "最优控制理论"),
            ("鲁棒控制定律", "鲁棒性", "鲁棒控制设计"),
            ("预测控制定律", "预测策略", "预测控制方法"),
            ("神经网络控制定律", "神经网络", "神经网络控制"),
            ("滑模控制定律", "滑模方法", "滑模控制原理"),
        ]
    }

    /// 系统分析定律
    pub fn analysis_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("根轨迹定律", "根轨迹分析", "根轨迹方法"),
            ("奈奎斯特定律", "频域判据", "奈奎斯特判据"),
            ("波特定律", "波特图", "波特图分析"),
            ("李雅普诺夫定律", "稳定性判据", "李雅普诺夫稳定性"),
            ("可控性定律", "可控条件", "系统可控性"),
            ("可观性定律", "可观条件", "系统可观性"),
            ("灵敏度定律", "灵敏度分析", "系统灵敏度"),
        ]
    }

    /// 控制应用定律
    pub fn application_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("过程控制定律", "工业过程", "工业过程控制"),
            ("运动控制定律", "运动系统", "运动控制系统"),
            ("机器人控制定律", "机器人控制", "机器人控制"),
            ("飞行控制定律", "飞行器控制", "飞行控制系统"),
            ("车辆控制定律", "车辆系统", "车辆控制"),
            ("电力控制定律", "电力系统", "电力系统控制"),
        ]
    }

    /// 控制元件
    pub fn components(&self) -> Vec<&'static str> {
        vec![
            "传感器",
            "控制器",
            "执行器",
            "反馈元件",
            "放大器",
            "滤波器",
            "转换器",
            "调节器",
        ]
    }

    /// 控制系统类型
    pub fn system_types(&self) -> Vec<&'static str> {
        vec![
            "连续系统",
            "离散系统",
            "线性系统",
            "非线性系统",
            "时变系统",
            "时不变系统",
            "单变量系统",
            "多变量系统",
        ]
    }
}

impl Default for ControlEngineeringLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ControlEngineeringLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("control_engineering")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【控制工程定律】\n\n理论定律:\n{}\n\n方法定律:\n{}\n\n分析定律:\n{}\n",
            self.theory_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.method_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.analysis_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_engineering_laws() {
        let laws = ControlEngineeringLaws::new();
        assert!(!laws.theory_laws().is_empty());
        assert!(!laws.method_laws().is_empty());
    }
}