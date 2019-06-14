#![feature(test)]

#[macro_use]
extern crate tokio_trace;
extern crate test;
use std::{
    fmt,
    sync::{Mutex, MutexGuard},
};
use test::Bencher;
use tokio_trace::{field, span, Event, Id, Metadata};
use trace_limit::LimitSubscriber;

// #[bench]
// fn baseline(b: &mut Bencher) {
//     tokio_trace::subscriber::with_default(EnabledSubscriber, || b.iter(|| info!("hello world")));
// }

#[bench]
fn baseline_record(b: &mut Bencher) {
    let sub = VisitingSubscriber(Mutex::new(String::from("")));
    let n = test::black_box(1000);
    tokio_trace::subscriber::with_default(sub, || {
        b.iter(|| {
            for _ in 0..n {
                info!(
                    message = "hello world",
                    foo = "foo",
                    bar = "bar",
                    baz = 3,
                    quuux = tokio_trace::field::debug(0.99)
                )
            }
        })
    });
}

#[bench]
fn limit_record_5(b: &mut Bencher) {
    let sub = LimitSubscriber::new(VisitingSubscriber(Mutex::new(String::from(""))));
    let n = test::black_box(1000);
    tokio_trace::subscriber::with_default(sub, || {
        b.iter(|| {
            for _ in 0..n {
                info!(
                    message = "hello world",
                    foo = "foo",
                    bar = "bar",
                    baz = 3,
                    quuux = tokio_trace::field::debug(0.99),
                    rate_limit = 5
                )
            }
        })
    });
}

#[bench]
fn limit_record_50(b: &mut Bencher) {
    let sub = LimitSubscriber::new(VisitingSubscriber(Mutex::new(String::from(""))));
    let n = test::black_box(1000);
    tokio_trace::subscriber::with_default(sub, || {
        b.iter(|| {
            for _ in 0..n {
                info!(
                    message = "hello world",
                    foo = "foo",
                    bar = "bar",
                    baz = 3,
                    quuux = tokio_trace::field::debug(0.99),
                    rate_limit = 50
                )
            }
        })
    });
}

#[bench]
fn limit_record_100(b: &mut Bencher) {
    let sub = LimitSubscriber::new(VisitingSubscriber(Mutex::new(String::from(""))));
    let n = test::black_box(1000);
    tokio_trace::subscriber::with_default(sub, || {
        b.iter(|| {
            for _ in 0..n {
                info!(
                    message = "hello world",
                    foo = "foo",
                    bar = "bar",
                    baz = 3,
                    quuux = tokio_trace::field::debug(0.99),
                    rate_limit = 100
                )
            }
        })
    });
}

// #[bench]
// fn limit_record_50(b: &mut Bencher) {
//     let sub = LimitSubscriber::new(VisitingSubscriber(Mutex::new(String::from(""))));
//     tokio_trace::subscriber::with_default(sub, || {
//         b.iter(|| {
//             info!(
//                 message = "hello world",
//                 foo = "foo",
//                 bar = "bar",
//                 baz = 3,
//                 quuux = tokio_trace::field::debug(0.99),
//                 rate_limit = 50
//             )
//         })
//     });
// }

// #[bench]
// fn limit_record_100(b: &mut Bencher) {
//     let sub = LimitSubscriber::new(VisitingSubscriber(Mutex::new(String::from(""))));
//     tokio_trace::subscriber::with_default(sub, || {
//         b.iter(|| {
//             info!(
//                 message = "hello world",
//                 foo = "foo",
//                 bar = "bar",
//                 baz = 3,
//                 quuux = tokio_trace::field::debug(0.99),
//                 rate_limt = 100
//             )
//         })
//     });
// }

// #[bench]
// fn limit_record_1_000(b: &mut Bencher) {
//     let sub = LimitSubscriber::new(VisitingSubscriber(Mutex::new(String::from(""))));
//     tokio_trace::subscriber::with_default(sub, || {
//         b.iter(|| {
//             info!(
//                 message = "hello world",
//                 foo = "foo",
//                 bar = "bar",
//                 baz = 3,
//                 quuux = tokio_trace::field::debug(0.99),
//                 rate_limit = 1_000
//             )
//         })
//     });
// }

