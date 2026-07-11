//! 微波技术规则
//!
//! 微波技术研究微波的产生、传输和处理技术。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: MicrowaveTechnologyRules,
    name: "微波技术规则",
    desc: "微波系统设计与应用方法",
    origin: "电磁学",
    tags: ["科学", "物理", "电磁", "微波"]
}

impl MicrowaveTechnologyRules {
    /// 微波特性
    pub fn microwave_properties(&self) -> Vec<&'static str> {
        vec![
            "微波频率: 3×10⁹ - 3×10¹² Hz",
            "微波波长: 1mm - 1m",
            "微波特点: 高频率、短波长、直线传播",
            "微波穿透: 能穿透某些材料",
            "微波吸收: 某些材料吸收微波发热",
            "微波反射: 金属表面反射微波",
            "微波散射: 微波遇到物体散射",
            "微波折射: 微波在不同介质折射",
        ]
    }

    /// 微波传输线
    pub fn transmission_lines(&self) -> Vec<&'static str> {
        vec![
            "传输线类型: 同轴线、微带线、波导",
            "同轴线: 双导体传输线",
            "微带线: PCB上的平面传输线",
            "波导: 金属管道传输微波",
            "传输线阻抗: Z₀ = √(L/C)",
            "特性阻抗: 匹配传输线阻抗",
            "驻波比: VSWR = Vmax/Vmin",
            "阻抗匹配: 消除反射",
        ]
    }

    /// 波导理论
    pub fn waveguide_theory(&self) -> Vec<&'static str> {
        vec![
            "波导原理: 空心金属管传输电磁波",
            "截止频率: fₛ = c/(2a)，最低传输频率",
            "波导模式: TE、TM模式",
            "TE模式: 电场垂直于传播方向",
            "TM模式: 磁场垂直于传播方向",
            "波导尺寸: 与波长相关",
            "波导损耗: 金属壁电阻损耗",
            "波导应用: 雷达、卫星通信",
        ]
    }

    /// 微波器件
    pub fn microwave_devices(&self) -> Vec<&'static str> {
        vec![
            "微波源: 磁控管、 Gunn振荡器",
            "微波放大器: 行波管、固态放大器",
            "微波混频器: 频率变换",
            "微波滤波器: 频率选择",
            "微波衰减器: 调节功率",
            "微波耦合器: 分配功率",
            "微波开关: 控制传输",
            "微波检波器: 检测信号",
        ]
    }

    /// 微波天线
    pub fn microwave_antennas(&self) -> Vec<&'static str> {
        vec![
            "天线类型: 喇叭天线、抛物面天线",
            "喇叭天线: 波导开口辐射",
            "抛物面天线: 反射聚焦",
            "天线增益: G = 4πA/λ²",
            "天线方向图: 辐射方向分布",
            "天线带宽: 工作频率范围",
            "天线效率: 有效辐射比例",
            "天线极化: 辐射电磁波极化",
        ]
    }

    /// 微波测量
    pub fn microwave_measurements(&self) -> Vec<&'static str> {
        vec![
            "功率测量: 测量微波功率",
            "频率测量: 测量微波频率",
            "阻抗测量: 测量传输线阻抗",
            "驻波测量: 测量驻波比",
            "衰减测量: 测量衰减量",
            "相位测量: 测量相位差",
            "网络分析仪: 测量S参数",
            "频谱分析仪: 分析频率成分",
        ]
    }

    /// 微波应用
    pub fn microwave_applications(&self) -> Vec<&'static str> {
        vec![
            "雷达: 探测、定位",
            "卫星通信: 全球通信",
            "无线通信: 手机、WiFi",
            "微波加热: 微波炉",
            "微波干燥: 工业干燥",
            "微波医学: 医疗诊断治疗",
            "微波遥感: 地球观测",
            "微波加热: 食物加热",
        ]
    }

    /// 微波安全
    pub fn microwave_safety(&self) -> Vec<&'static str> {
        vec![
            "功率密度限值: 安全功率密度",
            "暴露时间: 暴露时间限制",
            "防护措施: 屏蔽、远离",
            "泄漏检测: 检测微波泄漏",
            "微波危害: 高功率微波危害",
            "安全标准: 微波安全标准",
            "防护设备: 微波防护设备",
            "安全距离: 保持安全距离",
        ]
    }
}

impl Rule for MicrowaveTechnologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("microwave_technology")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "微波技术规则",
            &[
                ("微波特性", &self.microwave_properties()),
                ("微波传输线", &self.transmission_lines()),
                ("波导理论", &self.waveguide_theory()),
                ("微波器件", &self.microwave_devices()),
                ("微波天线", &self.microwave_antennas()),
                ("微波测量", &self.microwave_measurements()),
                ("微波应用", &self.microwave_applications()),
                ("微波安全", &self.microwave_safety()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_microwave_technology_rules() {
        let rules = MicrowaveTechnologyRules::new();
        assert_eq!(rules.metadata().name, "微波技术规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.microwave_properties().is_empty());
        assert!(!rules.transmission_lines().is_empty());
    }
}
