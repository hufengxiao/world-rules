//! 电磁感应规则
//!
//! 电磁感应研究变化的磁场产生电场的现象和规律。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ElectromagneticInductionRules,
    name: "电磁感应规则",
    desc: "电磁感应现象与应用方法",
    origin: "电磁学",
    tags: ["科学", "物理", "电磁", "感应"]
}

impl ElectromagneticInductionRules {
    /// 法拉第定律
    pub fn faraday_law(&self) -> Vec<&'static str> {
        vec![
            "法拉第电磁感应定律: ε = -dΦ/dt",
            "感应电动势: 磁通量变化产生电动势",
            "磁通量定义: Φ = B·A = BAcosθ",
            "磁通量变化: 面积变化、磁场变化、角度变化",
            "感应电动势方向: 楞次定律确定",
            "法拉第定律积分形式: ε = -N(dΦ/dt)",
            "感应电场: ∮E·dl = -dΦ/dt，非保守场",
            "涡旋电场: 变化磁场产生的环形电场",
        ]
    }

    /// 楞次定律
    pub fn lenz_law(&self) -> Vec<&'static str> {
        vec![
            "楞次定律: 感应电流阻碍磁通量变化",
            "能量守恒: 楞次定律体现能量守恒",
            "右手定则: 判断感应电流方向",
            "右手螺旋定则: 判断感应磁场方向",
            "阻碍增加: 磁通增加时感应电流产生反向磁场",
            "阻碍减少: 磁通减少时感应电流产生同向磁场",
            "惯性类比: 感应电流的惯性特性",
            "楞次定律应用: 电机、变压器设计",
        ]
    }

    /// 自感与互感
    pub fn self_mutual_induction(&self) -> Vec<&'static str> {
        vec![
            "自感现象: 线圈自身电流变化产生感应电动势",
            "自感系数: L = Φ/I，自感电动势 ε = -L(di/dt)",
            "互感现象: 一个线圈电流变化在另一线圈产生电动势",
            "互感系数: M = Φ₂₁/I₁ = Φ₁₂/I₂",
            "互感电动势: ε₂ = -M(di₁/dt)",
            "耦合系数: k = M/√(L₁L₂)，0 ≤ k ≤ 1",
            "理想耦合: k = 1，无漏磁",
            "同名端: 两线圈感应电动势同极性端",
        ]
    }

    /// 感应电流
    pub fn induced_current(&self) -> Vec<&'static str> {
        vec![
            "感应电流条件: 磁通量变化和闭合回路",
            "感应电流大小: I = ε/R",
            "感应电流方向: 楞次定律或右手定则判断",
            "导体切割磁力线: ε = Blv",
            "导体转动切割: ε = ½Bl²ω",
            "感应电流能量: 来自磁场能量或机械功",
            "感应电流热效应: Q = I²Rt",
            "感应电流限制: 电阻和磁通变化率",
        ]
    }

    /// 涡流
    pub fn eddy_current(&self) -> Vec<&'static str> {
        vec![
            "涡流定义: 大块导体中的感应电流",
            "涡流方向: 垂直于磁场变化方向",
            "涡流热效应: 涡流产生热量（涡流损耗）",
            "涡流应用: 感应加热、电磁炉",
            "涡流阻尼: 涡流产生的阻尼效应",
            "涡流屏蔽: 涡流屏蔽变化的磁场",
            "减小涡流: 分层、采用高电阻材料",
            "涡流检测: 无损检测金属材料缺陷",
        ]
    }

    /// 变压器原理
    pub fn transformer_principle(&self) -> Vec<&'static str> {
        vec![
            "变压器原理: 电磁感应改变电压",
            "理想变压器: V₁/V₂ = N₁/N₂",
            "电流关系: I₁/I₂ = N₂/N₁",
            "功率守恒: P₁ = P₂（理想变压器）",
            "升压变压器: N₂ > N₁",
            "降压变压器: N₂ < N₁",
            "铁芯作用: 增强磁场耦合",
            "变压器效率: η = P₂/P₁",
        ]
    }

    /// 电磁波传播
    pub fn electromagnetic_waves(&self) -> Vec<&'static str> {
        vec![
            "电磁波产生: 变化的电场和磁场相互激发",
            "电磁波速度: c = 1/√(ε₀μ₀) = 3×10⁸ m/s",
            "电磁波特性: 横波，E和B垂直于传播方向",
            "电磁波能量: E和B同相位变化",
            "电磁波能流: S = E×H（坡印廷矢量）",
            "电磁波频率: f = c/λ",
            "电磁波波长: λ = c/f",
            "电磁波谱: 无线电、微波、红外、可见光、紫外、X射线、γ射线",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "发电机: 机械能转电能",
            "变压器: 改变交流电压",
            "感应加热: 涡流加热金属",
            "电磁炉: 涡流加热厨具",
            "感应电动机: 旋转磁场驱动转子",
            "无线充电: 磁耦合传输能量",
            "电磁屏蔽: 防止电磁干扰",
            "磁悬浮: 电磁力悬浮物体",
        ]
    }
}

impl Rule for ElectromagneticInductionRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("electromagnetic_induction")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电磁感应规则",
            &[
                ("法拉第定律", &self.faraday_law()),
                ("楞次定律", &self.lenz_law()),
                ("自感与互感", &self.self_mutual_induction()),
                ("感应电流", &self.induced_current()),
                ("涡流", &self.eddy_current()),
                ("变压器原理", &self.transformer_principle()),
                ("电磁波传播", &self.electromagnetic_waves()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electromagnetic_induction_rules() {
        let rules = ElectromagneticInductionRules::new();
        assert_eq!(rules.metadata().name, "电磁感应规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.faraday_law().is_empty());
        assert!(!rules.lenz_law().is_empty());
    }
}
