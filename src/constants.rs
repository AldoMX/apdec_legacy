// exFAT >= 32Gb uses 128k for cluster size
// https://support.microsoft.com/en-us/topic/default-cluster-size-for-ntfs-fat-and-exfat-9772e6f1-e31a-00d7-e18f-73169155af95
pub const BUFFER_SIZE: usize = 0x20000;

pub const U32_SIZE: usize = std::mem::size_of::<u32>();
