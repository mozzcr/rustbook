// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Job {
    Maintenance,
    Marketer,
    Manager,
    Supervisor,
    Kitchen,
    Assembly,
}

struct Employee {
    job: Job,
    employed: bool,
}

fn access(employee: Employee) -> Result<(), ()> {
    if employee.employed == false {
        return Err(println!("Access denied"));
    }

    match employee.job {
        Job::Maintenance => Ok(println!("Access granted")),
        Job::Marketer => Ok(println!("Access granted")),
        Job::Manager => Ok(println!("Access granted")),
        Job::Supervisor => Err(println!("Access denied")),
        Job::Kitchen => Err(println!("Access denied")),
        Job::Assembly => Err(println!("Access denied")),
    }
}

fn main() {
    let employee1 = Employee {
        job: Job::Marketer,
        employed: false,
    };

    let result = access(employee1);
    println!("{:?}", result)
}
