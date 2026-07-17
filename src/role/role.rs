/// Different node state trasitions
#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(unused)]
#[derive(Default)]
pub enum Role {
    #[default]
    FOLLOWER,
    CANDIDATE,
    LEADER,
}

impl Role {
    pub fn current_role(&self) -> Role {
        *self
    }
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

#[cfg(test)]
mod test {
    use crate::role::role::Role;

    #[test]
    fn it_defaults() {
        let role: Role = Role::default();
        assert_eq!(role, Role::FOLLOWER);
    }
}
