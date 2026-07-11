//! 电磁兼容规则
//!
//! 电磁兼容研究电子设备在电磁环境中的正常工作和互不干扰。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: ElectromagneticCompatibilityRules,
    name: "电磁兼容规则",
    desc: "电磁干扰与防护方法",
    origin: "电磁学",
    tags: ["科学", "物理", "电磁", "兼容"]
}

impl ElectromagneticCompatibilityRules {
    /// 电磁干扰基础
    pub fn emi_basics(&self) -> Vec<&'static str> {
        vec![
            "电磁干扰(EMI): 电磁场对电子设备的不良影响",
            "干扰源: 自然干扰、人为干扰",
            "传播途径: 导线传导、空间辐射",
            "受害设备: 被干扰影响的设备",
            "干扰类型: 连续干扰、瞬态干扰",
            "干扰频段: 低频、高频、微波",
            "干扰强度: 取决于源强度和距离",
            "干扰特性: 频率、幅度、持续时间",
        ]
    }

    /// 传导干扰
    pub fn conducted_interference(&self) -> Vec<&'static str> {
        vec![
            "传导干扰: 通过导线传输的干扰",
            "共模干扰: 所有导线上的同相干扰",
            "差模干扰: 导线间的反相干扰",
            "电源线干扰: 电源线上的电磁干扰",
            "信号线干扰: 信号线上的电磁干扰",
            "地线干扰: 地线上的干扰电流",
            "传导路径: 导线、PCB走线、电缆",
            "传导抑制: 滤波器、隔离变压器",
        ]
    }

    /// 辐射干扰
    pub fn radiated_interference(&self) -> Vec<&'static str> {
        vec![
            "辐射干扰: 通过空间传播的干扰",
            "近场干扰: 近距离辐射干扰",
            "远场干扰: 远距离辐射干扰",
            "天线效应: 导线作为天线辐射",
            "PCB辐射: PCB走线辐射干扰",
            "机箱泄漏: 机箱缝隙辐射",
            "电缆辐射: 电缆作为辐射源",
            "辐射抑制: 屏蔽、布局优化",
        ]
    }

    /// 屏蔽技术
    pub fn shielding_techniques(&self) -> Vec<&'static str> {
        vec![
            "电场屏蔽: 利用导体屏蔽电场",
            "磁场屏蔽: 利用高导磁材料屏蔽磁场",
            "电磁屏蔽: 金属外壳屏蔽电磁波",
            "屏蔽效能: SE = 20log(E₁/E₂) dB",
            "反射损耗: 金属表面反射电磁波",
            "吸收损耗: 材料吸收电磁波能量",
            "多次反射: 屏蔽材料内部多次反射",
            "缝隙处理: 防止缝隙泄漏",
        ]
    }

    /// 滤波技术
    pub fn filtering_techniques(&self) -> Vec<&'static str> {
        vec![
            "滤波器: 抑制特定频率干扰",
            "低通滤波: 抑制高频干扰",
            "高通滤波: 抑制低频干扰",
            "带通滤波: 允许特定频段通过",
            "带阻滤波: 阻止特定频段通过",
            "电源滤波: 电源线上的滤波器",
            "信号滤波: 信号线上的滤波器",
            "EMI滤波器: 专用电磁干扰滤波器",
        ]
    }

    /// 接地技术
    pub fn grounding_techniques(&self) -> Vec<&'static str> {
        vec![
            "安全接地: 保护人身安全",
            "信号接地: 为信号提供参考电位",
            "屏蔽接地: 屏蔽层接地提高效能",
            "单点接地: 所有接地接到一点",
            "多点接地: 多处接大地",
            "混合接地: 不同频率采用不同接地",
            "浮地: 不接大地的接地方式",
            "接地电阻: 接地导线的电阻",
        ]
    }

    /// EMC标准
    pub fn emc_standards(&self) -> Vec<&'static str> {
        vec![
            "国际标准: IEC/CISPR标准",
            "欧盟标准: EN标准（CE认证）",
            "美国标准: FCC标准",
            "中国标准: GB标准",
            "发射标准: 设备发射限值",
            "抗扰标准: 设备抗扰等级",
            "测试方法: 标准规定的测试方法",
            "认证程序: EMC认证流程",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "电源设计: EMC电源设计",
            "PCB设计: EMC PCB布局",
            "线缆设计: EMC线缆设计",
            "机箱设计: EMC机箱设计",
            "芯片设计: EMC芯片设计",
            "系统设计: EMC系统集成",
            "产品认证: EMC测试认证",
            "现场整改: EMC问题整改",
        ]
    }
}

impl Rule for ElectromagneticCompatibilityRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("electromagnetic_compatibility")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "电磁兼容规则",
            &[
                ("电磁干扰基础", &self.emi_basics()),
                ("传导干扰", &self.conducted_interference()),
                ("辐射干扰", &self.radiated_interference()),
                ("屏蔽技术", &self.shielding_techniques()),
                ("滤波技术", &self.filtering_techniques()),
                ("接地技术", &self.grounding_techniques()),
                ("EMC标准", &self.emc_standards()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electromagnetic_compatibility_rules() {
        let rules = ElectromagneticCompatibilityRules::new();
        assert_eq!(rules.metadata().name, "电磁兼容规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.emi_basics().is_empty());
        assert!(!rules.shielding_techniques().is_empty());
    }
}
