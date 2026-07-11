//! 统计物理详细规则
//!
//! 统计物理学用统计方法研究大量粒子系统的宏观性质。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};

/// 统计物理详细规则集合
pub struct StatisticalPhysicsDetailedRules {
    metadata: RuleMetadata,
}

impl StatisticalPhysicsDetailedRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("统计物理详细规则", "统计物理学基本概念和定律")
                .with_origin("物理学")
                .with_tags(vec!["科学".into(), "物理".into(), "统计物理".into()]),
        }
    }

    /// 统计力学基础
    pub fn statistical_mechanics_basics(&self) -> Vec<&'static str> {
        vec![
            "统计力学定义: 用统计方法研究宏观系统",
            "微观态: 系统微观粒子分布的特定状态",
            "宏观态: 系统宏观可观测量的状态",
            "系综: 具有相同宏观性质的大量系统集合",
            "等概率原理: 平衡态各微观态概率相等",
            "玻尔兹曼熵公式: S = k ln W",
            "配分函数: Z = Σe^(-Ei/kT)",
            "自由能: F = -kT ln Z",
        ]
    }

    /// 系综理论
    pub fn ensemble_theory(&self) -> Vec<&'static str> {
        vec![
            "微正则系综: 能量固定的孤立系统",
            "正则系综: 温度固定与热源接触的系统",
            "巨正则系综: 温度和化学势固定的系统",
            "等压系综: 压力固定的系统",
            "系综平均: 对系综中所有系统取平均",
            "时间平均: 长时间观测值的平均",
            "各态历经: 时间平均等于系综平均",
            "系综等价: 大系统极限下各系综等价",
        ]
    }

    /// 玻尔兹曼分布
    pub fn boltzmann_distribution(&self) -> Vec<&'static str> {
        vec![
            "玻尔兹曼分布: 能量E态的概率P∝e^(-E/kT)",
            "玻尔兹曼因子: e^(-E/kT)决定态概率",
            "最概然分布: 熵最大的分布",
            "麦克斯韦速度分布: 分子速度的统计分布",
            "麦克斯韦-玻尔兹曼分布: 能量和速度联合分布",
            "平均能量: <E> = kT²(∂lnZ/∂T)",
            "能量涨落: (ΔE)² = kT²Cv",
            "配分函数与热力学量: 所有热力学量可从Z导出",
        ]
    }

    /// 费米-狄拉克统计
    pub fn fermi_dirac_statistics(&self) -> Vec<&'static str> {
        vec![
            "费米子: 服从泡利不相容原理的粒子",
            "费米-狄拉克分布: f(E) = 1/(e^(E-μ)/kT + 1)",
            "占据数: 每个态最多一个费米子",
            "费米能级: μ在T=0时的能量",
            "费米温度: TF = μ/k",
            "费米海: 费米能级以下所有态被占据",
            "电子气体: 金属中自由电子的费米统计",
            "泡利阻塞: 费米子不能占据已占据态",
        ]
    }

    /// 玻色-爱因斯坦统计
    pub fn bose_einstein_statistics(&self) -> Vec<&'static str> {
        vec![
            "玻色子: 不服从泡利不相容原理的粒子",
            "玻色-爱因斯坦分布: f(E) = 1/(e^(E-μ)/kT - 1)",
            "占据数: 每个态可有多个玻色子",
            "玻色凝聚: 低温时玻色子聚集到最低态",
            "临界温度: 发生玻色凝聚的温度",
            "凝聚体: 宏观数量粒子在同一态",
            "超流体:玻色凝聚导致的零粘性流动",
            "激光: 光子玻色凝聚的宏观表现",
        ]
    }

    /// 相变理论
    pub fn phase_transition_theory(&self) -> Vec<&'static str> {
        vec![
            "一级相变: 有潜热和体积突变",
            "二级相变: 无潜热有比热突变",
            "临界点: 相变发生的特定温度压力",
            "序参量: 描述有序程度的物理量",
            "对称性破缺: 相变时系统对称性降低",
            "临界指数: 描述临界点附近行为的指数",
            "普适性: 不同系统临界指数相同",
            "标度律: 临界指数之间的数学关系",
        ]
    }

    /// 朗道理论
    pub fn landau_theory(&self) -> Vec<&'static str> {
        vec![
            "朗道相变理论: 用序参量描述相变",
            "自由能展开: F = F₀ + aφ² + bφ⁴ + ...",
            "序参量演化: φ随温度变化",
            "对称性考虑: 自由能保持系统对称性",
            "临界温度附近: a∝(T-Tc)",
            "序参量跳跃: 二级相变序参量连续",
            "临界指数预测: 朗道理论给出临界指数",
            "涨落修正: 序参量涨落影响理论",
        ]
    }

    /// 重整化群理论
    pub fn renormalization_group(&self) -> Vec<&'static str> {
        vec![
            "重整化群: 研究系统不同尺度的理论",
            "标度变换: 改变系统尺度观察行为",
            "不动点: 标度变换下不变的状态",
            "相关长度: 系统关联的空间尺度",
            "普适类: 有相同不动点的系统",
            "临界指数计算: 重整化群精确计算",
            "标度不变性: 临界点附近尺度无关",
            "有效理论: 大尺度下有效的简化理论",
        ]
    }

    /// 应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "热力学计算",
            "相变研究",
            "材料设计",
            "量子统计",
            "等离子体理论",
            "天体物理",
            "凝聚态物理",
            "复杂系统研究",
        ]
    }
}

impl Default for StatisticalPhysicsDetailedRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for StatisticalPhysicsDetailedRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("statistical_physics_detailed")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "统计物理详细规则",
            &[
                ("统计力学基础", &self.statistical_mechanics_basics()),
                ("系综理论", &self.ensemble_theory()),
                ("玻尔兹曼分布", &self.boltzmann_distribution()),
                ("费米-狄拉克统计", &self.fermi_dirac_statistics()),
                ("玻色-爱因斯坦统计", &self.bose_einstein_statistics()),
                ("相变理论", &self.phase_transition_theory()),
                ("朗道理论", &self.landau_theory()),
                ("重整化群", &self.renormalization_group()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistical_physics_detailed_rules() {
        let rules = StatisticalPhysicsDetailedRules::new();
        assert_eq!(rules.metadata().name, "统计物理详细规则");
        assert!(!rules.statistical_mechanics_basics().is_empty());
        assert!(!rules.ensemble_theory().is_empty());
        assert!(!rules.boltzmann_distribution().is_empty());
        assert!(!rules.explain().is_empty());
    }
}