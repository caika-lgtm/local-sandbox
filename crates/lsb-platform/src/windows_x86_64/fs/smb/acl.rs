use std::path::PathBuf;

use super::types::{WindowsSmbAccess, WindowsSmbLifecycleError, WindowsSmbLifecyclePhase};
use super::user::WindowsSmbUserAccount;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowsSmbAclGrantRequest {
    pub path: PathBuf,
    pub account: WindowsSmbUserAccount,
    pub access: WindowsSmbAccess,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowsSmbAclGrant {
    pub path: PathBuf,
    pub principal: String,
    pub access: WindowsSmbAccess,
}

pub trait WindowsSmbAclManager {
    fn grant_access(
        &mut self,
        request: WindowsSmbAclGrantRequest,
    ) -> Result<WindowsSmbAclGrant, WindowsSmbLifecycleError>;

    fn revoke_access(&mut self, grant: &WindowsSmbAclGrant)
        -> Result<(), WindowsSmbLifecycleError>;
}

#[cfg(windows)]
#[derive(Default)]
pub struct NativeWindowsSmbAclManager;

#[cfg(windows)]
impl WindowsSmbAclManager for NativeWindowsSmbAclManager {
    fn grant_access(
        &mut self,
        request: WindowsSmbAclGrantRequest,
    ) -> Result<WindowsSmbAclGrant, WindowsSmbLifecycleError> {
        apply_acl_change(
            &request.path,
            &request.account.principal,
            request.access,
            windows_sys::Win32::Security::Authorization::GRANT_ACCESS,
            WindowsSmbLifecyclePhase::AclGrant,
        )?;
        Ok(WindowsSmbAclGrant {
            path: request.path,
            principal: request.account.principal,
            access: request.access,
        })
    }

    fn revoke_access(
        &mut self,
        grant: &WindowsSmbAclGrant,
    ) -> Result<(), WindowsSmbLifecycleError> {
        apply_acl_change(
            &grant.path,
            &grant.principal,
            grant.access,
            windows_sys::Win32::Security::Authorization::REVOKE_ACCESS,
            WindowsSmbLifecyclePhase::AclRevoke,
        )
    }
}

#[cfg(windows)]
fn apply_acl_change(
    path: &std::path::Path,
    principal: &str,
    access: WindowsSmbAccess,
    mode: windows_sys::Win32::Security::Authorization::ACCESS_MODE,
    phase: WindowsSmbLifecyclePhase,
) -> Result<(), WindowsSmbLifecycleError> {
    use std::ptr;

    use windows_sys::Win32::Foundation::LocalFree;
    use windows_sys::Win32::Security::Authorization::{
        GetNamedSecurityInfoW, SetEntriesInAclW, SetNamedSecurityInfoW, EXPLICIT_ACCESS_W,
        NO_MULTIPLE_TRUSTEE, SE_FILE_OBJECT, TRUSTEE_IS_NAME, TRUSTEE_IS_USER, TRUSTEE_W,
    };
    use windows_sys::Win32::Security::{
        DACL_SECURITY_INFORMATION, SUB_CONTAINERS_AND_OBJECTS_INHERIT,
    };

    let mut path_w = super::user::wide_null(&path.display().to_string());
    let mut principal_w = super::user::wide_null(principal);
    let mut old_dacl = ptr::null_mut();
    let mut security_descriptor = ptr::null_mut();
    let status = unsafe {
        GetNamedSecurityInfoW(
            path_w.as_mut_ptr(),
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut old_dacl,
            ptr::null_mut(),
            &mut security_descriptor,
        )
    };
    if status != 0 {
        return Err(WindowsSmbLifecycleError::operation_failed(
            phase,
            format!("GetNamedSecurityInfoW failed with win32 error {status}"),
        ));
    }

    let trustee = TRUSTEE_W {
        pMultipleTrustee: ptr::null_mut(),
        MultipleTrusteeOperation: NO_MULTIPLE_TRUSTEE,
        TrusteeForm: TRUSTEE_IS_NAME,
        TrusteeType: TRUSTEE_IS_USER,
        ptstrName: principal_w.as_mut_ptr(),
    };
    let mut entry = EXPLICIT_ACCESS_W {
        grfAccessPermissions: ntfs_access_mask(access),
        grfAccessMode: mode,
        grfInheritance: SUB_CONTAINERS_AND_OBJECTS_INHERIT,
        Trustee: trustee,
    };
    let mut new_acl = ptr::null_mut();
    let status = unsafe { SetEntriesInAclW(1, &mut entry, old_dacl, &mut new_acl) };
    if status != 0 {
        unsafe {
            LocalFree(security_descriptor);
        }
        return Err(WindowsSmbLifecycleError::operation_failed(
            phase,
            format!("SetEntriesInAclW failed with win32 error {status}"),
        ));
    }

    let status = unsafe {
        SetNamedSecurityInfoW(
            path_w.as_mut_ptr(),
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION,
            ptr::null_mut(),
            ptr::null_mut(),
            new_acl,
            ptr::null_mut(),
        )
    };
    unsafe {
        LocalFree(new_acl.cast());
        LocalFree(security_descriptor);
    }
    if status != 0 {
        return Err(WindowsSmbLifecycleError::operation_failed(
            phase,
            format!("SetNamedSecurityInfoW failed with win32 error {status}"),
        ));
    }
    Ok(())
}

#[cfg(windows)]
fn ntfs_access_mask(access: WindowsSmbAccess) -> u32 {
    use windows_sys::Win32::Storage::FileSystem::{
        DELETE, FILE_DELETE_CHILD, FILE_GENERIC_EXECUTE, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
    };

    match access {
        WindowsSmbAccess::ReadOnly => FILE_GENERIC_READ | FILE_GENERIC_EXECUTE,
        WindowsSmbAccess::ReadWrite => {
            FILE_GENERIC_READ
                | FILE_GENERIC_EXECUTE
                | FILE_GENERIC_WRITE
                | FILE_DELETE_CHILD
                | DELETE
        }
    }
}
