//! 化学工程定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 化学工程定律集合
pub struct ChemicalEngineeringLaws {
    metadata: RuleMetadata,
}

impl ChemicalEngineeringLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("化学工程定律", "化学工程基本定律")
                .with_origin("工程")
                .with_tags(vec!["科学".into(), "工程".into(), "化学".into()]),
        }
    }

    /// 反应工程定律
    pub fn reaction_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("反应器定律", "反应器设计", "化学反应器设计"),
            ("反应速率定律", "速率方程", "工业反应速率"),
            ("传质定律", "物质传递", "反应传质过程"),
            ("传热定律", "热量传递", "反应热传递"),
            ("催化定律", "催化反应", "工业催化原理"),
            ("选择性定律", "选择性控制", "产物选择性"),
            ("转化率定律", "转化控制", "反应转化率"),
            ("收率定律", "收率计算", "产物收率"),
        ]
    }

    /// 分离工程定律
    pub fn separation_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("蒸馏定律", "蒸馏分离", "蒸馏分离原理"),
            ("萃取定律", "萃取分离", "萃取分离过程"),
            ("吸收定律", "吸收分离", "吸收分离原理"),
            ("吸附定律", "吸附分离", "吸附分离过程"),
            ("结晶定律", "结晶分离", "结晶分离方法"),
            ("膜分离定律", "膜分离", "膜分离技术"),
            ("干燥定律", "干燥过程", "干燥分离原理"),
            ("过滤定律", "过滤分离", "过滤分离方法"),
        ]
    }

    /// 传递定律
    pub fn transport_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("流体流动定律", "流动规律", "流体流动原理"),
            ("热量传递定律", "传热规律", "热量传递过程"),
            ("质量传递定律", "传质规律", "质量传递过程"),
            ("动量传递定律", "动量传递", "动量传递原理"),
            ("边界层定律", "边界层理论", "边界层效应"),
            ("扩散定律", "扩散过程", "物质扩散规律"),
            ("对流定律", "对流传递", "对流传递过程"),
        ]
    }

    /// 过程控制定律
    pub fn control_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("过程控制定律", "自动控制", "过程自动控制"),
            ("反馈定律", "反馈控制", "反馈控制系统"),
            ("稳定性定律", "系统稳定", "控制稳定性"),
            ("优化定律", "过程优化", "过程优化方法"),
            ("PID控制定律", "PID控制", "PID控制原理"),
            ("串级控制定律", "串级系统", "串级控制系统"),
            ("前馈控制定律", "前馈系统", "前馈控制原理"),
        ]
    }

    /// 化工设备
    pub fn equipment(&self) -> Vec<&'static str> {
        vec![
            "反应器",
            "蒸馏塔",
            "换热器",
            "泵",
            "压缩机",
            "过滤器",
            "干燥器",
            "储罐",
        ]
    }

    /// 化工过程
    pub fn processes(&self) -> Vec<&'static str> {
        vec![
            "合成过程",
            "分离过程",
            "提纯过程",
            "回收过程",
            "循环过程",
            "连续过程",
            "间歇过程",
            "半连续过程",
        ]
    }

    /// 化工热力学定律
    pub fn thermodynamics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("相平衡定律", "相平衡", "化工过程相平衡计算"),
            ("化学平衡定律", "平衡常数", "化学反应平衡常数"),
            ("反应热定律", "反应热", "化学反应热效应计算"),
            ("逸度定律", "逸度", "实际气体逸度计算"),
            ("活度定律", "活度", "溶液活度系数计算"),
            ("状态方程定律", "状态方程", "立方型状态方程应用"),
            ("混合规则定律", "混合规则", "混合物性质计算规则"),
            ("过程热力学定律", "火用分析", "过程热力学效率分析"),
        ]
    }

    /// 化工安全定律
    pub fn safety_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("本质安全定律", "本质安全", "化工过程本质安全设计"),
            ("HAZOP分析定律", "危害分析", "危险与可操作性分析"),
            ("安全阀定律", "泄压保护", "安全阀泄压保护设计"),
            ("防火防爆定律", "防火防爆", "化工防火防爆措施"),
            ("毒性防护定律", "毒性控制", "化学品毒性防护措施"),
            ("应急响应定律", "应急预案", "化工事故应急响应"),
            ("安全连锁定律", "连锁保护", "安全连锁保护系统"),
            ("风险评估定律", "定量风险", "化工定量风险评估"),
        ]
    }

    /// 绿色化工定律
    pub fn green_engineering_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("原子经济性定律", "原子利用", "最大化原子利用率"),
            ("绿色溶剂定律", "溶剂选择", "绿色溶剂替代传统溶剂"),
            ("催化绿色定律", "绿色催化", "催化替代化学计量反应"),
            ("废物最小化定律", "源头减量", "化工废物源头最小化"),
            ("能量集成定律", "热集成", "过程能量集成优化"),
            ("水网络优化定律", "水回用", "化工用水网络优化"),
            ("生命周期评价定律", "LCA", "产品全生命周期环境影响"),
            ("过程强化定律", "过程强化", "化工过程强化技术"),
        ]
    }
}

impl Default for ChemicalEngineeringLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ChemicalEngineeringLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("chemical_engineering")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【化学工程定律】\n\n反应工程定律:\n{}\n\n分离工程定律:\n{}\n\n传递定律:\n{}\n\n化工热力学定律:\n{}\n\n化工安全定律:\n{}\n\n绿色化工定律:\n{}\n",
            self.reaction_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.separation_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.transport_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.thermodynamics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.safety_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.green_engineering_laws().iter()
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
    fn test_chemical_engineering_laws() {
        let laws = ChemicalEngineeringLaws::new();
        assert!(!laws.reaction_laws().is_empty());
        assert!(!laws.separation_laws().is_empty());
    }
}
