#[derive(Debug)]
pub enum ItRole {
    SDE1,
    SDE2,
    SDE3
}

#[derive(Debug)]
pub struct ItEmployee {
    name: String,
    id: i16,
    role: ItRole
}

impl ItEmployee {
    fn update_emp_details(
        &mut self,
        new_name: Option<String>,
        new_role: Option<ItRole>,
        new_id: Option<i16>
    ) {
        if let Some(name) = new_name {
            self.name = name
        }
        if let Some(role) = new_role {
            self.role = role
        }
        if let Some(id) = new_id {
            self.id = id
        }
    }
}


pub fn set_emp_details(name: String, role: ItRole, id: i16) -> ItEmployee {
    let e = ItEmployee { name: name, id: id, role: role };

    return e;
}

pub fn update_emp_details(
    emp: &mut ItEmployee,
    new_name: Option<String>,
    new_role: Option<ItRole>,
    new_id: Option<i16>
) {
    emp.update_emp_details(new_name, new_role, new_id);
}

