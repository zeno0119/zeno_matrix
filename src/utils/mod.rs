type Matrix<T> = super::Matrix<T>;

impl<T> Matrix<T>
where T: std::clone::Clone
{
    pub(crate) fn repeat(&self, num: usize) -> Vec<T> {
        let mut data = Vec::<T>::new();
        for _i in 0..num {
            data.append(&mut self.data.clone());
        }
        data
    }

    pub(crate) fn duplicate(&self, num: usize) -> Vec<T> {
        let mut data = Vec::<T>::new();
        for datum in self.data.clone() {
            data.append(&mut vec![datum; num]);
        }

        data
    }

    pub(crate) fn dimensional_broadcast(a: Matrix<T>, b: Matrix<T>) -> (Matrix<T>, Matrix<T>){
        let b = if a.dim.len() > b.dim.len(){
            let mut dim = b.dim.clone();
            dim.append(&mut vec![1;a.dim.len() - b.dim.len()]);
            Matrix{data: b.data.clone(), dim, size: b.size}
        }else {
            b
        };

        let a = if b.dim.len() > a.dim.len(){
            let mut dim = a.dim.clone();
            dim.append(&mut vec![1;b.dim.len() - a.dim.len()]);
            Matrix{data: a.data.clone(), dim, size: a.size}
        }else {
            a
        };
        (a, b)
    }

    pub fn vec(&self) -> Vec<T>{
        self.data.clone()
    }

}