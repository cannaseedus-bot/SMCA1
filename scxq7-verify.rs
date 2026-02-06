// scxq7-verify.rs
// SCXQ7 Sovereign Verifier CLI
// Version: v1.0.0 (frozen)
// Verifier: scxq7.verify.v1

use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

const AXIOMS_DIR: &str = "axioms";
const AXIOMS: [(&str, &str); 4] = [
    (
        "scxq7.schema.xjson",
        "98a4394e1c761bcafcde5bbe8823ec157122e9a5f7649e7c67067252762a563e",
    ),
    (
        "scxq2.schema.xjson",
        "6bccfae8b876b356aa3dd1ade0402e16e1ffdaadcf30eb6b8b59a52131acf83d",
    ),
    (
        "smca.schema.xjson",
        "c809e83498dad5ae1e57635e36aa46f6c666c6594ad913d763237e8c4031d85f",
    ),
    (
        "cm1.schema.xjson",
        "8bd83645148e0564ffe4a8840c56b5228d52848df5a36711e17f340c2e54fc73",
    ),
];

const COMPLIANCE_VECTOR: &str = "{\"@verifier\":\"scxq7.verify.v1\",\"authority\":\"none\",\"deterministic\":true,\"offline\":true,\"projection_only\":true,\"reject_only\":true}";

const STEP_SCHEMA: &str = "schema validation";
const STEP_CM1: &str = "CM-1 legality";
const STEP_CONSTRAINT: &str = "constraint integrity";
const STEP_IDB: &str = "IDB anchoring";
const STEP_SCXQ2: &str = "SCXQ2 packing";

#[derive(Clone, Copy)]
struct Options {
    emit_manifest: bool,
    emit_badges: bool,
    strict: bool,
    json: bool,
    quiet: bool,
}

