use ldap3::{LdapConn, Scope, SearchEntry};
use std::error::Error;

pub struct ADAuth {
    conn: LdapConn,
    base_dn: String,
}

impl ADAuth {
    pub fn new(ldap_url: &str, bind_dn: &str, bind_pw: &str, base_dn: &str) -> Result<Self, Box<dyn Error>> {
        let mut conn = LdapConn::new(ldap_url)?;
        conn.simple_bind(bind_dn, bind_pw)?;
        Ok(ADAuth {
            conn,
            base_dn: base_dn.to_string(),
        })
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
        let user_dn = self.get_user_dn(username)?;
        match self.conn.simple_bind(&user_dn, password) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn check_group_membership(&mut self, username: &str, group: &str) -> Result<bool, Box<dyn Error>> {
        let user_dn = self.get_user_dn(username)?;
        let (rs, _res) = self.conn.search(
            &self.base_dn,
            Scope::Subtree,
            &format!("(&(objectClass=group)(cn={})(member={}))", group, user_dn),
            vec!["cn"],
        )?.success()?;

        // Extract entries from the result set
        let entries: Vec<_> = rs.into_iter().map(SearchEntry::construct).collect();

        Ok(!entries.is_empty())
    }

    fn get_user_dn(&mut self, username: &str) -> Result<String, Box<dyn Error>> {
        let (rs, _res) = self.conn.search(
            &self.base_dn,
            Scope::Subtree,
            &format!("(&(objectClass=user)(sAMAccountName={}))", username),
            vec!["distinguishedName"],
        )?.success()?;

        // Extract entries from the result set
        let entries: Vec<_> = rs.into_iter().map(SearchEntry::construct).collect();

        if let Some(entry) = entries.first() {
            Ok(entry.dn.clone())
        } else {
            Err("User not found".into())
        }
    }
}