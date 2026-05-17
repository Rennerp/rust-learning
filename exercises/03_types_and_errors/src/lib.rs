#![allow(unused_variables)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Admin,
    Member,
    Guest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserError {
    EmptyEmail,
    MissingAtSign,
}

impl User {
    pub fn new(id: u64, email: String, role: Role) -> Result<Self, UserError> {
        // Validate first; construct User only when the input is valid.
        if email.is_empty() {
            return Err(UserError::EmptyEmail);
        }

        if !email.contains("@") {
            return Err(UserError::MissingAtSign);
        }

        Ok(User { id, email, role })
    }

    pub fn can_delete_users(&self) -> bool {
        // A method with &self reads the user without taking ownership.
        match self.role {
            Role::Admin => true,
            _ => false
        }
    }
}

pub fn parse_role(input: &str) -> Option<Role> {
    // Return None when the input does not map to a known role.
    if input.eq_ignore_ascii_case("admin") {
        Some(Role::Admin)
    } else if input.eq_ignore_ascii_case("member") {
        Some(Role::Member)
    } else if input.eq_ignore_ascii_case("guest") {
        Some(Role::Guest)
    } else {
        None
    }
}

pub fn find_user(users: &[User], id: u64) -> Option<&User> {
    // Return a borrowed user from the input slice.
    // todo!("find a user by id")
    users.iter().find(|user| user.id == id)
}

pub fn admin_emails(users: &[User]) -> Vec<String> {
    // Return owned emails, so cloning selected String values is expected.
   users.iter()
        .filter(|user| user.role == Role::Admin)
        .map(|user| user.email.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(id: u64, email: &str, role: Role) -> User {
        User {
            id,
            email: email.to_string(),
            role,
        }
    }

    #[test]
    fn validates_user_email() {
        assert_eq!(
            User::new(1, String::new(), Role::Member),
            Err(UserError::EmptyEmail)
        );
        assert_eq!(
            User::new(1, String::from("missing-at-sign"), Role::Member),
            Err(UserError::MissingAtSign)
        );

        let created = User::new(1, String::from("user@example.com"), Role::Member).unwrap();
        assert_eq!(created.id, 1);
    }

    #[test]
    fn checks_permissions() {
        assert!(user(1, "admin@example.com", Role::Admin).can_delete_users());
        assert!(!user(2, "member@example.com", Role::Member).can_delete_users());
        assert!(!user(3, "guest@example.com", Role::Guest).can_delete_users());
    }

    #[test]
    fn parses_roles() {
        assert_eq!(parse_role("admin"), Some(Role::Admin));
        assert_eq!(parse_role("MEMBER"), Some(Role::Member));
        assert_eq!(parse_role("Guest"), Some(Role::Guest));
        assert_eq!(parse_role("owner"), None);
    }

    #[test]
    fn finds_users() {
        let users = vec![
            user(1, "one@example.com", Role::Member),
            user(2, "two@example.com", Role::Admin),
        ];

        assert_eq!(find_user(&users, 2).unwrap().email, "two@example.com");
        assert_eq!(find_user(&users, 99), None);
    }

    #[test]
    fn lists_admin_emails() {
        let users = vec![
            user(1, "one@example.com", Role::Member),
            user(2, "two@example.com", Role::Admin),
            user(3, "three@example.com", Role::Admin),
        ];

        assert_eq!(
            admin_emails(&users),
            vec![
                String::from("two@example.com"),
                String::from("three@example.com")
            ]
        );
    }
}
