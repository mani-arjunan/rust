#[derive(Debug)]
pub enum CivilRole {
    MODELER,
    DETAILER,
    CHECKER
}

#[derive(Debug)]
pub struct CivilEmployee {
    name: String,
    id: i16,
    role: CivilRole
}

impl CivilEmployee {
    fn update_emp_details(
        &mut self,
        new_name: Option<String>,
        new_role: Option<CivilRole>,
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


pub fn set_emp_details(name: String, role: CivilRole, id: i16) -> CivilEmployee {
    let e = CivilEmployee { name: name, id: id, role: role };

    return e;
}

pub fn update_emp_details(
    emp: &mut CivilEmployee,
    new_name: Option<String>,
    new_role: Option<CivilRole>,
    new_id: Option<i16>
) {
    emp.update_emp_details(new_name, new_role, new_id);
}

