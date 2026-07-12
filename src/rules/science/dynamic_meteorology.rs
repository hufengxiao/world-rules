//! 动力气象学规则
//!
//! 动力气象学研究大气运动的动力学原理，
//! 包括大气环流、波动理论、能量转换和大气动力学方程。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 动力气象学规则集合
pub struct DynamicMeteorologyRules {
    metadata: RuleMetadata,
}

impl DynamicMeteorologyRules {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("动力气象学规则", "大气动力学原理和运动方程")
                .with_origin("地球科学")
                .with_tags(vec!["科学".into(), "气象".into(), "动力学".into()]),
        }
    }

    /// 大气运动方程规则
    pub fn atmospheric_motion_equations_rules(
        &self,
    ) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("连续方程定律", "质量守恒", "大气质量连续方程"),
            ("运动方程定律", "动量守恒", "大气运动Navier-Stokes方程"),
            ("热力学方程定律", "能量守恒", "大气热力学能量方程"),
            ("状态方程定律", "气体状态", "理想气体状态方程"),
            ("静力平衡方程定律", "垂直平衡", "大气静力平衡关系"),
            ("地转平衡方程定律", "水平平衡", "地转平衡运动方程"),
            ("涡度方程定律", "涡度演变", "大气涡度变化方程"),
            ("散度方程定律", "散度变化", "大气散度变化方程"),
        ]
    }

    /// 大气波动规则
    pub fn atmospheric_waves_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Rossby波定律", "行星波", "Rossby波形成和传播"),
            ("重力波定律", "重力振荡", "大气重力波特性"),
            ("惯性重力波定律", "惯性波", "惯性重力波分析"),
            ("声波定律", "声学波", "大气声波传播"),
            ("Kelvin波定律", "赤道Kelvin", "赤道Kelvin波特征"),
            ("混合Rossby重力波定律", "混合波", "混合Rossby重力波"),
            ("斜压波定律", "斜压发展", "斜压不稳定波动"),
            ("波动能量定律", "能量传播", "波动能量传播规律"),
        ]
    }

    /// 大气不稳定度规则
    pub fn atmospheric_instability_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("静力不稳定定律", "垂直不稳定", "大气静力不稳定度"),
            ("对流不稳定定律", "对流发展", "对流不稳定条件"),
            ("条件不稳定定律", "条件判断", "条件性不稳定分析"),
            ("斜压不稳定定律", "斜压发展", "斜压不稳定机制"),
            ("正压不稳定定律", "正压波动", "正压不稳定分析"),
            ("对称不稳定定律", "对称环流", "对称不稳定条件"),
            ("绝对不稳定定律", "绝对条件", "绝对不稳定度判断"),
            ("潜在不稳定定律", "潜在条件", "潜在不稳定分析"),
        ]
    }

    /// 大气能量规则
    pub fn atmospheric_energy_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("动能定律", "运动能量", "大气动能计算分析"),
            ("位能定律", "位势能量", "大气位能计算方法"),
            ("内能定律", "热力学能", "大气内能分析"),
            ("总能量定律", "能量总和", "大气总能量守恒"),
            ("有效位能定律", "有效能量", "有效位能转换分析"),
            ("能量转换定律", "能量循环", "动能位能转换循环"),
            ("能量平衡定律", "收支平衡", "大气能量收支平衡"),
            ("能量通量定律", "能量输送", "大气能量通量输送"),
        ]
    }

    /// 大气环流规则
    pub fn general_circulation_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("全球环流定律", "三圈环流", "全球大气三圈环流"),
            ("经向环流定律", "南北环流", "经向环流结构分析"),
            ("纬向环流定律", "东西环流", "纬向环流特征"),
            ("热力环流定律", "温差驱动", "热力环流形成机制"),
            ("动力环流定律", "动力驱动", "动力环流形成机制"),
            ("环流能量定律", "能量维持", "环流能量维持机制"),
            ("环流变异定律", "环流异常", "环流异常变化分析"),
            ("环流调整定律", "环流变化", "环流调整过程分析"),
        ]
    }

    /// 大气涡旋规则
    pub fn atmospheric_vortices_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("涡旋形成定律", "涡度源", "大气涡旋形成机制"),
            ("涡旋发展定律", "涡旋加强", "涡旋发展加强过程"),
            ("涡旋维持定律", "涡旋持续", "涡旋维持条件分析"),
            ("涡旋移动定律", "涡旋路径", "涡旋移动规律预测"),
            ("涡旋消散定律", "涡旋减弱", "涡旋消散减弱过程"),
            ("涡旋相互作用定律", "涡涡作用", "涡旋相互作用机制"),
            ("涡旋能量定律", "能量分析", "涡旋能量分布特征"),
            ("涡旋尺度定律", "尺度分类", "涡旋尺度分类特征"),
        ]
    }

    /// 大气边界层动力学规则
    pub fn boundary_layer_dynamics_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("湍流定律", "湍流运动", "大气湍流运动特性"),
            ("湍流能量定律", "湍流能量", "湍流能量收支平衡"),
            ("湍流输送定律", "湍流交换", "湍流热量动量输送"),
            ("边界层涡度定律", "边界层涡旋", "边界层涡度分布"),
            ("边界层稳定度定律", "稳定分析", "边界层稳定度变化"),
            ("边界层风定律", "风场分布", "边界层风廓线分析"),
            ("边界层混合定律", "混合过程", "边界层混合机制"),
            ("边界层响应定律", "响应时间", "边界层响应时间分析"),
        ]
    }

    /// 大气动力学诊断规则
    pub fn dynamics_diagnosis_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("涡度诊断定律", "涡度计算", "涡度计算和分析方法"),
            ("散度诊断定律", "散度计算", "散度计算和分析"),
            ("垂直运动诊断定律", "垂直速度", "垂直速度计算方法"),
            ("位涡诊断定律", "位涡分析", "位涡诊断和应用"),
            ("能量诊断定律", "能量收支", "能量收支诊断分析"),
            ("水汽诊断定律", "水汽收支", "水汽收支诊断计算"),
            ("温度诊断定律", "温度变化", "温度变化诊断分析"),
            ("环流诊断定律", "环流分析", "环流诊断分析方法"),
        ]
    }

    /// 数值方法规则
    pub fn numerical_methods_rules(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("差分方法定律", "有限差分", "大气方程差分求解"),
            ("谱方法定律", "谱展开", "大气方程谱方法"),
            ("有限元方法定律", "有限元", "大气方程有限元方法"),
            ("时间积分定律", "时间步长", "时间积分方案选择"),
            ("空间离散定律", "网格设计", "空间离散网格设计"),
            ("边界条件定律", "边界处理", "数值边界条件处理"),
            ("稳定性定律", "数值稳定", "数值稳定性分析"),
            ("收敛性定律", "数值收敛", "数值收敛性判断"),
        ]
    }

    /// 动力学应用领域
    pub fn application_areas(&self) -> Vec<&'static str> {
        vec![
            "天气预报: 大气动力学应用于天气预报",
            "气候模拟: 大气动力学在气候模式中的应用",
            "风暴预报: 动力学方法预报风暴发展",
            "环流分析: 大气环流动力学分析",
            "能量分析: 大气能量收支动力学研究",
            "涡旋研究: 涡旋动力学研究方法",
            "边界层研究: 边界层动力学应用",
            "波动研究: 大气波动动力学分析",
        ]
    }

    /// 动力学研究方法
    pub fn research_methods(&self) -> Vec<&'static str> {
        vec![
            "理论分析: 大气动力学理论模型分析",
            "数值模拟: 大气动力学数值模拟方法",
            "观测诊断: 观测资料动力学诊断",
            "实验研究: 实验室动力学实验方法",
            "波动分析: 大气波动分析方法",
            "能量分析: 大气能量分析方法",
            "尺度分析: 大气运动尺度分析方法",
            "稳定性分析: 大气不稳定度分析方法",
        ]
    }
}

