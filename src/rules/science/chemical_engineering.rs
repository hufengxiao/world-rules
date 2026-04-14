//! 化学工程定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 化学工程定律集合
pub struct ChemicalEngineeringLaws {
    metadata: RuleMetadata,
}

impl ChemicalEngineeringLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new(
                "化学工程定律",
                "化学工程基本定律"
            )
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
            "【化学工程定律】\n\n反应工程定律:\n{}\n\n分离工程定律:\n{}\n\n传递定律:\n{}\n",
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