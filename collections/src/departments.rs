pub mod abm{
    use std::collections::HashMap;
    fn get_or_add_dept(
        dept: &mut String,
        company: &mut HashMap<String, Vec<String>>) -> String{
        let empty_dept: Vec<String>  = Vec::new();
        company.entry(dept.to_string()).or_insert(empty_dept);
        return dept.to_string()
    }

    pub fn add_person_to_dept(
        dept: String, person: String,
        company: &mut HashMap<String, Vec<String>>) {
            let department = get_or_add_dept(
                &mut dept.to_string(), company).to_string();
            if let Some(dept) = company.get_mut(&department) {
                dept.push(person)
            }

    }
    pub fn get_all(company: &mut HashMap<String, Vec<String>>) {
        let mut departments: Vec<String> = Vec::new();
        for dept in company.keys() {
            departments.push(dept.to_string());
        }
        departments.sort();
        for department in &departments{
            get_by_dept(department.to_string(), company);
        }

    }

    pub fn get_by_dept(dept: String, company: &mut HashMap<String, Vec<String>>) {
        println!("Department: {}", dept);
        if let Some(dept) = company.get_mut(&dept) {
            dept.sort()
        }
        match company.get(&dept) {
            Some(x) => println!("{:#?}", x),
            None => println!("[]")
        }

    }
}
