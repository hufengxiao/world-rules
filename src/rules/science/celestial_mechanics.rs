//! 天体力学规则
//!
//! 天体力学研究天体运动的规律。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: CelestialMechanicsRules,
    name: "天体力学规则",
    desc: "天体运动、轨道计算和引力场分析",
    origin: "力学",
    tags: ["科学", "物理", "力学", "天体"]
}

impl CelestialMechanicsRules {
    /// 万有引力定律
    pub fn gravitational_laws(&self) -> Vec<&'static str> {
        vec![
            "万有引力定律: F = GMm/r²",
            "引力常数 G: G ≈ 6.674×10⁻¹¹ N·m²/kg²",
            "引力场强度: g = GM/r²",
            "引力势能: U = -GMm/r",
            "引力势: Φ = -GM/r",
            "引力场: 引力作用的空间",
            "引力范围: 无限远",
            "引力作用: 所有有质量的物体",
        ]
    }

    /// 轨道运动
    pub fn orbital_motion(&self) -> Vec<&'static str> {
        vec![
            "开普勒第一定律: 轨道为椭圆，中心天体在焦点",
            "开普勒第二定律: 等面积定律，面积速率恒定",
            "开普勒第三定律: T² = (4π²/GM) a³",
            "椭圆轨道: 半长轴 a，半短轴 b",
            "轨道周期: T = 2π√(a³/GM)",
            "轨道速度: v² = GM(2/r - 1/a)",
            "近地点: 最接近中心天体",
            "远地点: 最远离中心天体",
        ]
    }

    /// 圆轨道
    pub fn circular_orbit(&self) -> Vec<&'static str> {
        vec![
            "圆轨道条件: v = √(GM/r)",
            "圆轨道周期: T = 2πr/v = 2π√(r³/GM)",
            "圆轨道速度: 第一宇宙速度 v₁ = √(GM/R)",
            "地球第一宇宙速度: v₁ ≈ 7.9 km/s",
            "圆轨道能量: E = -GMm/(2r)",
            "圆轨道稳定: 无扰动时保持圆形",
            "同步轨道: 与中心天体自转同步",
            "地球同步轨道: 轨道半径 ≈ 42,164 km",
        ]
    }

    /// 轨道能量
    pub fn orbital_energy(&self) -> Vec<&'static str> {
        vec![
            "轨道动能: Ek = GMm/(2r)（圆轨道）",
            "轨道势能: U = -GMm/r",
            "总能量: E = Ek + U = -GMm/(2r)（圆轨道）",
            "能量判别: E < 0 椭圆，E = 0 抛物线，E > 0 双曲线",
            "椭圆轨道能量: E = -GMm/(2a)",
            "逃逸能量: E = 0（抛物线轨道）",
            "逃逸速度: v₂ = √(2GM/r)",
            "地球逃逸速度: v₂ ≈ 11.2 km/s",
        ]
    }

    /// 轨道转移
    pub fn orbital_transfer(&self) -> Vec<&'static str> {
        vec![
            "霍曼转移轨道: 最省能量的轨道转移",
            "转移椭圆: 连接两个圆轨道的椭圆",
            "转移时间: T转移 = π√(a转移³/GM)",
            "轨道变轨: 改变速度实现轨道转移",
            "单次变轨: 改变一个轨道参数",
            "多次变轨: 分步实现目标轨道",
            "轨道会合: 两个天体在同一轨道相遇",
            "轨道修正: 调整轨道偏差",
        ]
    }

    /// 多体问题
    pub fn multi_body_problem(&self) -> Vec<&'static str> {
        vec![
            "二体问题: 可精确求解",
            "三体问题: 一般无解析解",
            "限制性三体问题: 一个天体质量很小",
            "拉格朗日点: 三体系统的平衡点",
            "L1点: 位于两个大天体之间",
            "L2点: 位于小天体背后",
            "L4、L5点: 与两个大天体形成三角形",
            "多体系统: 复杂的引力相互作用",
        ]
    }

    /// 轨道摄动
    pub fn orbital_perturbation(&self) -> Vec<&'static str> {
        vec![
            "摄动因素: 其他天体引力、非球形引力",
            "摄动方程: 轨道参数随时间变化",
            "长期摄动: 长时间的轨道漂移",
            "周期摄动: 周期性的轨道变化",
            "轨道共振: 轨道周期有整数比关系",
            "轨道衰减: 能量损失导致的轨道变化",
            "轨道进动: 轨道平面转动",
            "轨道稳定性: 长期保持轨道形态",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "卫星轨道: 通信、导航、科学卫星",
            "行星探测: 深空探测任务",
            "彗星轨道: 彗星的椭圆或抛物线轨道",
            "小行星轨道: 近地小行星追踪",
            "引力波探测: 双星系统引力波",
            "轨道设计: 航天任务规划",
            "轨道预测: 天体碰撞预警",
            "引力辅助: 利用行星引力改变轨道",
        ]
    }
}

impl Rule for CelestialMechanicsRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("celestial_mechanics")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "天体力学规则",
            &[
                ("万有引力定律", &self.gravitational_laws()),
                ("轨道运动", &self.orbital_motion()),
                ("圆轨道", &self.circular_orbit()),
                ("轨道能量", &self.orbital_energy()),
                ("轨道转移", &self.orbital_transfer()),
                ("多体问题", &self.multi_body_problem()),
                ("轨道摄动", &self.orbital_perturbation()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celestial_mechanics_rules() {
        let rules = CelestialMechanicsRules::new();
        assert_eq!(rules.metadata().name, "天体力学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.gravitational_laws().is_empty());
        assert!(!rules.orbital_motion().is_empty());
        assert!(!rules.circular_orbit().is_empty());
    }
}