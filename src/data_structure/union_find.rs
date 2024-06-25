mod union_find {
    ///first, implement a struct that hold parent and rank.
    /// when init, each node point to its self, cause each node has itself is the parent node.
    /// two is the rank, this tank is the level of each sub-tree.
    struct UnionFind {
        parent: Vec<usize>,
        rank: Vec<usize>
    }

    impl UnionFind {
        
        fn new(size: usize) -> Self {
            UnionFind {
                parent: (0..size).collect(),
                rank: vec![0;size]
            }
        }

        ///this is find the root of every node, then point that node to the root and skip all 
        /// of its superiors, knows as path compression.
        fn find_parent(&mut self, id: usize) -> usize {
            if self.parent[id] != id {
                self.parent[id] = self.find_parent(self.parent[id]);
            }

            self.parent[id] 
        }

        ///union two set by pointing the set with lower rank to the set with higher rank.
        fn union(&mut self, x: usize, y: usize) {
            let root_x = self.find_parent(x);
            let root_y = self.find_parent(y);

            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else {
                self.parent[root_x] = root_y;
                self.rank[root_y] += 1;
            }
        }
    }

    ///LeetCode number 200
    fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }

        let rows = grid.len();
        let cols = grid[0].len();
        let mut uf = UnionFind::new(rows*cols);
        let mut count = 0;

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' {
                    //define a position to represent in 1d-array
                    let id = r * cols + c;
                    if r > 0 && grid[r - 1][c] == '1' {
                        uf.union(id, (r - 1) * cols + c);
                    }
                    if c > 0 && grid[r][c - 1] == '1' {
                        uf.union(id, r * cols + c - 1);
                    }
                    if r < rows && grid[r + 1][c] == '1' {
                        uf.union(id, (r + 1) * cols + c);
                    }
                    if c < cols && grid[r][c + 1] == '1' {
                        uf.union(id, r * cols + c + 1);
                    }
                } else {
                    count += 1;
                }
            }
        }

        let total_sets = uf.parent.iter()
            .enumerate()
            .filter(|&(i, &p)| {i == p})
            .count();
        (total_sets - count) as i32
    }
}