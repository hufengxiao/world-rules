//! 计算力学规则
//!
//! 计算力学研究数值方法和计算技术在力学中的应用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ComputationalMechanicsRules,
    name: "计算力学规则",
    desc: "力学数值计算方法和仿真技术",
    origin: "力学",
    tags: ["科学", "物理", "力学", "计算"]
}

impl ComputationalMechanicsRules {
    /// 数值方法基础
    pub fn numerical_methods(&self) -> Vec<&'static str> {
        vec![
            "数值求解: 用计算机求解力学方程",
            "离散化: 连续问题离散为数值问题",
            "网格生成: 划分计算区域",
            "时间步长: 时间离散化间隔",
            "空间步长: 空间离散化间隔",
            "数值稳定: 算法不发散",
            "数值精度: 结果接近真实值",
            "收敛性: 结果随网格细化趋于真值",
        ]
    }

    /// 有限元方法
    pub fn finite_element_method(&self) -> Vec<&'static str> {
        vec![
            "有限元法 FEM: 将连续体离散为有限单元",
            "单元类型: 三角形、四边形、四面体、六面体",
            "节点: 单元的连接点",
            "形函数: 描述单元内位移分布",
            "刚度矩阵: 单元刚度矩阵组装为整体刚度矩阵",
            "位移场: 由节点位移确定",
            "应力计算: 由位移计算应力",
            "边界条件: 施加力和位移约束",
        ]
    }

    /// 边界元方法
    pub fn boundary_element_method(&self) -> Vec<&'static str> {
        vec![
            "边界元法 BEM: 只离散边界",
            "边界积分方程: 边界上的积分方程",
            "边界离散: 边界划分为边界单元",
            "优势: 减少计算量",
            "应用: 无限域问题、裂纹问题",
            "局限: 需要解析基本解",
            "边界条件: 边界上的力或位移",
            "内点计算: 由边界值计算内部场",
        ]
    }

    /// 无网格方法
    pub fn meshless_methods(&self) -> Vec<&'static str> {
        vec![
            "无网格方法: 不需要网格划分",
            "粒子法: 用粒子代表物质点",
            "光滑粒子流体动力学 SPH: 流体无网格方法",
            "物质点法 MPM: 结合有限元和粒子法",
            "径向基函数: 无网格插值函数",
            "优势: 处理大变形、断裂",
            "局限: 计算量较大",
            "应用: 高速冲击、爆炸、流体",
        ]
    }

    /// 多体动力学
    pub fn multibody_dynamics(&self) -> Vec<&'static str> {
        vec![
            "多体系统: 多个刚体或柔体组成的系统",
            "约束方程: 描述体间连接关系",
            "动力学方程: 拉格朗日方程或牛顿-欧拉方程",
            "数值积分: Runge-Kutta、Adams等",
            "碰撞检测: 判断体间接触",
            "接触力学: 接触力和变形",
            "柔性多体: 考虑体弹性变形",
            "应用: 机械系统、车辆、机器人",
        ]
    }

    /// 流体计算
    pub fn computational_fluid_dynamics(&self) -> Vec<&'static str> {
        vec![
            "CFD: 流体动力学数值计算",
            "网格类型: 结构网格、非结构网格",
            "离散方法: 有限体积、有限差分、有限元",
            "湍流模型: k-ε、k-ω、RANS等",
            "边界条件: 入口、出口、壁面",
            "求解器: 压力-速度耦合算法",
            "收敛判据: 残差小于阈值",
            "应用: 航空、汽车、气象",
        ]
    }

    /// 结构优化
    pub fn structural_optimization(&self) -> Vec<&'static str> {
        vec![
            "尺寸优化: 优化构件尺寸参数",
            "形状优化: 优化构件几何形状",
            "拓扑优化: 优化材料分布",
            "目标函数: 最小重量、最大刚度",
            "约束条件: 应力、位移、频率约束",
            "优化算法: 遗传算法、梯度法",
            "灵敏度分析: 目标对设计变量的导数",
            "应用: 航空结构、汽车结构",
        ]
    }

    /// 软件和工具
    pub fn software_tools(&self) -> Vec<&'static str> {
        vec![
            "商业软件: ANSYS、Abaqus、COMSOL",
            "开源软件: Code_Aster、OpenFOAM",
            "前处理: 几何建模、网格生成",
            "求解器: 核心计算引擎",
            "后处理: 结果可视化、分析",
            "并行计算: 多核、多机并行",
            "GPU加速: 利用GPU加速计算",
            "云计算: 云平台进行大规模计算",
        ]
    }
}

impl Rule for ComputationalMechanicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("computational_mechanics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "计算力学规则",
            &[
                ("数值方法基础", &self.numerical_methods()),
                ("有限元方法", &self.finite_element_method()),
                ("边界元方法", &self.boundary_element_method()),
                ("无网格方法", &self.meshless_methods()),
                ("多体动力学", &self.multibody_dynamics()),
                ("流体计算", &self.computational_fluid_dynamics()),
                ("结构优化", &self.structural_optimization()),
                ("软件和工具", &self.software_tools()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computational_mechanics_rules() {
        let rules = ComputationalMechanicsRules::new();
        assert_eq!(rules.metadata().name, "计算力学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.numerical_methods().is_empty());
        assert!(!rules.finite_element_method().is_empty());
        assert!(!rules.computational_fluid_dynamics().is_empty());
    }
}