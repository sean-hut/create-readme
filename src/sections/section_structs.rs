pub struct Section<'a> {
    pub flag: &'a str,
    pub append_message: &'a str,
    pub exclude_message: &'a str,
    pub error_message: &'a str,
}

pub const OVERVIEW: Section = Section {
    flag: "overview-exclude",
    append_message: "Overview section appended",
    exclude_message: "Overview section excluded",
    error_message: "Only one --exclude-overview allowed",
};

pub const EXAMPLE_USE: Section = Section {
    flag: "example-use-exclude",
    append_message: "Example use section appended",
    exclude_message: "Example use section excluded",
    error_message: "Only one --exclude-example-use allowed",
};

pub const LICENSE: Section = Section {
    flag: "license-exclude",
    append_message: "Licence section appended",
    exclude_message: "License section excluded",
    error_message: "Only one --exclude-licence allowed",
};

pub const DOCUMENTATION: Section = Section {
    flag: "documentation-exclude",
    append_message: "Documentation section appended",
    exclude_message: "Documentation section excluded",
    error_message: "Only one --exclude-documentation allowed",
};

pub const CHANGELOG: Section = Section {
    flag: "changelog-exclude",
    append_message: "Changelog section appended",
    exclude_message: "Changelog section excluded",
    error_message: "Only one --exclude-changelog allowed",
};

pub const DEVELOPMENT_VERSION: Section = Section {
    flag: "development-version-exclude",
    append_message: "Development version section appended",
    exclude_message: "Development version section excluded",
    error_message: "Only one --exclude-development-version allowed",
};

pub const STABLE_VERSION: Section = Section {
    flag: "stable-version-exclude",
    append_message: "Stable version section appended",
    exclude_message: "Stable version section excluded",
    error_message: "Only one --exclude-stable-version allowed",
};

pub const CONTRIBUTING: Section = Section {
    flag: "contributing-exclude",
    append_message: "Contributing section appended",
    exclude_message: "Contributing section excluded",
    error_message: "Only one --exclude-contributing allowed",
};
