// This file is part of uptime. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/uptime/master/COPYRIGHT. No part of uptime, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of uptime. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/uptime/master/COPYRIGHT.


extern crate libc;
extern crate timeval;
#[cfg(target_os = "windows")] extern crate kernel32;
extern crate sysctl;
use std::io::Error;
use std::io::Result;
use std::ptr;
use self::timeval::difference_in_microseconds;
use self::timeval::UnsignedMicrosecond;
use std::mem::uninitialized;

// For interest, this GitHub comment shows how to polyfill gettimeofday on Windows: https://stackoverflow.com/questions/10905892/equivalent-of-gettimeday-for-windows

#[cfg(not(any(target_os = "linux", target_os = "android", target_os = "windows", target_os = "solaris")))]
pub fn uptime() -> Result<UnsignedMicrosecond>
{
	let boot = try!(self::sysctl::boot_time());
	
	unsafe
	{
		let mut now: self::libc::timeval = uninitialized();
		
		match self::libc::gettimeofday(&mut now as *mut self::libc::timeval, ptr::null_mut())
		{
			0 => Ok(difference_in_microseconds(now, boot) as UnsignedMicrosecond),
			-1 => Err(Error::last_os_error()),
			unexpected @ _ => panic!("Unexpected result from gettimeofday {}", unexpected),
		}
	}
}

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn uptime() -> Result<UnsignedMicrosecond>
{
	unsafe
	{
		let mut sysinfo: self::libc::sysinfo = uninitialized();
		match self::libc::sysinfo(&mut sysinfo as *mut sysinfo)
		{
			0 => Ok(sysinfo.uptime as UnsignedMicrosecond),
			-1 => Err(Error::last_os_error()),
			unexpected @ _ => panic!("Unexpected result from sysinfo {}", unexpected),
		}
	}
}

#[cfg(target_os = "windows")]
pub fn uptime() -> Result<UnsignedMicrosecond>
{
	((self::kernel32::GetTickCount() as UnsignedMillisecond) * 1_000) as UnsignedMicrosecond
}

// ... just leaves Solaris ...
