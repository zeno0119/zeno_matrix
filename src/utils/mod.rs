type Matrix<T> = super::Matrix<T>;

impl<T> Matrix<T>
where T: std::clone::Clone
{
    pub(crate) fn repeat(&self, num: usize) -> Vec<T> {
        let mut data = Vec::<T>::new();
        for i in 0..num {
            data.append(&mut self.data.clone());
        }
        return data;
    }

    pub(crate) fn duplicate(&self, num: usize) -> Vec<T> {
        let mut data = Vec::<T>::new();
        for datum in self.data.clone() {
            data.append(&mut vec![datum; num]);
        }

        return data;
    }
}