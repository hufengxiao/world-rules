//! 生物化学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult};

/// 生物化学定律集合
pub struct BiochemistryLaws {
    metadata: RuleMetadata,
}

impl BiochemistryLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("生物化学定律", "生物化学基本定律")
                .with_origin("化学")
                .with_tags(vec!["科学".into(), "化学".into(), "生物".into()]),
        }
    }

    /// 代谢定律
    pub fn metabolism_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("糖代谢定律", "葡萄糖分解", "糖类物质代谢过程"),
            ("糖酵解定律", "ATP生成", "葡萄糖分解产生ATP"),
            ("三羧酸循环定律", "TCA循环", "柠檬酸循环"),
            ("氧化磷酸化定律", "电子传递", "ATP合成"),
            ("脂肪酸代谢定律", "β氧化", "脂肪酸分解"),
            ("氨基酸代谢定律", "脱氨基", "氨基酸代谢"),
            ("核酸代谢定律", "核苷酸代谢", "核酸合成分解"),
            ("呼吸定律", "能量释放", "细胞呼吸过程"),
        ]
    }

    /// 酶学定律
    pub fn enzyme_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("米氏方程定律", "v = Vmax[S]/(Km+[S])", "酶反应速率方程"),
            ("酶催化定律", "催化效率", "酶催化效率极高"),
            ("酶特异性定律", "专一性", "酶对底物专一"),
            ("酶活性定律", "条件依赖", "酶活性依赖条件"),
            ("酶抑制定律", "抑制剂效应", "抑制剂降低酶活性"),
            ("酶激活定律", "激活剂效应", "激活剂提高酶活性"),
            ("酶动力学定律", "反应速率", "酶催化反应动力学"),
            ("酶调节定律", "调节机制", "酶活性调节机制"),
        ]
    }

    /// 蛋白质定律
    pub fn protein_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("蛋白质结构定律", "四级结构", "蛋白质四级结构"),
            ("一级结构定律", "氨基酸序列", "氨基酸排列顺序"),
            ("二级结构定律", "α螺旋β折叠", "蛋白质局部结构"),
            ("三级结构定律", "三维结构", "蛋白质三维形状"),
            ("四级结构定律", "亚基组装", "多亚基蛋白质组装"),
            ("蛋白质折叠定律", "折叠规则", "蛋白质折叠规律"),
            ("变性定律", "结构破坏", "蛋白质结构破坏"),
            ("蛋白质功能定律", "结构决定", "结构决定功能"),
        ]
    }

    /// DNA/RNA定律
    pub fn nucleic_acid_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("DNA结构定律", "双螺旋", "DNA双螺旋结构"),
            ("碱基配对定律", "A-T G-C", "碱基互补配对"),
            ("DNA复制定律", "半保留复制", "DNA复制机制"),
            ("转录定律", "mRNA合成", "DNA转录成RNA"),
            ("翻译定律", "蛋白质合成", "mRNA翻译成蛋白质"),
            ("基因表达定律", "表达调控", "基因表达调控"),
            ("密码定律", "三联体密码", "遗传密码规则"),
            ("突变定律", "DNA变异", "DNA序列变异"),
        ]
    }

    /// 生物能量定律
    pub fn energy_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("ATP定律", "能量货币", "ATP作为能量载体"),
            ("能量守恒定律", "生物能量守恒", "生物系统能量守恒"),
            ("能量转化定律", "形式转换", "能量形式转换"),
            ("自由能定律", "ΔG", "反应自发判据"),
            ("氧化还原定律", "电子传递", "生物氧化还原"),
            ("能量效率定律", "转化效率", "能量转化效率"),
        ]
    }

    /// 生物分子类型
    pub fn biomolecules(&self) -> Vec<&'static str> {
        vec![
            "蛋白质",
            "核酸",
            "碳水化合物",
            "脂质",
            "维生素",
            "辅酶",
            "激素",
            "酶",
        ]
    }

    /// 信号传导定律
    pub fn signaling_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("受体信号定律", "信号接收", "细胞表面受体信号传导"),
            ("G蛋白定律", "信号转导", "G蛋白偶联受体信号"),
            ("第二信使定律", "信号放大", "cAMP等第二信使"),
            ("激酶级联定律", "磷酸化", "激酶级联放大信号"),
            ("转录因子定律", "基因调控", "转录因子调控基因表达"),
        ]
    }

    /// 膜生物学定律
    pub fn membrane_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("流动镶嵌定律", "膜模型", "细胞膜流动镶嵌模型"),
            ("膜通透性定律", "选择通透", "膜的选择性通透"),
            ("膜电位定律", "电位差", "细胞膜内外电位差"),
            ("离子通道定律", "离子转运", "离子通道转运离子"),
            ("膜融合定律", "囊泡融合", "囊泡与膜融合"),
        ]
    }

    /// 分子生物学定律
    pub fn molecular_biology_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("操纵子定律", "基因调控", "原核生物基因调控"),
            ("转录调控定律", "启动子", "转录水平调控"),
            ("RNA剪接定律", "内含子", "RNA剪接加工"),
            ("蛋白质修饰定律", "翻译后修饰", "蛋白质翻译后修饰"),
            ("泛素化定律", "蛋白降解", "泛素标记蛋白降解"),
        ]
    }

    /// 生物化学技术
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "色谱法",
            "电泳",
            "光谱法",
            "质谱",
            "PCR",
            "基因克隆",
            "蛋白质纯化",
            "DNA测序",
        ]
    }

    /// 蛋白质结构
    pub fn protein_structure(&self) -> Vec<&'static str> {
        vec![
            "氨基酸: 20种标准氨基酸是蛋白质的基本构件",
            "一级结构: 氨基酸的线性排列顺序",
            "二级结构: α螺旋和β折叠等局部空间构象",
            "三级结构: 整条多肽链的三维空间结构",
            "四级结构: 多个亚基的组装方式",
            "分子伴侣: 帮助蛋白质正确折叠的辅助蛋白",
            "蛋白质变性: 理化因素破坏高级结构导致失活",
        ]
    }

    /// 酶动力学
    pub fn enzyme_kinetics(&self) -> Vec<&'static str> {
        vec![
            "米氏方程: v=Vmax[S]/(Km+[S])描述底物浓度与反应速度",
            "酶的专一性: 酶对底物的选择性",
            "竞争性抑制: 抑制剂与底物竞争酶的活性中心",
            "非竞争性抑制: 抑制剂结合在酶的其他部位",
            "别构调节: 调节物结合在酶的别构位点改变活性",
            "酶原激活: 无活性前体经蛋白酶水解变为有活性",
            "辅酶: 与酶结合的有机小分子如NAD⁺和FAD",
        ]
    }
}

impl Default for BiochemistryLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for BiochemistryLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("biochemistry")
    }

    fn validate(&self, context: &str) -> RuleResult<bool> {
        Ok(!context.is_empty())
    }

    fn explain(&self) -> String {
        format!(
            "【生物化学定律】\n\n代谢定律:\n{}\n\n酶学定律:\n{}\n\n蛋白质定律:\n{}\n\n信号传导定律:\n{}\n\n膜生物学定律:\n{}\n\n分子生物学定律:\n{}\n",
            self.metabolism_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.enzyme_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.protein_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.signaling_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.membrane_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.molecular_biology_laws().iter()
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
    fn test_biochemistry_laws() {
        let laws = BiochemistryLaws::new();
        assert!(!laws.metabolism_laws().is_empty());
        assert!(!laws.enzyme_laws().is_empty());
    }
}
