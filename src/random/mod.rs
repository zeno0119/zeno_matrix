use rand;
use rand::Rng;

type Matrix<T> = super::Matrix<T>;

impl<T> super::Matrix<T>
where T: std::clone::Clone
{
    // 任意の行列・ベクトルからランダムにアイテムを抽出
    // 被りなし
    // 1次元のベクトルとして返却
    pub fn extract(&self, n: usize) -> Matrix<T>{
        if self.size < n {panic!("matrix size is smaller than n")}
        let mut origin = self.data.clone();
        let mut data: Vec<T> = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..n {
            let idx = rng.gen::<usize>() % origin.len();
            data.push(origin[idx].clone());
            origin.remove(idx);
        }
        Matrix{data, dim: vec![n], size: n}
    }

    // 任意の行列・ベクトルからランダムにアイテムを抽出
    // 被りあり
    // 1次元のベクトルとして返却
    pub fn select(&self, n: usize) -> Matrix<T>{
        let mut origin = self.data.clone();
        let mut data: Vec<T> = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..n {
            let idx = rng.gen::<usize>() % origin.len();
            data.push(origin[idx].clone());
        }
        Matrix{data, dim: vec![n], size: n}
    }
}