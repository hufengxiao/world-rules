//! 遗传学定律

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 遗传学定律集合
pub struct GeneticsLaws {
    metadata: RuleMetadata,
}

impl GeneticsLaws {
    pub fn new() -> Self {
        Self {
            metadata: RuleMetadata::new("遗传学定律", "遗传学基本定律")
                .with_origin("生物学")
                .with_tags(vec!["科学".into(), "生物".into(), "遗传".into()]),
        }
    }

    /// 孟德尔定律
    pub fn mendelian_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("分离定律", "等位基因分离", "一对等位基因分离"),
            ("自由组合定律", "独立分配", "不同基因自由组合"),
            ("显隐性定律", "显性隐性", "基因显隐性关系"),
            ("基因型定律", "基因组合", "个体基因组成"),
            ("表现型定律", "性状表现", "基因表现特征"),
            ("纯合定律", "相同等位基因", "等位基因相同"),
            ("杂合定律", "不同等位基因", "等位基因不同"),
        ]
    }

    /// 遗传模式定律
    pub fn inheritance_patterns(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("常染色体显性遗传", "代代相传", "显性基因遗传"),
            ("常染色体隐性遗传", "隔代遗传", "隐性基因遗传"),
            ("X连锁遗传", "性别差异", "X染色体基因遗传"),
            ("Y连锁遗传", "男性传递", "Y染色体基因遗传"),
            ("伴性遗传定律", "性别相关", "性别相关遗传"),
            ("限性遗传定律", "特定性别", "只在特定性别表现"),
            ("多基因遗传定律", "多基因控制", "多个基因共同作用"),
            ("母系遗传定律", "线粒体遗传", "线粒体DNA遗传"),
        ]
    }

    /// 染色体定律
    pub fn chromosome_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("染色体数目定律", "恒定数目", "物种染色体数目恒定"),
            ("染色体结构定律", "DNA蛋白质", "染色体组成"),
            ("同源染色体定律", "成对存在", "同源染色体配对"),
            ("染色体分离定律", "减数分离", "减数分裂染色体分离"),
            ("连锁定律", "基因连锁", "同染色体基因连锁"),
            ("交换定律", "交叉互换", "同源染色体交换"),
            ("染色体变异定律", "数目结构变异", "染色体异常"),
        ]
    }

    /// 分子遗传定律
    pub fn molecular_genetics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("中心法则定律", "DNA→RNA→蛋白质", "遗传信息流向"),
            ("DNA复制定律", "半保留复制", "DNA复制机制"),
            ("转录定律", "RNA合成", "DNA转录成RNA"),
            ("翻译定律", "蛋白质合成", "RNA翻译成蛋白质"),
            ("逆转录定律", "RNA→DNA", "逆转录病毒"),
            ("密码定律", "三联体密码", "遗传密码"),
            ("基因突变定律", "序列改变", "DNA序列突变"),
            ("基因重组定律", "序列交换", "基因序列重组"),
        ]
    }

    /// 群体遗传定律
    pub fn population_genetics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("哈迪-温伯格定律", "基因频率恒定", "理想群体基因频率"),
            ("基因频率定律", "p+q=1", "等位基因频率"),
            ("基因型频率定律", "p²+2pq+q²=1", "基因型频率分布"),
            ("自然选择定律", "频率改变", "选择改变基因频率"),
            ("基因漂变定律", "随机变化", "随机因素改变频率"),
            ("迁移定律", "基因流动", "种群迁移基因流动"),
            ("突变定律", "新等位基因", "突变产生新基因"),
        ]
    }

    /// 遗传现象
    pub fn phenomena(&self) -> Vec<&'static str> {
        vec![
            "不完全显性",
            "共显性",
            "上位效应",
            "多效性",
            "遗传印记",
            "嵌合体",
            "三体综合征",
            "基因沉默",
            "表观遗传",
            "嵌合遗传",
        ]
    }

    /// 表观遗传定律
    pub fn epigenetics_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("DNA甲基化定律", "基因沉默", "DNA甲基化抑制基因"),
            ("组蛋白修饰定律", "染色质重塑", "组蛋白修饰影响表达"),
            ("染色质重塑定律", "结构变化", "染色质结构变化"),
            ("非编码RNA定律", "调控RNA", "非编码RNA调控基因"),
            ("印记定律", "亲本印记", "基因印记表达"),
        ]
    }

    /// 基因工程定律
    pub fn genetic_engineering_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("重组DNA定律", "基因拼接", "DNA重组技术"),
            ("基因克隆定律", "基因复制", "基因克隆扩增"),
            ("转基因定律", "基因导入", "外源基因导入"),
            ("基因编辑定律", "CRISPR", "基因编辑技术"),
            ("基因治疗定律", "基因修复", "基因治疗疾病"),
        ]
    }

    /// 发育遗传定律
    pub fn developmental_laws(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("Hox基因定律", "体节发育", "Hox基因控制体节"),
            ("形态发生定律", "形态形成", "胚胎形态发生"),
            ("细胞分化定律", "命运决定", "细胞命运决定"),
            ("信号梯度定律", "浓度梯度", "信号分子浓度梯度"),
            ("同源异形定律", "基因转换", "同源异形基因突变"),
        ]
    }

    /// 遗传技术
    pub fn techniques(&self) -> Vec<&'static str> {
        vec![
            "基因克隆",
            "转基因技术",
            "基因编辑",
            "基因测序",
            "PCR技术",
            "基因芯片",
            "基因组学",
            "蛋白质组学",
        ]
    }

    /// 孟德尔遗传
    pub fn mendelian_genetics(&self) -> Vec<&'static str> {
        vec![
            "分离定律: 等位基因在形成配子时彼此分离",
            "自由组合定律: 非等位基因自由组合",
            "显隐性关系: 显性等位基因掩盖隐性等位基因的表达",
            "不完全显性: 杂合子表现中间型性状",
            "共显性: 两个等位基因同时表达",
            "连锁遗传: 同一染色体上的基因倾向于一起遗传",
        ]
    }

    /// 分子遗传学
    pub fn molecular_genetics(&self) -> Vec<&'static str> {
        vec![
            "基因表达: DNA转录为RNA再翻译为蛋白质",
            "基因突变: DNA序列的改变包括点突变和插入缺失",
            "基因调控: 操纵子模型等基因表达调控机制",
            "表观遗传: 不改变DNA序列的可遗传基因表达变化",
            "RNA干扰: 小RNA分子沉默特定基因表达",
            "基因组印记: 父源和母源等位基因差异表达",
            "转座子: 基因组中可移动的DNA片段",
        ]
    }
}

impl Default for GeneticsLaws {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for GeneticsLaws {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("genetics")
    }

    fn explain(&self) -> String {
        format!(
            "【遗传学定律】\n\n孟德尔定律:\n{}\n\n染色体定律:\n{}\n\n分子遗传定律:\n{}\n\n表观遗传定律:\n{}\n\n基因工程定律:\n{}\n\n发育遗传定律:\n{}\n",
            self.mendelian_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.chromosome_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.molecular_genetics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.epigenetics_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.genetic_engineering_laws().iter()
                .map(|(name, formula, desc)| format!("▶ {}: {} - {}", name, formula, desc))
                .collect::<Vec<_>>()
                .join("\n"),
            self.developmental_laws().iter()
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
    fn test_genetics_laws() {
        let laws = GeneticsLaws::new();
        assert!(!laws.mendelian_laws().is_empty());
        assert!(!laws.molecular_genetics_laws().is_empty());
    }
}
