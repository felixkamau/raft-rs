/// Different node state trasitions
#[derive(Debug)]
#[allow(unused)]
pub enum Role {
    LEADER,
    FOLLOWER,
    CANDIDATE,
}

impl TryFrom<&str> for Role {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "leader" => Ok(Role::LEADER),
            "follower" => Ok(Role::FOLLOWER),
            "candidate" => Ok(Role::CANDIDATE),
            _ => Err("Unknown State".to_string()),
        }
    }
}
