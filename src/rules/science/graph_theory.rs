//! 图论规则
//!
//! 图论研究图的结构和性质，是离散数学的重要分支。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: GraphTheoryRules,
    name: "图论规则",
    desc: "图的性质、算法与应用",
    origin: "数学",
    tags: ["科学", "数学", "图论"]
}

impl GraphTheoryRules {
    /// 图的基本概念
    pub fn graph_basics(&self) -> Vec<&'static str> {
        vec![
            "图定义: G = (V, E)，顶点集 V 和边集 E",
            "有向图: 边有方向的图",
            "无向图: 边无方向的图",
            "简单图: 无自环和无重边的图",
            "顶点度数: 与该顶点相连的边数",
            "路径: 顶点序列，相邻顶点有边连接",
            "回路: 起点和终点相同的路径",
            "连通图: 任意两顶点有路径连接",
        ]
    }

    /// 特殊图类型
    pub fn special_graphs(&self) -> Vec<&'static str> {
        vec![
            "完全图 K_n: 每对顶点都有边",
            "二部图: 顶点分为两部分，边只连接不同部分",
            "树: 无回路且连通的图",
            "森林: 无回路的图（可能不连通）",
            "环图 C_n: n 个顶点形成环",
            "星图 S_n: 一个中心顶点连接其他 n-1 个顶点",
            "平面图: 可画在平面且边不相交",
            "欧拉图: 有经过每条边一次的回路",
        ]
    }

    /// 图的矩阵表示
    pub fn matrix_representation(&self) -> Vec<&'static str> {
        vec![
            "邻接矩阵: A[i,j] = 1 表示有边 i→j",
            "邻接表: 每个顶点的邻居列表",
            "关联矩阵: 表示顶点与边的关联关系",
            "距离矩阵: 表示顶点间最短距离",
            "矩阵性质: 对称性、幂运算",
            "谱图理论: 用矩阵特征值研究图",
            "邻接矩阵幂: A^k 表示长度 k 的路径数",
            "存储效率: 邻接表更节省空间",
        ]
    }

    /// 图的连通性
    pub fn connectivity(&self) -> Vec<&'static str> {
        vec![
            "连通分量: 极大连通子图",
            "强连通: 有向图中任意两顶点互相可达",
            "弱连通: 忽略方向后连通",
            "割点: 删除后图不再连通的顶点",
            "桥: 删除后图不再连通的边",
            "连通度: κ(G) 表示顶点连通度",
            "边连通度: λ(G) 表示边连通度",
            "Menger 定理: 连通度与路径数的等价性",
        ]
    }

    /// 匹配与覆盖
    pub fn matching_and_covering(&self) -> Vec<&'static str> {
        vec![
            "匹配: 边集，任意两条边无公共顶点",
            "完美匹配: 每个顶点都在匹配中",
            "最大匹配: 边数最多的匹配",
            "Hall 定理: 二部图有完美匹配的条件",
            "顶点覆盖: 顶点集覆盖所有边",
            "边覆盖: 边集覆盖所有顶点",
            "König 定理: 二部图的最大匹配等于最小覆盖",
            "应用: 分配问题、调度问题",
        ]
    }

    /// 着色问题
    pub fn coloring(&self) -> Vec<&'static str> {
        vec![
            "顶点着色: 相邻顶点不同色",
            "色数: χ(G)，最少需要的颜色数",
            "边着色: 相邻边不同色",
            "边色数: χ'(G)",
            "Vizing 定理: χ'(G) ≤ Δ(G) + 1",
            "平面图着色: χ ≤ 4（四色定理）",
            "色多项式: 用 k 种颜色的着色数",
            "应用: 课程安排、寄存器分配",
        ]
    }

    /// 著名问题
    pub fn famous_problems(&self) -> Vec<&'static str> {
        vec![
            "最短路径: Dijkstra、Bellman-Ford 算法",
            "最小生成树: Kruskal、Prim 算法",
            "哈密尔顿回路: 经过每个顶点一次",
            "欧拉回路: 经过每条边一次",
            "图的平面性判定: Kuratowski 定理",
            "图的同构判定: 结构相同",
            "旅行商问题 TSP: NP-hard",
            "最大流问题: Ford-Fulkerson 算法",
        ]
    }

    /// 树的性质
    pub fn tree_properties(&self) -> Vec<&'static str> {
        vec![
            "树的定义: 连通无回路图",
            "树的边数: |E| = |V| - 1",
            "树的性质: 删除任意边不连通",
            "树的性质: 添加任意边产生回路",
            "生成树: 包含所有顶点的树",
            "最小生成树: 权值最小的生成树",
            "根树: 有根节点的树",
            "树的遍历: DFS、BFS",
        ]
    }

    /// 图算法
    pub fn graph_algorithms(&self) -> Vec<&'static str> {
        vec![
            "DFS 深度优先搜索: 深入探索",
            "BFS 广度优先搜索: 逐层探索",
            "拓扑排序: 有向无环图的排序",
            "连通分量算法: 找出所有连通分量",
            "最短路径算法: Dijkstra、Floyd-Warshall",
            "最小生成树算法: Kruskal、Prim",
            "匹配算法: Hungarian 算法",
            "流算法: 网络流、匹配",
        ]
    }

    /// 应用领域
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "计算机网络: 网络拓扑、路由",
            "社交网络: 关系分析、影响力",
            "化学: 分子结构、化学键",
            "生物学: 蛋白质相互作用网络",
            "运筹学: 调度、分配、优化",
            "地图与导航: 路径规划",
            "编译器: 数据流分析",
            "电子电路: 电路设计、布线",
        ]
    }
}

impl Rule for GraphTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("graph_theory")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "图论规则",
            &[
                ("图的基本概念", &self.graph_basics()),
                ("特殊图类型", &self.special_graphs()),
                ("图的矩阵表示", &self.matrix_representation()),
                ("图的连通性", &self.connectivity()),
                ("匹配与覆盖", &self.matching_and_covering()),
                ("着色问题", &self.coloring()),
                ("著名问题", &self.famous_problems()),
                ("树的性质", &self.tree_properties()),
                ("图算法", &self.graph_algorithms()),
                ("应用领域", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_theory_rules() {
        let rules = GraphTheoryRules::new();
        assert_eq!(rules.metadata().name, "图论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.graph_basics().is_empty());
        assert!(!rules.special_graphs().is_empty());
        assert!(!rules.matrix_representation().is_empty());
        assert!(!rules.connectivity().is_empty());
        assert!(!rules.matching_and_covering().is_empty());
        assert!(!rules.coloring().is_empty());
        assert!(!rules.famous_problems().is_empty());
        assert!(!rules.tree_properties().is_empty());
        assert!(!rules.graph_algorithms().is_empty());
        assert!(!rules.applications().is_empty());
    }
}
