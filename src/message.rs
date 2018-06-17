use serde_derive::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(rustfmt, rustfmt::skip)]
pub enum OxyMessage {
    ProtocolVersionQuery { },
    ProtocolVersionAnnounce { version: u64 },
    Ping { },
    Pong { },
    DummyMessage { data: Vec<u8> },
    BasicCommand { command: String },
    BasicCommandOutput { stdout: Vec<u8>, stderr: Vec<u8> },
    PipeCommand { command: String },
    PipeCommandOutput { reference: u64, stdout: Vec<u8>, stderr: Vec<u8> },
    PipeCommandInput { reference: u64, input: Vec<u8> },
    PipeCommandExited { reference: u64 },
    Reject { reference: u64, note: String },
    Success { reference: u64 },
    PtyRequest { command: String },
    PtySizeAdvertisement { w: u16, h: u16 },
    PtyInput { data: Vec<u8> },
    PtyOutput { data: Vec<u8> },
    PtyExited { status: i32 },
    DownloadRequest { path: String, offset_start: Option<u64>, offset_end: Option<u64> },
    UploadRequest { path: String, filepart: String, offset_start: Option<u64> },
    FileData { reference: u64, data: Vec<u8> },
    RemoteOpen { addr: String },
    RemoteBind { addr: String },
    CloseRemoteBind { reference: u64 },
    RemoteStreamData { reference: u64, data: Vec<u8> },
    LocalStreamData { reference: u64, data: Vec<u8> },
    RemoteStreamClosed { reference: u64 },
    LocalStreamClosed { reference: u64 },
    BindConnectionAccepted { reference: u64 },
    TunnelRequest { tap: bool, name: String },
    TunnelData { reference: u64, data: Vec<u8> },
    StatRequest { path: String },
    StatResult { reference: u64, len: u64, is_dir: bool, is_file: bool, owner: String, group: String, octal_permissions: u16, atime: Option<SystemTime>, mtime: Option<SystemTime>, ctime: Option<SystemTime> },
    ReadDir { path: String },
    ReadDirResult { reference: u64, complete: bool, answers: Vec<String> },
    FileHashRequest { path: String, offset_start: Option<u64>, offset_end: Option<u64>, hash_algorithm: u64 },
    FileHashData { reference: u64, digest: Vec<u8> },
    FileTruncateRequest { path: String, len: u64 },
    KnockForward { destination: String, knock: Vec<u8> },
    AdvertiseXAuth { cookie: String },
    UsernameAdvertisement { username: String },
}
