use std::{
    fs::{FileType, Metadata},
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
};

pub fn parse_generic_command(command: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    let command = parts.get(0).map_or("", |&cmd| cmd).to_string();
    let args = parts.into_iter().skip(1).map(String::from).collect();
    (command, args)
}

pub fn identify_file_type(metadata: &Metadata, file: &FileType) -> String {
    let mut symbolic_indicator = String::new();
    match file {
        ft if ft.is_dir() => symbolic_indicator.push('/'),
        ft if ft.is_file() && metadata.mode() & 0o111 != 0 => symbolic_indicator.push('*'),
        ft if ft.is_symlink() => symbolic_indicator.push('@'),
        ft if ft.is_socket() => symbolic_indicator.push('='),
        ft if ft.is_fifo() => symbolic_indicator.push('|'),
        _ => {}
    };
    return symbolic_indicator;
}

pub fn format_permissions(metadata: &Metadata) -> String {
    let mode = metadata.permissions().mode();
    let mut permissions = String::with_capacity(11);

    // File type
    if mode & 0o170000 == 0o040000 {
        permissions.push('d'); // Directory
    } else {
        permissions.push('-'); // Regular file
    }

    // Owner permissions
    let owner_perms = [(0o400, 'r'), (0o200, 'w'), (0o100, 'x')];
    for &(mask, ch) in &owner_perms {
        if mode & mask != 0 {
            permissions.push(ch);
        } else {
            permissions.push('-');
        }
    }

    // Group permissions
    let group_perms = [(0o40, 'r'), (0o20, 'w'), (0o10, 'x')];
    for &(mask, ch) in &group_perms {
        if mode & mask != 0 {
            permissions.push(ch);
        } else {
            permissions.push('-');
        }
    }

    // Other permissions
    let other_perms = [(0o4, 'r'), (0o2, 'w'), (0o1, 'x')];
    for &(mask, ch) in &other_perms {
        if mode & mask != 0 {
            permissions.push(ch);
        } else {
            permissions.push('-');
        }
    }
    permissions
}
