pub fn route_is_open(checks: &[bool]) -> bool {
    checks.iter().all(|check| *check)
}
