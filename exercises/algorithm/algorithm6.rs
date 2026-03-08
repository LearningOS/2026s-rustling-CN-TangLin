/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, // 邻接表：adj[i] 存储节点i的所有邻接节点
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // 添加无向边：双向更新邻接表
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // DFS 辅助函数（递归核心）
    // v: 当前节点，visited: 已访问节点集合，visit_order: 访问顺序列表
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        // 1. 若当前节点已访问，直接返回（避免重复/环导致的无限递归）
        if visited.contains(&v) {
            return;
        }

        // 2. 标记当前节点为已访问，并加入访问顺序
        visited.insert(v);
        visit_order.push(v);

        // 3. 遍历当前节点的所有邻接节点，递归访问未访问的节点
        for &neighbor in &self.adj[v] {
            self.dfs_util(neighbor, visited, visit_order);
        }
    }

    // DFS 入口方法：返回访问顺序列表
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new(); // 初始化访问标记集合
        let mut visit_order = Vec::new(); // 初始化访问顺序

        // 边界：起始节点超出图的节点范围，返回空列表
        if start >= self.adj.len() {
            return visit_order;
        }

        // 调用递归辅助函数
        self.dfs_util(start, &mut visited, &mut visit_order);

        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); // 自环

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); // 非连通分支

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]);
    }
}