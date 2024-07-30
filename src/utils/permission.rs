use crate::actions::auth::ADAuth;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ad_auth = ADAuth::new(
        "ldap://your_domain_controller:389",
        "CN=BindUser,OU=ServiceAccounts,DC=yourdomain,DC=com",
        "bind_password",
        "DC=yourdomain,DC=com"
    )?;

    let username = "user1";
    let password = "user1_password";

    if ad_auth.authenticate(username, password)? {
        println!("Authentication successful");

        if ad_auth.check_group_membership(username, "AdminGroup")? {
            println!("User is a member of AdminGroup");
            // Proceed with admin operations
        } else {
            println!("User is not a member of AdminGroup");
            // Proceed with regular user operations
        }
    } else {
        println!("Authentication failed");
    }

    Ok(())
}