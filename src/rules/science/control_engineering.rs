//! 控制工程定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 控制工程定律集合
pub struct ControlEngineeringLaws {
    metadata: RuleMetadata,
}

impl ControlEngineeringLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("控制工程定律", "控制工程基本定律")
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

    /// 数字控制定律
    pub fn digital_control_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("采样定律", "香农采样", "采样频率大于信号最高频率两倍"),
            ("Z变换定律", "离散域", "离散系统Z变换分析"),
            ("数字PID定律", "数字PID", "数字PID控制算法"),
            ("量化误差定律", "量化精度", "数字量化引入误差"),
            ("数字滤波定律", "数字滤波", "数字滤波器设计规律"),
            ("离散状态空间定律", "离散状态", "离散状态空间模型"),
            ("数字重构定律", "信号重构", "数字信号重构恢复"),
            ("多采样定律", "多速率", "多采样率控制系统"),
        ]
    }

    /// 非线性控制定律
    pub fn nonlinear_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("描述函数定律", "谐波线性化", "非线性环节描述函数分析"),
            ("相平面定律", "相轨迹", "二阶非线性系统相平面"),
            ("李雅普诺夫定律", "稳定性", "非线性系统李雅普诺夫方法"),
            ("输入输出稳定性定律", "小增益", "非线性系统输入输出稳定性"),
            ("反馈线性化定律", "精确线性化", "非线性系统反馈线性化"),
            ("反步控制定律", "反步法", "非线性系统反步设计"),
            ("自适应非线性定律", "自适应", "非线性系统自适应控制"),
            ("无源性定律", "无源系统", "非线性系统无源性分析"),
        ]
    }

    /// 智能控制定律
    pub fn intelligent_control_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("专家控制定律", "知识库", "基于专家知识的控制"),
            ("遗传算法控制定律", "进化优化", "遗传算法优化控制参数"),
            ("粒子群控制定律", "群智能", "粒子群优化控制策略"),
            ("深度强化学习定律", "深度RL", "深度强化学习控制策略"),
            ("知识推理定律", "推理控制", "基于知识推理的控制"),
            ("人机交互控制定律", "人机协同", "人机交互智能控制"),
            ("自主控制定律", "自主决策", "无人系统自主控制"),
            ("多智能体控制定律", "协调控制", "多智能体协调控制"),
        ]
    }

    /// 反馈控制
    pub fn feedback_control(&self) -> Vec<&'static str> {
        vec![
            "PID控制: 比例积分微分三种控制作用的组合",
            "根轨迹法: 分析闭环极点随增益变化的轨迹",
            "频率响应: 系统对正弦输入的稳态响应",
            "伯德图: 用对数坐标绘制频率响应的幅值和相位",
            "奈奎斯特判据: 通过开环频率响应判断闭环稳定性",
            "相位裕度: 系统距离不稳定还有多大的裕量",
        ]
    }

    /// 现代控制理论
    pub fn modern_control(&self) -> Vec<&'static str> {
        vec![
            "状态空间法: 用一阶微分方程组描述系统",
            "能控性: 系统能否通过输入从任意状态到达任意状态",
            "能观性: 能否通过输出观测推断系统内部状态",
            "极点配置: 通过状态反馈将闭环极点移到期望位置",
            "观测器设计: 根据输入输出估计系统内部状态",
            "最优控制: 使某性能指标最优的控制策略",
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

    fn explain(&self) -> String {
        format!(
            "【控制工程定律】\n\n理论定律:\n{}\n\n方法定律:\n{}\n\n分析定律:\n{}\n\n数字控制定律:\n{}\n\n非线性控制定律:\n{}\n\n智能控制定律:\n{}\n",
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
                .join("\n"),
            self.digital_control_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.nonlinear_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.intelligent_control_laws().iter()
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