// #[bench]
// fn limit_record_100_000(b: &mut Bencher) {
//     let sub = LimitSubscriber::new(VisitingSubscriber(Mutex::new(String::from(""))));
//     tokio_trace::subscriber::with_default(sub, || {
//         b.iter(|| {
//             info!(
//                 message = "hello world",
//                 foo = "foo",
//                 bar = "bar",
//                 baz = 3,
//                 quuux = tokio_trace::field::debug(0.99),
//                 rate_limit = 100_000
//             )
//         })
//     });
// }

// #[bench]
// fn limit_5(b: &mut Bencher) {
//     let sub = LimitSubscriber::new(EnabledSubscriber);
//     tokio_trace::subscriber::with_default(sub, || {
//         b.iter(|| info!(message = "hello world", rate_limit = 5))
//     });
// }

// #[bench]
// fn limit_50(b: &mut Bencher) {
//     let sub = LimitSubscriber::new(EnabledSubscriber);
//     tokio_trace::subscriber::with_default(sub, || {
//         b.iter(|| info!(message = "hello world", rate_limit = 50))
//     });
// }

// #[bench]
// fn limit_100(b: &mut Bencher) {
//     let sub = LimitSubscriber::new(EnabledSubscriber);
//     tokio_trace::subscriber::with_default(sub, || {
//         b.iter(|| info!(message = "hello world", rate_limit = 100))
//     });
// }

// #[bench]
// fn limit_1_000(b: &mut Bencher) {
//     let sub = LimitSubscriber::new(EnabledSubscriber);
//     tokio_trace::subscriber::with_default(sub, || {
//         b.iter(|| info!(message = "hello world", rate_limit = 1_000))
//     });
// }

// #[bench]
// fn limit_100_000(b: &mut Bencher) {
//     let sub = LimitSubscriber::new(EnabledSubscriber);
//     tokio_trace::subscriber::with_default(sub, || {
//         b.iter(|| info!(message = "hello world", rate_limit = 100_000))
//     });
// }

/// A subscriber that is enabled but otherwise does nothing.
struct EnabledSubscriber;

impl tokio_trace::Subscriber for EnabledSubscriber {
    fn new_span(&self, span: &span::Attributes) -> Id {
        let _ = span;
        Id::from_u64(0xDEADFACE)
    }

    fn event(&self, event: &Event) {
        let _ = event;
    }

    fn record(&self, span: &Id, values: &span::Record) {
        let _ = (span, values);
    }

    fn record_follows_from(&self, span: &Id, follows: &Id) {
        let _ = (span, follows);
    }

    fn enabled(&self, metadata: &Metadata) -> bool {
        let _ = metadata;
        true
    }

    fn enter(&self, span: &Id) {
        let _ = span;
    }

    fn exit(&self, span: &Id) {
        let _ = span;
    }
}

/// Simulates a subscriber that records span data.
struct VisitingSubscriber(Mutex<String>);

struct Visitor<'a>(MutexGuard<'a, String>);

impl<'a> field::Visit for Visitor<'a> {
    fn record_debug(&mut self, _field: &field::Field, value: &dyn fmt::Debug) {
        use std::fmt::Write;
        let _ = write!(&mut *self.0, "{:?}", value);
    }
}

impl tokio_trace::Subscriber for VisitingSubscriber {
    fn new_span(&self, span: &span::Attributes) -> Id {
        let mut visitor = Visitor(self.0.lock().unwrap());
        span.record(&mut visitor);
        Id::from_u64(0xDEADFACE)
    }

    fn record(&self, _span: &Id, values: &span::Record) {
        let mut visitor = Visitor(self.0.lock().unwrap());
        values.record(&mut visitor);
    }

    fn event(&self, event: &Event) {
        let mut visitor = Visitor(self.0.lock().unwrap());
        event.record(&mut visitor);
    }

    fn record_follows_from(&self, span: &Id, follows: &Id) {
        let _ = (span, follows);
    }

    fn enabled(&self, metadata: &Metadata) -> bool {
        let _ = metadata;
        true
    }

    fn enter(&self, span: &Id) {
        let _ = span;
    }

    fn exit(&self, span: &Id) {
        let _ = span;
    }
}
