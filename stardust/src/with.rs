// #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
// pub struct With<T, R = ()> {
//     pub inner: T,
//     pub related: R,
// }

// impl<T, R> With<T, R> {
//     pub fn new(inner: T, related: R) -> Self {
//         Self { inner, related }
//     }

//     pub fn into_inner(self) -> T {
//         self.inner
//     }

//     pub fn into_related(self) -> R {
//         self.related
//     }

//     pub fn into_parts(self) -> (T, R) {
//         (self.inner, self.related)
//     }

//     pub fn as_inner(&self) -> &T {
//         &self.inner
//     }

//     pub fn as_related(&self) -> &R {
//         &self.related
//     }

//     pub fn map_inner<T1>(self, f: impl FnOnce(T) -> T1) -> With<T1, R> {
//         With {
//             inner: f(self.inner),
//             related: self.related,
//         }
//     }

//     pub fn map_related<R1>(self, f: impl FnOnce(R) -> R1) -> With<T, R1> {
//         With {
//             inner: self.inner,
//             related: f(self.related),
//         }
//     }
// }

// impl<T, R> std::ops::Deref for With<T, R> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// impl<T, R> std::ops::DerefMut for With<T, R> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.inner
//     }
// }
