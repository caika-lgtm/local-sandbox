mod acl;
mod admin;
mod lifecycle;
mod password;
mod share;
mod types;
mod user;

#[cfg(windows)]
pub use acl::NativeWindowsSmbAclManager;
pub use acl::{WindowsSmbAclGrant, WindowsSmbAclGrantRequest, WindowsSmbAclManager};
#[cfg(windows)]
pub use admin::NativeWindowsSmbAdmin;
pub use admin::WindowsSmbAdmin;
pub use lifecycle::{WindowsSmbActiveResources, WindowsSmbLifecycleManager};
pub use password::{
    NativeWindowsSmbPasswordGenerator, WindowsSmbPassword, WindowsSmbPasswordGenerator,
};
#[cfg(windows)]
pub use share::NativeWindowsSmbShareManager;
pub use share::{
    WindowsSmbShare, WindowsSmbShareCreateRequest, WindowsSmbShareManager, WindowsSmbShareName,
};
pub use types::{
    generate_smb_share_name, generate_smb_user_name, validate_smb_share_name,
    validate_smb_user_name, WindowsSmbAccess, WindowsSmbCleanupFailure, WindowsSmbLifecycleConfig,
    WindowsSmbLifecycleError, WindowsSmbLifecyclePhase, WindowsSmbMount,
    WINDOWS_SMB_GATEWAY_SERVER,
};
#[cfg(windows)]
pub use user::NativeWindowsSmbUserManager;
pub use user::{WindowsSmbUserAccount, WindowsSmbUserManager, WindowsSmbUserName};
