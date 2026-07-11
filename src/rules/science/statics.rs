//! 静力学规则
//!
//! 静力学研究物体在力作用下的平衡条件，是工程力学的基础。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: StaticsRules,
    name: "静力学规则",
    desc: "静力学平衡条件与受力分析方法",
    origin: "力学",
    tags: ["科学", "物理", "力学", "静力学"]
}

impl StaticsRules {
    /// 平衡条件
    pub fn equilibrium_conditions(&self) -> Vec<&'static str> {
        vec![
            "力的平衡条件: ΣF = 0（合力为零）",
            "力矩平衡条件: ΣM = 0（合力矩为零）",
            "二力平衡: 两个力大小相等、方向相反、作用在同一直线上",
            "三力平衡: 三力汇交于一点，形成封闭三角形",
            "物体平衡状态: 静止或匀速直线运动",
            "稳定平衡: 物体偏离平衡位置后能自动恢复",
            "不稳定平衡: 物体偏离平衡位置后不能恢复",
            "随遇平衡: 物体在任何位置都能保持平衡",
        ]
    }

    /// 力的分析方法
    pub fn force_analysis(&self) -> Vec<&'static str> {
        vec![
            "受力分析步骤: 确定研究对象、隔离物体、画受力图",
            "重力: G = mg，方向竖直向下，作用在重心",
            "弹力: 方向垂直于接触面，大小由胡克定律 F = kx 确定",
            "摩擦力: 静摩擦力 0 ≤ f ≤ μN，滑动摩擦力 f = μN",
            "支持力: 垂直于接触面，指向物体",
            "绳索拉力: 沿绳索方向，大小处处相等（忽略绳索质量）",
            "铰链约束: 限制移动但允许转动",
            "固定端约束: 同时限制移动和转动",
        ]
    }

    /// 力矩计算
    pub fn moment_calculation(&self) -> Vec<&'static str> {
        vec![
            "力矩定义: M = F × r = Fr sinθ",
            "力臂: 从转动轴到力作用线的垂直距离",
            "正力矩: 使物体逆时针转动的力矩",
            "负力矩: 使物体顺时针转动的力矩",
            "力矩平衡: ΣM = 0 对任意转动轴成立",
            "合力矩: 多个力矩的代数和",
            "力偶: 两个大小相等、方向相反、不在同一直线上的力",
            "力偶矩: M = Fd，其中 d 为两力作用线间的距离",
        ]
    }

    /// 杆件分析
    pub fn truss_analysis(&self) -> Vec<&'static str> {
        vec![
            "桁架假设: 杆件为二力杆，节点为铰接",
            "节点法: 对每个节点应用平衡方程",
            "截面法: 截断部分杆件，分析截面平衡",
            "零力杆判断: 不受力或受力为零的杆件",
            "桁架类型: 简单桁架、组合桁架、复杂桁架",
            "杆件内力: 拉力（正）和压力（负）",
            "稳定性: 桁架必须有足够的约束",
            "静定桁架: 平衡方程足以求解所有内力",
        ]
    }

    /// 梁的分析
    pub fn beam_analysis(&self) -> Vec<&'static str> {
        vec![
            "梁的类型: 简支梁、悬臂梁、外伸梁",
            "支座类型: 固定铰支座、可动铰支座、固定端",
            "载荷类型: 集中力、分布载荷、集中力偶",
            "弯矩定义: M = Fd，使梁产生弯曲变形",
            "剪力定义: V = ΣF，垂直于梁轴线方向的力",
            "弯矩图: 表示弯矩沿梁长度分布的图形",
            "剪力图: 表示剪力沿梁长度分布的图形",
            "弯矩与剪力关系: dM/dx = V",
        ]
    }

    /// 摩擦分析
    pub fn friction_analysis(&self) -> Vec<&'static str> {
        vec![
            "静摩擦系数 μs: 最大静摩擦力与正压力之比",
            "动摩擦系数 μk: 滑动摩擦力与正压力之比",
            "摩擦角 φ: tanφ = μ，摩擦力达到最大时的角度",
            "自锁条件: 主动力作用线在摩擦角内",
            "摩擦力方向: 与相对运动或相对运动趋势方向相反",
            "滚动摩擦: 比滑动摩擦小得多",
            "摩擦应用: 制动器、离合器、螺纹紧固",
            "减小摩擦: 润滑、滚动轴承、气浮支承",
        ]
    }

    /// 重心计算
    pub fn center_of_gravity(&self) -> Vec<&'static str> {
        vec![
            "重心定义: 重力作用点",
            "质心定义: 质量分布中心",
            "均匀物体: 重心与几何中心重合",
            "复合形体: 各部分重心的加权平均",
            "重心坐标: xc = Σ(mx)/Σm",
            "悬挂法: 利用悬挂两次确定重心位置",
            "称重法: 通过称重测量重心位置",
            "稳定性: 重心越低越稳定",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "建筑结构: 桁架、梁、柱的设计",
            "桥梁设计: 确保各种载荷下的稳定",
            "机械设计: 轴承、齿轮、连杆",
            "起重机设计: 防倾覆计算",
            "塔吊稳定: 配重和倾覆力矩平衡",
            "压力容器: 应力和稳定性分析",
            "水利工程: 堤坝稳定性",
            "航空结构: 机翼、机身受力分析",
        ]
    }
}

impl Rule for StaticsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("statics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "静力学规则",
            &[
                ("平衡条件", &self.equilibrium_conditions()),
                ("受力分析", &self.force_analysis()),
                ("力矩计算", &self.moment_calculation()),
                ("桁架分析", &self.truss_analysis()),
                ("梁的分析", &self.beam_analysis()),
                ("摩擦分析", &self.friction_analysis()),
                ("重心计算", &self.center_of_gravity()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statics_rules() {
        let rules = StaticsRules::new();
        assert_eq!(rules.metadata().name, "静力学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.equilibrium_conditions().is_empty());
        assert!(!rules.force_analysis().is_empty());
        assert!(!rules.moment_calculation().is_empty());
    }
}