impl Default for DynamicMeteorologyRules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for DynamicMeteorologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("dynamic_meteorology")
    }

    fn explain(&self) -> String {
        format!(
            "【动力气象学规则】\n\n\
            大气运动方程规则:\n{}\n\n\
            大气波动规则:\n{}\n\n\
            大气不稳定度规则:\n{}\n\n\
            大气能量规则:\n{}\n\n\
            大气环流规则:\n{}\n\n\
            大气涡旋规则:\n{}\n\n\
            大气边界层动力学规则:\n{}\n\n\
            大气动力学诊断规则:\n{}\n\n\
            数值方法规则:\n{}\n\n\
            动力学应用领域:\n{}\n\n\
            动力学研究方法:\n{}",
            self.atmospheric_motion_equations_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_waves_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_instability_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_energy_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.general_circulation_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.atmospheric_vortices_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.boundary_layer_dynamics_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.dynamics_diagnosis_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.numerical_methods_rules()
                .iter()
                .map(|(n, d, e)| format!("  • {}: {} - {}", n, d, e))
                .collect::<Vec<_>>()
                .join("\n"),
            self.application_areas()
                .iter()
                .map(|a| format!("  • {}", a))
                .collect::<Vec<_>>()
                .join("\n"),
            self.research_methods()
                .iter()
                .map(|m| format!("  • {}", m))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_meteorology_rules() {
        let rules = DynamicMeteorologyRules::new();
        assert_eq!(rules.atmospheric_motion_equations_rules().len(), 8);
        assert_eq!(rules.atmospheric_waves_rules().len(), 8);
        assert_eq!(rules.atmospheric_instability_rules().len(), 8);
        assert_eq!(rules.atmospheric_energy_rules().len(), 8);
        assert_eq!(rules.general_circulation_rules().len(), 8);
        assert_eq!(rules.atmospheric_vortices_rules().len(), 8);
        assert_eq!(rules.boundary_layer_dynamics_rules().len(), 8);
        assert_eq!(rules.dynamics_diagnosis_rules().len(), 8);
        assert_eq!(rules.numerical_methods_rules().len(), 8);
        assert!(!rules.explain().is_empty());
    }

    #[test]
    fn test_waves_rules() {
        let rules = DynamicMeteorologyRules::new();
        let laws = rules.atmospheric_waves_rules();
        assert!(laws.iter().any(|(n, _, _)| n.contains("Rossby")));
    }

    #[test]
    fn test_energy_rules() {
        let rules = DynamicMeteorologyRules::new();
        assert_eq!(rules.atmospheric_energy_rules().len(), 8);
    }

    #[test]
    fn test_application_areas() {
        let rules = DynamicMeteorologyRules::new();
        assert_eq!(rules.application_areas().len(), 8);
    }
}
