pub struct Section<'a> {
    pub flag: &'a str,
    pub append_message: &'a str,
    pub exclude_message: &'a str,
    pub error_message: &'a str,
}

pub const OVERVIEW: Section = Section {
    flag: "overview-exclude",
    append_message: "[Info] Overview section appended",
    exclude_message: "[Info] Overview section excluded",
    error_message: "[Error] Only one --exclude-overview allowed",
};

pub const EXAMPLE_USE: Section = Section {
    flag: "example-use-exclude",
    append_message: "[Info] Example use section appended",
    exclude_message: "[Info] Example use section excluded",
    error_message: "[Error] Only one --exclude-example-use allowed",
};

pub const LICENSE: Section = Section {
    flag: "license-exclude",
    append_message: "[Info] Licence section appended",
    exclude_message: "[Info] License section excluded",
    error_message: "[Error] Only one --exclude-licence allowed",
};

pub const DOCUMENTATION: Section = Section {
    flag: "documentation-exclude",
    append_message: "[Info] Documentation section appended",
    exclude_message: "[Info] Documentation section excluded",
    error_message: "[Error] Only one --exclude-documentation allowed",
};

pub const CHANGELOG: Section = Section {
    flag: "changelog-exclude",
    append_message: "[Info] Changelog section appended",
    exclude_message: "[Info] Changelog section excluded",
    error_message: "[Error] Only one --exclude-changelog allowed",
};

pub const DEVELOPMENT_VERSION: Section = Section {
    flag: "development-version-exclude",
    append_message: "[Info] Development version section appended",
    exclude_message: "[Info] Development version section excluded",
    error_message: "[Error] Only one --exclude-development-version allowed",
};

pub const STABLE_VERSION: Section = Section {
    flag: "stable-version-exclude",
    append_message: "[Info] Stable version section appended",
    exclude_message: "[Info] Stable version section excluded",
    error_message: "[Error] Only one --exclude-stable-version allowed",
};

pub const CONTRIBUTING: Section = Section {
    flag: "contributing-exclude",
    append_message: "[Info] Contributing section appended",
    exclude_message: "[Info] Contributing section excluded",
    error_message: "[Error] Only one --exclude-contributing allowed",
};
