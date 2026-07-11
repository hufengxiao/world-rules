//! 拓扑学规则
//!
//! 拓扑学研究空间在连续变换下保持不变的性质，是现代数学的重要分支。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: TopologyRules,
    name: "拓扑学规则",
    desc: "拓扑空间、连续性与拓扑性质",
    origin: "数学",
    tags: ["科学", "数学", "拓扑"]
}

impl TopologyRules {
    /// 拓扑空间基础
    pub fn topological_space_basics(&self) -> Vec<&'static str> {
        vec![
            "拓扑定义: 集合 X 的子集族 T，包含 X、∅，对任意并、有限交封闭",
            "开集定义: 属于拓扑 T 的子集",
            "闭集定义: 补集是开集的子集",
            "拓扑空间: (X, T)，集合 X 配上拓扑 T",
            "诱导拓扑: 子集上的拓扑由原拓扑诱导",
            "粗拓扑: T₁ ⊂ T₂，T₁ 比 T₂ 粗",
            "细拓扑: T₁ ⊂ T₂，T₂ 比 T₁ 细",
            "平庸拓扑: T = {∅, X}，最粗的拓扑",
        ]
    }

    /// 连续性与同胚
    pub fn continuity_and_homeomorphism(&self) -> Vec<&'static str> {
        vec![
            "连续映射: f: X→Y，Y 的开集在 X 中的原像是开集",
            "开映射: X 的开集在 Y 中的像是开集",
            "同胚: 双射且连续，逆映射也连续",
            "拓扑性质: 同胚映射保持的性质",
            "局部同胚: 每点附近同胚",
            "嵌入: 映射将 X 嵌入 Y 的子空间",
            "覆盖映射: 每点被均匀覆盖",
            "紧映射: 紧集的像是紧集",
        ]
    }

    /// 拓扑基与子基
    pub fn bases_and_subbases(&self) -> Vec<&'static str> {
        vec![
            "拓扑基: B 的任意并生成拓扑",
            "基判定: B₁∩B₂ = ∪B₃ 对所有 x ∈ B₃",
            "子基: 有限交生成拓扑基",
            "局部基: 每点的开集族生成该点的拓扑",
            "可数基: 拓扑有可数基",
            "第二可数空间: 有可数基的拓扑空间",
            "第一可数空间: 每点有可数局部基",
            "可分空间: 有可数稠密子集",
        ]
    }

    /// 紧性
    pub fn compactness(&self) -> Vec<&'static str> {
        vec![
            "紧致空间: 每个开覆盖有有限子覆盖",
            "有限空间: 必是紧致空间",
            "Heine-Borel 定理: R^n 的有界闭集是紧集",
            "紧集性质: 紧集的闭子集是紧集",
            "紧集映射: 紧集在连续映射下的像是紧集",
            "列紧: 每个序列有收敛子列",
            "局部紧致: 每点有紧致邻域",
            "紧化: 添加点使空间成为紧致空间",
        ]
    }

    /// 连通性
    pub fn connectedness(&self) -> Vec<&'static str> {
        vec![
            "连通空间: 不能分成两个不相交非空开集",
            "连通分支: 最大连通子集",
            "道路连通: 两点间有连续道路",
            "局部连通: 每点有连通邻域",
            "离散拓扑: 每点都是开集，全不连通",
            "连通性质: 连通集的像是连通集",
            "区间连通: R 的区间都是连通集",
            "完全不连通: 连通分支都是单点",
        ]
    }

    /// 分离性公理
    pub fn separation_axioms(&self) -> Vec<&'static str> {
        vec![
            "T₀ (科尔莫戈罗夫): 不同点有不同的开集",
            "T₁ (弗雷歇): 单点集是闭集",
            "T₂ (豪斯多夫): 不同点有不相交的开邻域",
            "正则空间: 点与闭集可分离",
            "T₃: T₁ + 正则",
            "正规空间: 两闭集可分离",
            "T₄: T₁ + 正规",
            "完全正规: 所有子集可分离",
        ]
    }

    /// 重要定理
    pub fn important_theorems(&self) -> Vec<&'static str> {
        vec![
            "Brouwer 不动点定理: 球到自身的连续映射有不动点",
            "若尔当曲线定理: 简单闭曲线将平面分成内外两部分",
            "欧拉示性数: χ = V - E + F（多面体）",
            "Borsuk-Ulam 定理: 球面到平面的映射",
            "紧集上的连续函数: 必达到最大最小值",
            "Arzela-Ascoli 定理: 函数列紧的条件",
            "Urysohn 引理: 正规空间上连续函数分离闭集",
            "Tietze 扩张定理: 正规空间上连续函数可扩张",
        ]
    }

    /// 同伦与同调
    pub fn homotopy_and_homology(&self) -> Vec<&'static str> {
        vec![
            "同伦定义: 两映射可通过连续变形相互转换",
            "同伦等价: 空间可相互嵌入和收缩",
            "基本群: π₁(X,x₀)，道路的同伦类",
            "单连通: 基本群平凡",
            "覆叠空间: 局部同胚且覆盖原空间",
            "同调群: H_n(X)，刻画空间的洞",
            "贝蒂数: b_n = rank(H_n)",
            "应用: 空间的分类和性质",
        ]
    }

    /// 纤维丛与流形
    pub fn bundles_and_manifolds(&self) -> Vec<&'static str> {
        vec![
            "纤维丛: 局部像乘积空间",
            "向量丛: 纤维是向量空间",
            "切丛: 流形上所有切空间",
            "流形定义: 局部同胚于欧氏空间",
            "微分流形: 有微分结构的流形",
            "黎曼流形: 配上度量张量的微分流形",
            "图卡: 流形的局部坐标",
            "图册: 覆盖流形的图卡族",
        ]
    }

    /// 代数拓扑应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "物理学: 弦理论、量子场论",
            "数据分析: 拓扑数据分析",
            "机器人学: 运动规划、路径连通性",
            "计算机科学: 网络拓扑、分布式系统",
            "生物学: DNA拓扑、蛋白质结构",
            "几何拓扑: 三维流形分类",
            "动力系统: 拓扑动力系统",
            "经济学: 均衡存在性证明",
        ]
    }
}

impl Rule for TopologyRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("topology")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "拓扑学规则",
            &[
                ("拓扑空间基础", &self.topological_space_basics()),
                ("连续性与同胚", &self.continuity_and_homeomorphism()),
                ("拓扑基与子基", &self.bases_and_subbases()),
                ("紧性", &self.compactness()),
                ("连通性", &self.connectedness()),
                ("分离性公理", &self.separation_axioms()),
                ("重要定理", &self.important_theorems()),
                ("同伦与同调", &self.homotopy_and_homology()),
                ("纤维丛与流形", &self.bundles_and_manifolds()),
                ("代数拓扑应用", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_rules() {
        let rules = TopologyRules::new();
        assert_eq!(rules.metadata().name, "拓扑学规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.topological_space_basics().is_empty());
        assert!(!rules.continuity_and_homeomorphism().is_empty());
        assert!(!rules.bases_and_subbases().is_empty());
        assert!(!rules.compactness().is_empty());
        assert!(!rules.connectedness().is_empty());
        assert!(!rules.separation_axioms().is_empty());
        assert!(!rules.important_theorems().is_empty());
        assert!(!rules.homotopy_and_homology().is_empty());
        assert!(!rules.bundles_and_manifolds().is_empty());
        assert!(!rules.applications().is_empty());
    }
}
