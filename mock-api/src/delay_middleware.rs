use pin_project_lite::pin_project;
use std::time::Duration;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::Sleep;
use tower::{Layer, Service};

/// Applies a Delay to requests via the supplied inner service.
#[derive(Debug, Clone)]
pub struct DelayLayer {
    delay: Duration,
}

impl DelayLayer {
    pub const fn new(delay: Duration) -> Self {
        DelayLayer { delay }
    }
}

impl<S> Layer<S> for DelayLayer {
    type Service = Delay<S>;

    fn layer(&self, service: S) -> Self::Service {
        Delay::new(service, self.delay)
    }
}

pin_project! {
    #[derive(Debug)]
    pub struct ResponseFuture<T> {
        #[pin]
        response: T,
        #[pin]
        sleep: Sleep,
    }
}

impl<T> ResponseFuture<T> {
    pub(crate) fn new(response: T, sleep: Sleep) -> Self {
        ResponseFuture { response, sleep }
    }
}

impl<F, T, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<T, E>>,
{
    type Output = Result<T, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.sleep.poll(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(_) => {}
        }

        match this.response.poll(cx) {
            Poll::Ready(v) => Poll::Ready(v.map_err(Into::into)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Delay<T> {
    inner: T,
    delay: Duration,
}

impl<T> Delay<T> {
    pub const fn new(inner: T, delay: Duration) -> Self {
        Delay { inner, delay }
    }

    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<S, Request> Service<Request> for Delay<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match self.inner.poll_ready(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(r) => Poll::Ready(r.map_err(Into::into)),
        }
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let response = self.inner.call(request);
        let sleep = tokio::time::sleep(self.delay);

        ResponseFuture::new(response, sleep)
    }
}