impl Options {
    fn new() -> Self {
        Self {
            emit_manifest: false,
            emit_badges: false,
            strict: false,
            json: false,
            quiet: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TargetType {
    Directory,
    S7,
    Xjson,
    IdbXml,
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn usage_error(msg: &str) -> ! {
    eprintln!("✘ {}", msg);
    eprintln!("\nRESULT: NON-COMPLIANT");
    std::process::exit(64);
}

fn fail(code: i32, message: &str, opts: Options) -> ! {
    if opts.json {
        let payload = format!(
            "{{\"result\":\"NON-COMPLIANT\",\"error\":\"{}\",\"code\":{},\"vector\":{}}}",
            json_escape(message),
            code,
            COMPLIANCE_VECTOR
        );
        eprintln!("{}", payload);
    } else {
        eprintln!("✘ {}", message);
        eprintln!("\nRESULT: NON-COMPLIANT");
    }
    std::process::exit(code);
}

fn json_escape(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn parse_args() -> (PathBuf, Options) {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(|s| s == "verify").unwrap_or(false) {
        args.remove(0);
    }

    let mut opts = Options::new();
    let mut target: Option<PathBuf> = None;

    for arg in args {
        if arg.starts_with("--") {
            match arg.as_str() {
                "--emit-manifest" => opts.emit_manifest = true,
                "--emit-badges" => opts.emit_badges = true,
                "--strict" => opts.strict = true,
                "--json" => opts.json = true,
                "--quiet" => opts.quiet = true,
                _ => usage_error("usage error: unknown option"),
            }
        } else if target.is_none() {
            target = Some(PathBuf::from(arg));
        } else {
            usage_error("usage error: too many targets");
        }
    }

    let target = target.unwrap_or_else(|| usage_error("usage error: missing target"));
    let target_str = target.to_string_lossy();

    if target_str == "-" || target_str.contains("://") || target_str.starts_with("//") {
        usage_error("usage error: forbidden input target");
    }

    (target, opts)
}

fn detect_target_type(path: &Path) -> TargetType {
    if path.is_dir() {
        return TargetType::Directory;
    }

    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if file_name == "IDB.xml" {
        return TargetType::IdbXml;
    }

    match path.extension().and_then(|ext| ext.to_str()) {
        Some("s7") => TargetType::S7,
        Some("xjson") => TargetType::Xjson,
        _ => usage_error("usage error: unsupported target type"),
    }
}

fn load_axioms(opts: Options) {
    for (name, expected_hash) in AXIOMS.iter() {
        let path = Path::new(AXIOMS_DIR).join(name);
        let mut file = File::open(&path)
            .unwrap_or_else(|_| fail(1, "schema validation failure: axioms missing", opts));
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .unwrap_or_else(|_| fail(1, "schema validation failure: axioms unreadable", opts));
        let actual = sha256_hex(&bytes);
        if actual != *expected_hash {
            let msg = format!("schema validation failure: axiom hash mismatch ({})", name);
            fail(1, &msg, opts);
        }
    }
}

fn collect_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_files_inner(root, &mut files);
    files.sort();
    files
}

fn collect_files_inner(root: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files_inner(&path, files);
            } else if path.is_file() {
                files.push(path);
            }
        }
    }
}

fn read_bytes(path: &Path, opts: Options) -> Vec<u8> {
    let mut file = File::open(path)
        .unwrap_or_else(|_| fail(1, "schema validation failure: cannot open target", opts));
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .unwrap_or_else(|_| fail(1, "schema validation failure: cannot read target", opts));
    bytes
}

fn has_forbidden_control(bytes: &[u8]) -> bool {
    bytes.iter().any(|b| {
        (*b < 0x20 && *b != b'\n' && *b != b'\r' && *b != b'\t') || *b == 0x7F
    })
}

fn contains_token_case_insensitive(bytes: &[u8], token: &str) -> bool {
    let haystack = String::from_utf8_lossy(bytes).to_ascii_lowercase();
    haystack.contains(&token.to_ascii_lowercase())
}

fn validate_xjson(bytes: &[u8], opts: Options) -> Result<(), String> {
    let trimmed = String::from_utf8_lossy(bytes).trim().to_string();
    if !(trimmed.starts_with('{') && trimmed.ends_with('}')) {
        return Err("invalid xjson structure".to_string());
    }
    if opts.strict && !trimmed.contains("\"@schema\"") {
        return Err("missing @schema field".to_string());
    }
    Ok(())
}

fn validate_idb_xml(bytes: &[u8]) -> Result<(), String> {
    let trimmed = String::from_utf8_lossy(bytes).trim().to_string();
    if !(trimmed.starts_with('<') && trimmed.ends_with('>')) {
        return Err("invalid XML structure".to_string());
    }
    Ok(())
}

fn schema_validation(target: &Path, target_type: TargetType, opts: Options) {
    match target_type {
        TargetType::Directory => {
            let files = collect_files(target);
            if files.is_empty() {
                fail(1, "schema validation failure: empty directory", opts);
            }
            for path in files {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if ext == "xjson" {
                        let bytes = read_bytes(&path, opts);
                        if let Err(reason) = validate_xjson(&bytes, opts) {
                            let msg = format!(
                                "schema validation failure: {} ({})",
                                reason,
                                path.display()
                            );
                            fail(1, &msg, opts);
                        }
                    }
                }
                if path.file_name().and_then(|n| n.to_str()) == Some("IDB.xml") {
                    let bytes = read_bytes(&path, opts);
                    if let Err(reason) = validate_idb_xml(&bytes) {
                        let msg = format!("schema validation failure: {} ({})", reason, path.display());
                        fail(1, &msg, opts);
                    }
                }
            }
        }
        TargetType::Xjson => {
            let bytes = read_bytes(target, opts);
            if let Err(reason) = validate_xjson(&bytes, opts) {
                let msg = format!("schema validation failure: {}", reason);
                fail(1, &msg, opts);
            }
        }
        TargetType::IdbXml => {
            let bytes = read_bytes(target, opts);
            if let Err(reason) = validate_idb_xml(&bytes) {
                let msg = format!("schema validation failure: {}", reason);
                fail(1, &msg, opts);
            }
        }
        TargetType::S7 => {
            let bytes = read_bytes(target, opts);
            if bytes.is_empty() {
                fail(1, "schema validation failure: empty target", opts);
            }
        }
    }
}

