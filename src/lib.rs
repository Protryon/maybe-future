use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pin_project_lite::pin_project! {
    pub struct MaybeFuture<O, F: Future<Output=O>> {
        #[pin]
        inner: Option<F>,
    }
}

impl<O, F: Future<Output = O>> MaybeFuture<O, F> {
    pub fn new(inner: Option<F>) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> Option<F> {
        self.inner
    }

    pub fn inner(&self) -> &Option<F> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Option<F> {
        &mut self.inner
    }

    pub fn reset(self: Pin<&mut Self>, value: Option<F>) {
        let mut this = self.project();
        this.inner.set(value);
    }
}

impl<O, F: Future<Output = O>> Future for MaybeFuture<O, F> {
    type Output = O;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match this.inner.as_pin_mut() {
            Some(inner) => inner.poll(cx),
            None => Poll::Pending,
        }
    }
}
