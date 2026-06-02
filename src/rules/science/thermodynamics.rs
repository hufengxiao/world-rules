//! 热力学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 热力学定律集合
pub struct ThermodynamicsLaws {
    metadata: RuleMetadata,
}

impl ThermodynamicsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "热力学定律",
                "热力学基本定律"
            )
            .with_origin("物理学")
            .with_tags(vec!["科学".into(), "物理".into(), "热力学".into()]),
        }
    }

    /// 热力学定律
    pub fn all_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("热力学第零定律", "热平衡传递", "若A与B热平衡，B与C热平衡，则A与C热平衡"),
            ("热力学第一定律", "ΔU = Q - W", "能量守恒定律在热力学中的应用"),
            ("热力学第二定律", "ΔS ≥ 0", "孤立系统熵永不减少"),
            ("热力学第三定律", "T→0时 S→常数", "绝对零度不可达到"),
            ("卡诺定理", "η ≤ 1-Tc/Th", "热机效率上限"),
            ("克劳修斯不等式", "循环过程dS≥dQ/T", "任意循环过程的熵变"),
            ("吉布斯自由能", "G = H - TS", "恒温恒压系统的自发判据"),
            ("亥姆霍兹自由能", "A = U - TS", "恒温恒容系统的自发判据"),
            ("理想气体定律", "PV = nRT", "理想气体状态方程"),
            ("范德瓦尔斯方程", "(P+a/V²)(V-b)=RT", "实际气体状态方程"),
            ("热传导定律", "Q = -kA(dT/dx)", "傅里叶热传导定律"),
            ("热对流定律", "Q = hA(Ts-Tf)", "牛顿冷却定律"),
        ]
    }

    /// 热力学过程
    pub fn processes(&self) -> Vec<&'static str> {
        vec![
            "等温过程",
            "等压过程",
            "等容过程",
            "绝热过程",
            "循环过程",
            "卡诺循环",
            "奥托循环",
            "狄塞尔循环",
            "朗肯循环",
            "可逆过程",
            "不可逆过程",
        ]
    }

    /// 热力学函数
    pub fn functions(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("内能 U", "系统内部总能量"),
            ("焓 H", "H = U + PV"),
            ("熵 S", "系统无序度度量"),
            ("自由能 G", "Gibbs自由能"),
            ("自由能 A", "Helmholtz自由能"),
            ("热容 C", "C = dQ/dT"),
            ("比热容 c", "单位质量热容"),
            ("潜热 L", "相变所需热量"),
        ]
    }

    /// 热力学常数
    pub fn constants(&self) -> Vec<(&'static str, f64, &'static str)> {
        vec![
            ("理想气体常数 R", 8.314, "J/(mol·K)"),
            ("玻尔兹曼常数 k", 1.381e-23, "J/K"),
            ("阿伏伽德罗常数 NA", 6.022e23, "mol⁻¹"),
            ("标准大气压", 101325.0, "Pa"),
            ("标准温度", 273.15, "K"),
        ]
    }

    /// 统计力学定律
    pub fn statistical_mechanics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("麦克斯韦-玻尔兹曼分布定律", "f(v)∝exp(-mv²/2kT)", "理想气体分子速度分布"),
            ("玻尔兹曼熵定律", "S = k ln Ω", "熵与微观状态数关系"),
            ("能均分定律", "E = n·½kT", "每个自由度平均能量"),
            ("费米-狄拉克分布定律", "f(E)=1/(exp((E-μ)/kT)+1)", "费米子统计分布"),
            ("玻色-爱因斯坦分布定律", "f(E)=1/(exp((E-μ)/kT)-1)", "玻色子统计分布"),
            ("普朗克黑体辐射定律", "B(ν,T)", "黑体辐射谱分布"),
            ("维恩位移定律", "λmax·T = b", "黑体辐射峰值波长"),
            ("斯特藩-玻尔兹曼定律", "J = σT⁴", "黑体总辐射功率"),
        ]
    }

    /// 相变定律
    pub fn phase_transition_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("克拉珀龙方程", "dP/dT = ΔH/(TΔV)", "相变平衡线斜率"),
            ("克劳修斯-克拉珀龙方程", "dP/dT = ΔH·P/(RT²)", "气液相变关系"),
            ("吉布斯相律", "F = C - P + 2", "系统自由度与相数关系"),
            ("临界点定律", "临界状态", "物质临界点特性"),
            ("三相点定律", "三相共存", "物质三相点条件"),
            ("朗道相变定律", "序参量", "二级相变序参量描述"),
            ("相图定律", "相平衡", "物质相图规律"),
            ("过冷过热定律", "亚稳态", "过冷过热亚稳态现象"),
        ]
    }

    /// 非平衡热力学定律
    pub fn non_equilibrium_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("昂萨格倒易关系定律", "Lij = Lji", "线性非平衡区交叉系数对称"),
            ("最小熵产生定律", "dPi/dt ≤ 0", "稳态下熵产生最小"),
            ("耗散结构定律", "远离平衡", "普利高津耗散结构理论"),
            ("涨落定律", "涨落关联", "非平衡态涨落特性"),
            ("熵流定律", "熵流交换", "开放系统熵流交换"),
            ("热传导定律", "傅里叶定律", "热传导本构关系"),
            ("粘性流动定律", "牛顿粘性", "流体粘性应力定律"),
            ("扩散定律", "菲克定律", "物质扩散传质定律"),
        ]
    }

    /// 统计力学
    pub fn statistical_mechanics(&self) -> Vec<&'static str> {
        vec![
            "玻尔兹曼分布: 粒子在各能级上的平衡分布",
            "麦克斯韦速度分布: 理想气体分子速度的统计分布",
            "配分函数: 统计力学的核心量包含系统所有热力学信息",
            "熵的统计解释: S=k ln Ω系统微观状态数的对数",
            "系综理论: 大量相同系统的统计集合",
            "涨落: 物理量围绕平均值的随机偏离",
        ]
    }

    /// 相变理论
    pub fn phase_transitions(&self) -> Vec<&'static str> {
        vec![
            "一级相变: 有潜热和体积突变的相变",
            "二级相变: 无潜热但有比热等响应函数突变",
            "临界现象: 临界点附近物理量的奇异性",
            "临界指数: 描述临界点附近物理量标度行为的指数",
            "序参量: 描述系统有序程度的物理量",
            "对称性破缺: 相变时系统对称性降低",
            "重整化群: 研究不同尺度上物理行为的理论框架",
        ]
    }

}

impl Default for ThermodynamicsLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ThermodynamicsLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("thermodynamics")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【热力学定律】\n\n基本定律:\n{}\n\n统计力学定律:\n{}\n\n相变定律:\n{}\n\n非平衡热力学定律:\n{}\n",
            self.all_laws().iter()
                .map(|(name, formula, desc)| format!(
                    "▶ {}\n   公式/原理: {}\n   说明: {}\n",
                    name, formula, desc
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            self.statistical_mechanics_laws().iter()
                .map(|(name, formula, desc)| format!(
                    "▶ {}\n   公式/原理: {}\n   说明: {}\n",
                    name, formula, desc
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            self.phase_transition_laws().iter()
                .map(|(name, formula, desc)| format!(
                    "▶ {}\n   公式/原理: {}\n   说明: {}\n",
                    name, formula, desc
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            self.non_equilibrium_laws().iter()
                .map(|(name, formula, desc)| format!(
                    "▶ {}\n   公式/原理: {}\n   说明: {}\n",
                    name, formula, desc
                ))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermodynamics_laws() {
        let laws = ThermodynamicsLaws::new();
        assert!(!laws.all_laws().is_empty());
    }
}