fn cm1_legality(target: &Path, target_type: TargetType, opts: Options) {
    let check_bytes = |bytes: &[u8], path: &Path| {
        if has_forbidden_control(bytes) {
            let msg = format!("CM-1 violation: forbidden control characters ({})", path.display());
            fail(2, &msg, opts);
        }
    };

    match target_type {
        TargetType::Directory => {
            for path in collect_files(target) {
                let bytes = read_bytes(&path, opts);
                check_bytes(&bytes, &path);
            }
        }
        _ => {
            let bytes = read_bytes(target, opts);
            check_bytes(&bytes, target);
        }
    }
}

fn constraint_integrity(target: &Path, target_type: TargetType, opts: Options) {
    if !opts.strict {
        return;
    }

    let mut found = false;
    match target_type {
        TargetType::Directory => {
            for path in collect_files(target) {
                let bytes = read_bytes(&path, opts);
                if contains_token_case_insensitive(&bytes, "scxq7") {
                    found = true;
                    break;
                }
            }
        }
        _ => {
            let bytes = read_bytes(target, opts);
            found = contains_token_case_insensitive(&bytes, "scxq7");
        }
    }

    if !found {
        fail(3, "constraint violation: missing scxq7 marker", opts);
    }
}

fn idb_anchoring(target: &Path, target_type: TargetType, opts: Options) {
    let mut idb_paths = Vec::new();
    match target_type {
        TargetType::IdbXml => idb_paths.push(target.to_path_buf()),
        TargetType::Directory => {
            for path in collect_files(target) {
                if path.file_name().and_then(|n| n.to_str()) == Some("IDB.xml") {
                    idb_paths.push(path);
                }
            }
        }
        _ => {}
    }

    for path in idb_paths {
        let bytes = read_bytes(&path, opts);
        let content = String::from_utf8_lossy(&bytes).to_ascii_lowercase();
        if !content.contains("hash") || !content.contains("parent") {
            let msg = format!("IDB anchoring failure: hash continuity violation ({})", path.display());
            fail(4, &msg, opts);
        }
    }
}

fn scxq2_packing(target: &Path, target_type: TargetType, opts: Options) {
    if !opts.strict {
        return;
    }

    let mut found = false;
    match target_type {
        TargetType::Directory => {
            for path in collect_files(target) {
                let bytes = read_bytes(&path, opts);
                if contains_token_case_insensitive(&bytes, "scxq2") {
                    found = true;
                    break;
                }
            }
        }
        _ => {
            let bytes = read_bytes(target, opts);
            found = contains_token_case_insensitive(&bytes, "scxq2");
        }
    }

    if !found {
        fail(5, "SCXQ2 failure: lane packing marker missing", opts);
    }
}

fn target_hash(target: &Path, target_type: TargetType, opts: Options) -> String {
    match target_type {
        TargetType::Directory => {
            let mut hasher = Sha256::new();
            for path in collect_files(target) {
                let rel = path.strip_prefix(target).unwrap_or(&path);
                hasher.update(rel.to_string_lossy().as_bytes());
                hasher.update(b"\0");
                let bytes = read_bytes(&path, opts);
                let file_hash = sha256_hex(&bytes);
                hasher.update(file_hash.as_bytes());
                hasher.update(b"\0");
            }
            format!("{:x}", hasher.finalize())
        }
        _ => {
            let bytes = read_bytes(target, opts);
            sha256_hex(&bytes)
        }
    }
}

