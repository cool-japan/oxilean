//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::object::RtObject;
use std::collections::HashMap;

use super::types::{
    ErrorHandling, IoBuffer, IoChannel, IoError, IoErrorKind, IoErrorPolicy, IoEvent, IoEventKind,
    IoExecutor, IoFileWatcher, IoLog, IoMetrics, IoMock, IoPolicy, IoRuntime, IoSessionStats,
    IoThrottle, IoValue, MockIoOp, PipePair, StringFormatter, VirtualFilesystem,
};

/// Result type for I/O operations.
pub type IoResult<T> = Result<T, IoError>;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_io_error_creation() {
        let err = IoError::new(IoErrorKind::FileNotFound, "not found");
        assert_eq!(err.kind, IoErrorKind::FileNotFound);
        assert!(err.path.is_none());
    }
    #[test]
    fn test_io_error_with_path() {
        let err = IoError::file_not_found("/tmp/test.txt");
        assert_eq!(err.kind, IoErrorKind::FileNotFound);
        assert_eq!(err.path, Some("/tmp/test.txt".to_string()));
    }
    #[test]
    fn test_io_value() {
        let v = IoValue::pure_val(RtObject::nat(42));
        assert!(!v.is_error());
        assert_eq!(v.to_rt_object().as_small_nat(), Some(42));
        let e = IoValue::error(IoError::user_error("boom"));
        assert!(e.is_error());
    }
    #[test]
    fn test_io_runtime_sandbox() {
        let mut runtime = IoRuntime::sandboxed();
        assert!(!runtime.is_enabled());
        assert!(runtime.exec_println("hello").is_ok());
        let output = runtime.captured_output().expect("execution should succeed");
        assert_eq!(output, &["hello"]);
    }
    #[test]
    fn test_io_runtime_refs() {
        let mut runtime = IoRuntime::new();
        let id = runtime.new_ref(RtObject::nat(42));
        let value = runtime.read_ref(id).expect("execution should succeed");
        assert_eq!(value.as_small_nat(), Some(42));
        runtime
            .write_ref(id, RtObject::nat(100))
            .expect("execution should succeed");
        let value = runtime.read_ref(id).expect("execution should succeed");
        assert_eq!(value.as_small_nat(), Some(100));
    }
    #[test]
    fn test_io_runtime_modify_ref() {
        let mut runtime = IoRuntime::new();
        let id = runtime.new_ref(RtObject::nat(10));
        runtime
            .modify_ref(id, |v| {
                let n = v.as_small_nat().expect("type conversion should succeed");
                RtObject::nat(n * 2)
            })
            .expect("test operation should succeed");
        let value = runtime.read_ref(id).expect("execution should succeed");
        assert_eq!(value.as_small_nat(), Some(20));
    }
    #[test]
    fn test_io_runtime_invalid_ref() {
        let mut runtime = IoRuntime::new();
        assert!(runtime.read_ref(999).is_err());
        assert!(runtime.write_ref(999, RtObject::nat(0)).is_err());
    }
    #[test]
    fn test_io_runtime_input_mock() {
        let mut runtime = IoRuntime::new();
        runtime.push_input("hello".to_string());
        runtime.push_input("world".to_string());
        assert_eq!(
            runtime.exec_get_line().expect("execution should succeed"),
            "hello"
        );
        assert_eq!(
            runtime.exec_get_line().expect("execution should succeed"),
            "world"
        );
    }
    #[test]
    fn test_string_formatter() {
        assert_eq!(StringFormatter::nat_to_string(42), "42");
        assert_eq!(StringFormatter::bool_to_string(true), "true");
        assert_eq!(StringFormatter::pad_left("42", 5, '0'), "00042");
        assert_eq!(StringFormatter::pad_right("hi", 5, ' '), "hi   ");
        assert_eq!(StringFormatter::to_upper("hello"), "HELLO");
        assert_eq!(StringFormatter::to_lower("HELLO"), "hello");
        assert_eq!(StringFormatter::trim("  hello  "), "hello");
    }
    #[test]
    fn test_string_split_join() {
        let parts = StringFormatter::split("a,b,c", ",");
        assert_eq!(parts, vec!["a", "b", "c"]);
        let joined = StringFormatter::join(&parts, "-");
        assert_eq!(joined, "a-b-c");
    }
    #[test]
    fn test_string_operations() {
        assert!(StringFormatter::starts_with("hello", "hel"));
        assert!(StringFormatter::ends_with("hello", "llo"));
        assert!(StringFormatter::contains("hello world", "world"));
        assert_eq!(StringFormatter::replace("hello", "l", "r"), "herro");
    }
    #[test]
    fn test_error_handling() {
        let ok = ErrorHandling::ok(RtObject::nat(42));
        assert!(!ErrorHandling::is_error(&ok));
        let exc = ErrorHandling::make_exception("boom");
        let msg = ErrorHandling::get_message(&exc);
        assert_eq!(msg, Some("boom".to_string()));
    }
    #[test]
    fn test_io_executor_sandbox() {
        let mut runtime = IoRuntime::sandboxed();
        runtime.push_input("test input".to_string());
        {
            let mut exec = IoExecutor::new(&mut runtime);
            let result = exec.println("hello");
            assert!(!result.is_error());
            let result = exec.get_line();
            assert!(!result.is_error());
        }
        let output = runtime.captured_output().expect("execution should succeed");
        assert_eq!(output, &["hello"]);
    }
    #[test]
    fn test_io_executor_refs() {
        let mut runtime = IoRuntime::new();
        {
            let mut exec = IoExecutor::new(&mut runtime);
            let id_result = exec.new_ref(RtObject::nat(42));
            let id_obj = id_result.to_rt_object();
            let id = id_obj
                .as_small_nat()
                .expect("type conversion should succeed");
            let value = exec.read_ref(id);
            assert_eq!(value.to_rt_object().as_small_nat(), Some(42));
            let _write_result = exec.write_ref(id, RtObject::nat(100));
            let value2 = exec.read_ref(id);
            assert_eq!(value2.to_rt_object().as_small_nat(), Some(100));
        }
    }
    #[test]
    fn test_io_runtime_reset() {
        let mut runtime = IoRuntime::sandboxed();
        runtime.new_ref(RtObject::nat(1));
        runtime
            .exec_println("test")
            .expect("execution should succeed");
        runtime.reset();
        assert!(runtime
            .captured_output()
            .expect("execution should succeed")
            .is_empty());
        assert!(runtime.read_ref(0).is_err());
    }
    #[test]
    fn test_io_runtime_env_vars() {
        let mut runtime = IoRuntime::new();
        runtime.set_env("MY_VAR".to_string(), "hello".to_string());
        assert_eq!(runtime.get_env_var("MY_VAR"), Some("hello".to_string()));
        assert!(runtime.get_env_var("NONEXISTENT_VAR_12345").is_none());
    }
    #[test]
    fn test_io_stats() {
        let mut runtime = IoRuntime::sandboxed();
        runtime.exec_println("a").expect("execution should succeed");
        runtime.exec_println("b").expect("execution should succeed");
        runtime.new_ref(RtObject::unit());
        assert_eq!(runtime.stats().console_outputs, 2);
        assert_eq!(runtime.stats().refs_created, 1);
    }
}
#[cfg(test)]
mod tests_extended {
    use super::*;
    #[test]
    fn test_io_buffer_write_below_threshold() {
        let mut buf = IoBuffer::new(1024);
        let flushed = buf.write(b"hello");
        assert!(flushed.is_empty());
        assert_eq!(buf.buffered_bytes(), 5);
    }
    #[test]
    fn test_io_buffer_auto_flush() {
        let mut buf = IoBuffer::new(4);
        let flushed = buf.write(b"hello");
        assert_eq!(flushed, b"hello");
        assert_eq!(buf.buffered_bytes(), 0);
    }
    #[test]
    fn test_io_buffer_manual_flush() {
        let mut buf = IoBuffer::new(1024);
        buf.write_str("test");
        let flushed = buf.flush();
        assert_eq!(flushed, b"test");
        assert!(!buf.has_data());
    }
    #[test]
    fn test_io_buffer_stats() {
        let mut buf = IoBuffer::new(100);
        buf.write(b"hello");
        buf.write(b"world");
        assert_eq!(buf.write_count(), 2);
        assert_eq!(buf.total_bytes(), 10);
    }
    #[test]
    fn test_io_channel_write_read() {
        let mut ch = IoChannel::new();
        ch.write(b"hello world");
        let data = ch.read(5);
        assert_eq!(data, b"hello");
        let rest = ch.read_all();
        assert_eq!(rest, b" world");
    }
    #[test]
    fn test_io_channel_read_line() {
        let mut ch = IoChannel::new();
        ch.write(b"line1\nline2\n");
        let line = ch.read_line().expect("test operation should succeed");
        assert_eq!(line, "line1\n");
    }
    #[test]
    fn test_io_channel_closed() {
        let mut ch = IoChannel::new();
        ch.close();
        assert!(!ch.write(b"blocked"));
    }
    #[test]
    fn test_io_channel_available() {
        let mut ch = IoChannel::new();
        ch.write(b"abcde");
        assert_eq!(ch.available(), 5);
        ch.read(3);
        assert_eq!(ch.available(), 2);
    }
    #[test]
    fn test_io_channel_bytes_stats() {
        let mut ch = IoChannel::new();
        ch.write(b"hello");
        ch.read_all();
        assert_eq!(ch.bytes_written(), 5);
        assert_eq!(ch.bytes_read(), 5);
    }
    #[test]
    fn test_io_log_record_and_query() {
        let mut log = IoLog::new(100);
        log.record(IoEvent {
            kind: IoEventKind::Read,
            path: Some("/tmp/file".to_string()),
            bytes: 128,
            timestamp_ms: 1000,
            success: true,
        });
        log.record(IoEvent {
            kind: IoEventKind::Write,
            path: Some("/tmp/file".to_string()),
            bytes: 64,
            timestamp_ms: 2000,
            success: true,
        });
        assert_eq!(log.total_bytes_read(), 128);
        assert_eq!(log.total_bytes_written(), 64);
        assert_eq!(log.events_of_kind(&IoEventKind::Read).len(), 1);
    }
    #[test]
    fn test_io_log_overflow() {
        let mut log = IoLog::new(3);
        for i in 0..5u64 {
            log.record(IoEvent {
                kind: IoEventKind::Write,
                path: None,
                bytes: i as usize,
                timestamp_ms: i,
                success: true,
            });
        }
        assert!(log.has_overflowed());
        assert_eq!(log.len(), 3);
    }
    #[test]
    fn test_io_log_clear() {
        let mut log = IoLog::new(100);
        log.record(IoEvent {
            kind: IoEventKind::Close,
            path: None,
            bytes: 0,
            timestamp_ms: 0,
            success: true,
        });
        log.clear();
        assert!(log.is_empty());
        assert!(!log.has_overflowed());
    }
    #[test]
    fn test_io_throttle_allow() {
        let mut th = IoThrottle::new(1024, 1000);
        assert!(th.try_consume(512, 0));
        assert!(th.try_consume(512, 0));
        assert!(!th.try_consume(1, 0));
    }
    #[test]
    fn test_io_throttle_new_window() {
        let mut th = IoThrottle::new(100, 1000);
        th.try_consume(100, 0);
        assert!(th.try_consume(100, 2000));
    }
    #[test]
    fn test_io_throttle_stats() {
        let mut th = IoThrottle::new(100, 1000);
        th.try_consume(100, 0);
        th.try_consume(50, 0);
        assert_eq!(th.throttle_events(), 1);
        assert_eq!(th.total_throttled_bytes(), 50);
    }
    #[test]
    fn test_vfs_write_and_read() {
        let mut vfs = VirtualFilesystem::new();
        vfs.write_file("/test.txt", b"hello")
            .expect("test operation should succeed");
        let data = vfs
            .read_file("/test.txt")
            .expect("test operation should succeed");
        assert_eq!(data, b"hello");
    }
    #[test]
    fn test_vfs_file_not_found() {
        let vfs = VirtualFilesystem::new();
        let result = vfs.read_file("/nonexistent.txt");
        assert!(result.is_err());
    }
    #[test]
    fn test_vfs_delete() {
        let mut vfs = VirtualFilesystem::new();
        vfs.write_file("/del.txt", b"data")
            .expect("test operation should succeed");
        assert!(vfs.file_exists("/del.txt"));
        vfs.delete_file("/del.txt");
        assert!(!vfs.file_exists("/del.txt"));
    }
    #[test]
    fn test_vfs_append() {
        let mut vfs = VirtualFilesystem::new();
        vfs.write_file("/log.txt", b"line1\n")
            .expect("test operation should succeed");
        vfs.append_file("/log.txt", b"line2\n")
            .expect("test operation should succeed");
        let data = vfs
            .read_file("/log.txt")
            .expect("test operation should succeed");
        assert_eq!(data, b"line1\nline2\n");
    }
    #[test]
    fn test_vfs_copy_and_rename() {
        let mut vfs = VirtualFilesystem::new();
        vfs.write_file("/src.txt", b"content")
            .expect("test operation should succeed");
        vfs.copy_file("/src.txt", "/dst.txt")
            .expect("test operation should succeed");
        assert!(vfs.file_exists("/dst.txt"));
        vfs.rename_file("/dst.txt", "/new.txt")
            .expect("creation should succeed");
        assert!(vfs.file_exists("/new.txt"));
        assert!(!vfs.file_exists("/dst.txt"));
    }
    #[test]
    fn test_vfs_read_only() {
        let mut vfs = VirtualFilesystem::new();
        vfs.set_read_only(true);
        let result = vfs.write_file("/blocked.txt", b"data");
        assert!(result.is_err());
    }
    #[test]
    fn test_vfs_list_dir() {
        let mut vfs = VirtualFilesystem::new();
        vfs.write_file("/dir/a.txt", b"a")
            .expect("test operation should succeed");
        vfs.write_file("/dir/b.txt", b"b")
            .expect("test operation should succeed");
        vfs.write_file("/other/c.txt", b"c")
            .expect("test operation should succeed");
        let mut files = vfs.list_dir("/dir");
        files.sort();
        assert_eq!(files.len(), 2);
    }
    #[test]
    fn test_vfs_stats() {
        let mut vfs = VirtualFilesystem::new();
        vfs.write_file("/a.txt", b"hello")
            .expect("test operation should succeed");
        vfs.write_file("/b.txt", b"world!")
            .expect("test operation should succeed");
        assert_eq!(vfs.file_count(), 2);
        assert_eq!(vfs.total_bytes(), 11);
    }
    #[test]
    fn test_io_stats_basic() {
        let mut stats = IoSessionStats::default();
        stats.record_read(1024);
        stats.record_write(512);
        stats.record_flush();
        assert_eq!(stats.reads, 1);
        assert_eq!(stats.writes, 1);
        assert_eq!(stats.flushes, 1);
        assert_eq!(stats.bytes_read, 1024);
        assert_eq!(stats.bytes_written, 512);
    }
    #[test]
    fn test_io_stats_read_ratio() {
        let mut stats = IoSessionStats::default();
        stats.record_read(0);
        stats.record_read(0);
        stats.record_write(0);
        assert!((stats.read_ratio() - (2.0 / 3.0)).abs() < 1e-10);
    }
    #[test]
    fn test_io_stats_display() {
        let mut stats = IoSessionStats::default();
        stats.record_read(100);
        let s = format!("{}", stats);
        assert!(s.contains("reads: 1"));
    }
    #[test]
    fn test_io_mock_read() {
        let mut mock = IoMock::new(vec![MockIoOp::Read {
            expected: vec![],
            result: b"hello".to_vec(),
        }]);
        let mut buf = Vec::new();
        let result = mock.read(&mut buf).expect("test operation should succeed");
        assert_eq!(result, b"hello");
        assert!(mock.is_exhausted());
    }
    #[test]
    fn test_io_mock_write() {
        let mut mock = IoMock::new(vec![MockIoOp::Write {
            expected: b"test".to_vec(),
            ok: true,
        }]);
        assert!(mock.write(b"test"));
        assert!(mock.is_exhausted());
    }
    #[test]
    fn test_io_mock_calls_log() {
        let mut mock = IoMock::new(vec![
            MockIoOp::Read {
                expected: vec![],
                result: b"abc".to_vec(),
            },
            MockIoOp::Write {
                expected: b"xyz".to_vec(),
                ok: true,
            },
        ]);
        mock.read(&mut Vec::new());
        mock.write(b"xyz");
        assert_eq!(mock.calls().len(), 2);
    }
}
#[cfg(test)]
mod tests_extended2 {
    use super::*;
    #[test]
    fn test_file_watcher_basic() {
        let mut watcher = IoFileWatcher::new(100);
        watcher.watch("/tmp/test.txt", 0, 0);
        let mut sizes = HashMap::new();
        sizes.insert("/tmp/test.txt".to_string(), 500u64);
        let changed = watcher.poll(&sizes, 200);
        assert!(changed.contains(&"/tmp/test.txt".to_string()));
        let record = watcher
            .record("/tmp/test.txt")
            .expect("test operation should succeed");
        assert_eq!(record.change_count, 1);
        assert_eq!(record.size, 500);
    }
    #[test]
    fn test_file_watcher_no_change() {
        let mut watcher = IoFileWatcher::new(100);
        watcher.watch("/f", 100, 0);
        let mut sizes = HashMap::new();
        sizes.insert("/f".to_string(), 100u64);
        let changed = watcher.poll(&sizes, 200);
        assert!(changed.is_empty());
    }
    #[test]
    fn test_file_watcher_unwatch() {
        let mut watcher = IoFileWatcher::new(100);
        watcher.watch("/a", 0, 0);
        watcher.unwatch("/a");
        assert_eq!(watcher.watch_count(), 0);
    }
    #[test]
    fn test_pipe_pair_bidirectional() {
        let mut pipe = PipePair::new();
        pipe.send_a_to_b(b"hello from A");
        let received = pipe.recv_b(12);
        assert_eq!(received, b"hello from A");
        pipe.send_b_to_a(b"reply from B");
        let reply = pipe.recv_a(12);
        assert_eq!(reply, b"reply from B");
    }
    #[test]
    fn test_pipe_pair_close() {
        let mut pipe = PipePair::new();
        pipe.close();
        assert!(!pipe.send_a_to_b(b"blocked"));
    }
    #[test]
    fn test_io_policy_strict() {
        let policy = IoPolicy::strict();
        assert_eq!(policy.read_error, IoErrorPolicy::Propagate);
    }
    #[test]
    fn test_io_policy_lenient() {
        let policy = IoPolicy::lenient();
        assert_eq!(policy.read_error, IoErrorPolicy::LogAndContinue);
    }
    #[test]
    fn test_io_policy_retry() {
        let policy = IoPolicy::retry();
        assert_eq!(policy.write_error, IoErrorPolicy::Retry { max: 3 });
    }
}
#[cfg(test)]
mod tests_metrics {
    use super::*;
    #[test]
    fn test_io_metrics_record_and_bw() {
        let mut metrics = IoMetrics::new(1000, 64);
        metrics.record_read(500, 0);
        metrics.record_read(500, 500);
        let bw = metrics.read_bw(1000);
        assert!(bw > 0.0);
        assert_eq!(metrics.total_read(), 1000);
    }
    #[test]
    fn test_io_metrics_write_bw() {
        let mut metrics = IoMetrics::new(1000, 64);
        metrics.record_write(256, 0);
        assert_eq!(metrics.total_write(), 256);
    }
    #[test]
    fn test_io_metrics_windowed() {
        let mut metrics = IoMetrics::new(100, 64);
        metrics.record_read(1000, 0);
        let bw = metrics.read_bw(200);
        assert_eq!(bw, 0.0);
    }
}
