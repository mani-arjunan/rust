
#[derive(Debug)]
pub struct User {
    name: String,
    id: i16,
}

impl User {
    fn update_user_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    fn update_user_id(&mut self, new_id: i16) {
        self.id = new_id
    }
}


pub fn set_user_details(user_name: &str, user_id: i16) -> User {
    let u = User {
        name: user_name.to_string(),
        id: user_id,
    };

    return u;
}

pub fn update_user_details(u: &mut User, user_name: Option<&str>, user_id: Option<i16>) {
    match user_name {
        Some(name) => u.update_user_name(name.to_string()),
        None => {}
    }

    if let Some(id) = user_id {
        u.update_user_id(id);
    }
}