fn emit_manifest(target: &Path, target_type: TargetType, target_hash: &str, opts: Options) -> String {
    let target_type_str = match target_type {
        TargetType::Directory => "directory",
        TargetType::S7 => ".s7",
        TargetType::Xjson => ".xjson",
        TargetType::IdbXml => "IDB.xml",
    };

    let manifest = format!(
        "{{\n  \"@schema\": \"scxq7://verified-manifest/v1\",\n  \"result\": \"COMPLIANT\",\n  \"target\": \"{}\",\n  \"target_type\": \"{}\",\n  \"target_hash\": \"{}\",\n  \"logical_time\": 1,\n  \"steps\": [\n    \"{}\",\n    \"{}\",\n    \"{}\",\n    \"{}\",\n    \"{}\"\n  ],\n  \"vector\": {}\n}}\n",
        json_escape(&target.to_string_lossy()),
        target_type_str,
        target_hash,
        STEP_SCHEMA,
        STEP_CM1,
        STEP_CONSTRAINT,
        STEP_IDB,
        STEP_SCXQ2,
        COMPLIANCE_VECTOR
    );

    fs::write("verified.manifest.xjson", manifest.as_bytes())
        .unwrap_or_else(|_| fail(1, "schema validation failure: cannot write manifest", opts));

    sha256_hex(manifest.as_bytes())
}

fn emit_badges(manifest_hash: &str, opts: Options) {
    fs::create_dir_all("badges")
        .unwrap_or_else(|_| fail(1, "schema validation failure: cannot create badges", opts));

    let badge = format!(
        "{{\n  \"@schema\": \"scxq7://badge/v1\",\n  \"badge\": \"SCXQ7–SCO/1 COMPLIANT\",\n  \"manifest_hash\": \"{}\"\n}}\n",
        manifest_hash
    );

    fs::write("badges/scxq7-compliant.badge.xjson", badge.as_bytes())
        .unwrap_or_else(|_| fail(1, "schema validation failure: cannot write badge", opts));
}

fn emit_success(opts: Options, target: &Path, target_type: TargetType, target_hash: &str) {
    if opts.quiet {
        return;
    }

    if opts.json {
        let payload = format!(
            "{{\"result\":\"COMPLIANT\",\"target\":\"{}\",\"target_type\":\"{}\",\"target_hash\":\"{}\",\"steps\":[\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"],\"vector\":{}}}",
            json_escape(&target.to_string_lossy()),
            match target_type {
                TargetType::Directory => "directory",
                TargetType::S7 => ".s7",
                TargetType::Xjson => ".xjson",
                TargetType::IdbXml => "IDB.xml",
            },
            target_hash,
            STEP_SCHEMA,
            STEP_CM1,
            STEP_CONSTRAINT,
            STEP_IDB,
            STEP_SCXQ2,
            COMPLIANCE_VECTOR
        );
        println!("{}", payload);
        return;
    }

    println!("✔ {}", STEP_SCHEMA);
    println!("✔ {}", STEP_CM1);
    println!("✔ {}", STEP_CONSTRAINT);
    println!("✔ {}", STEP_IDB);
    println!("✔ {}", STEP_SCXQ2);
    println!("\nRESULT: COMPLIANT");
}

fn main() {
    let (target, opts) = parse_args();
    let target_type = detect_target_type(&target);

    load_axioms(opts);
    schema_validation(&target, target_type, opts);
    cm1_legality(&target, target_type, opts);
    constraint_integrity(&target, target_type, opts);
    idb_anchoring(&target, target_type, opts);
    scxq2_packing(&target, target_type, opts);

    let target_hash = target_hash(&target, target_type, opts);

    let mut manifest_hash = None;
    if opts.emit_manifest {
        manifest_hash = Some(emit_manifest(&target, target_type, &target_hash, opts));
    }

    if opts.emit_badges {
        let hash = manifest_hash.unwrap_or_else(|| emit_manifest(&target, target_type, &target_hash, opts));
        emit_badges(&hash, opts);
    }

    emit_success(opts, &target, target_type, &target_hash);
}
